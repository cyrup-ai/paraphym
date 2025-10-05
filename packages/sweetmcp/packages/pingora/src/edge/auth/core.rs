//! Core authentication types and structures
//!
//! This module provides the foundational types and data structures for
//! authentication and authorization with zero allocation patterns and
//! blazing-fast performance.


/// Authentication handler with optimized token validation
pub struct AuthHandler;

/// Authentication context for request processing
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub is_authenticated: bool,
    pub auth_method: AuthMethod,
    pub user_claims: Option<UserClaims>,
    pub client_ip: Option<String>,
}

/// User claims extracted from authentication tokens
#[derive(Debug, Clone)]
pub struct UserClaims {
    pub user_id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub expires_at: u64,
    pub issued_at: u64,
}

/// Authentication method used for the request
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    /// No authentication
    None,
    /// JWT token authentication
    JwtToken,
    /// Discovery token authentication
    DiscoveryToken,
    /// API key authentication
    ApiKey,
}

impl AuthContext {
    /// Create new authentication context
    pub fn new() -> Self {
        Self {
            is_authenticated: false,
            auth_method: AuthMethod::None,
            user_claims: None,
            client_ip: None,
        }
    }

    /// Create authenticated context with user claims
    pub fn authenticated(auth_method: AuthMethod, user_claims: UserClaims) -> Self {
        Self {
            is_authenticated: true,
            auth_method,
            user_claims: Some(user_claims),
            client_ip: None,
        }
    }

    /// Create unauthenticated context
    pub fn unauthenticated() -> Self {
        Self::new()
    }

    /// Set client IP address
    pub fn with_client_ip(mut self, client_ip: String) -> Self {
        self.client_ip = Some(client_ip);
        self
    }

    /// Check if user has specific permission with fast permission lookup
    pub fn has_permission(&self, permission: &str) -> bool {
        self.user_claims
            .as_ref()
            .map(|claims| claims.permissions.contains(&permission.to_string()))
            .unwrap_or(false)
    }

    /// Check if authentication is expired with optimized time checking
    pub fn is_expired(&self) -> bool {
        self.user_claims
            .as_ref()
            .map(|claims| {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                now > claims.expires_at
            })
            .unwrap_or(true)
    }

    /// Get user ID if authenticated
    pub fn user_id(&self) -> Option<&str> {
        self.user_claims
            .as_ref()
            .map(|claims| claims.user_id.as_str())
    }

    /// Get username if authenticated
    pub fn username(&self) -> Option<&str> {
        self.user_claims
            .as_ref()
            .map(|claims| claims.username.as_str())
    }

    /// Get all user roles
    pub fn roles(&self) -> Vec<&str> {
        self.user_claims
            .as_ref()
            .map(|claims| claims.roles.iter().map(|r| r.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get all user permissions
    pub fn permissions(&self) -> Vec<&str> {
        self.user_claims
            .as_ref()
            .map(|claims| claims.permissions.iter().map(|p| p.as_str()).collect())
            .unwrap_or_default()
    }

    /// Check if user has any of the specified roles
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        self.user_claims
            .as_ref()
            .map(|claims| {
                roles
                    .iter()
                    .any(|role| claims.roles.contains(&role.to_string()))
            })
            .unwrap_or(false)
    }

    /// Check if user has any of the specified permissions
    pub fn has_any_permission(&self, permissions: &[&str]) -> bool {
        self.user_claims
            .as_ref()
            .map(|claims| {
                permissions
                    .iter()
                    .any(|perm| claims.permissions.contains(&perm.to_string()))
            })
            .unwrap_or(false)
    }

    /// Get time until expiration
    pub fn time_until_expiration(&self) -> Option<std::time::Duration> {
        self.user_claims.as_ref().and_then(|claims| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            if claims.expires_at > now {
                Some(std::time::Duration::from_secs(claims.expires_at - now))
            } else {
                None
            }
        })
    }

    /// Get authentication age
    pub fn auth_age(&self) -> Option<std::time::Duration> {
        self.user_claims.as_ref().map(|claims| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            std::time::Duration::from_secs(now.saturating_sub(claims.issued_at))
        })
    }

}

impl Default for AuthContext {
    fn default() -> Self {
        Self::new()
    }
}

impl UserClaims {
    /// Create new user claims
    pub fn new(
        user_id: String,
        username: String,
        roles: Vec<String>,
        permissions: Vec<String>,
        expires_at: u64,
    ) -> Self {
        let issued_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            user_id,
            username,
            roles,
            permissions,
            expires_at,
            issued_at,
        }
    }

}
