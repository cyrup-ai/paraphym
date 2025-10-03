# Authentication: Missing Authorization Features

## Status
**INCOMPLETE** - Core security implemented but missing stated requirements

## QA Review Rating: 7/10

### Rating Justification

**Strengths (Production Quality):**
- ✅ JWT expiry validation fully implemented (validation.rs:430-450)
- ✅ HTTPS enforcement properly configured (proxy_impl.rs:171-188)
- ✅ Token encryption/decryption actively used (AES-256-GCM with PBKDF2)
- ✅ Password validation with weak pattern detection (key_encryption.rs:51)
- ✅ Secure token infrastructure in production (api/peers.rs:7-22)
- ✅ Role-based access control for /admin paths (proxy_impl.rs:414-431)
- ✅ Permission checks for DELETE method (proxy_impl.rs:433-451)
- ✅ Audit logging for authenticated requests

**Critical Gaps (Stated in DoD but Missing):**
1. ❌ PUT method permission check MISSING - DoD claims "[x] Permission checks for (DELETE, PUT)" but proxy_impl.rs:433 ONLY checks DELETE
2. ❌ Auth attempt rate limiting MISSING - Security improvements claim "Rate Limiting enforced per IP" but no implementation exists

**Scope Limitations:**
- RBAC only protects /admin/* paths (limited coverage)
- PBAC only protects DELETE method (no PATCH/POST checks)

## Outstanding Items

### 1. Add PUT Method Permission Check
**Location**: `packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs:433-451`

**Current Code** (lines 433-451):
```rust
// Permission-based access control
if method == pingora::http::Method::DELETE 
   && !auth_context.has_permission("delete") {
    warn!("Delete permission denied for user: {:?}", auth_context.user_id());
    _ctx.status_code = 403;
    // ... metrics and error response ...
}
```

**Required Fix** - Add PUT method check:
```rust
// Permission-based access control for DELETE
if method == pingora::http::Method::DELETE 
   && !auth_context.has_permission("delete") {
    warn!("Delete permission denied for user: {:?}", auth_context.user_id());
    _ctx.status_code = 403;
    // ... metrics and error response ...
    session.respond_error(403).await?;
    return Ok(true);
}

// Permission-based access control for PUT
if method == pingora::http::Method::PUT 
   && !auth_context.has_permission("write") {
    warn!("Write permission denied for user: {:?}", auth_context.user_id());
    _ctx.status_code = 403;
    // ... metrics and error response ...
    session.respond_error(403).await?;
    return Ok(true);
}
```

### 2. Implement Auth Attempt Rate Limiting
**Location**: `packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs` (before authentication)

**Config Exists But Unused**: `auth.max_auth_attempts_per_minute` in AuthConfig

**Required Implementation**:
```rust
// After HTTPS validation, before authentication
// Track auth attempts per client IP
let client_ip = AuthHandler::extract_client_ip(session)
    .unwrap_or_else(|| "unknown".to_string());

// Check rate limit (requires rate limiter implementation)
if auth_attempts_exceeded(&client_ip, self.cfg.auth.max_auth_attempts_per_minute) {
    warn!("Max auth attempts exceeded for IP: {}", client_ip);
    _ctx.status_code = 429;
    session.respond_error(429).await?;
    return Ok(true);
}
```

**Implementation Steps**:
1. Add `DashMap<String, (u32, Instant)>` to EdgeService for tracking auth attempts per IP
2. Create `auth_attempts_exceeded()` method that:
   - Checks current attempt count for IP
   - Resets counter if >1 minute elapsed
   - Increments counter and checks against `max_auth_attempts_per_minute`
3. Record failed auth attempts in authenticate_request()

## Optional Enhancements

### Expand Authorization Scope
- Add permission checks for PATCH and POST methods
- Support path-based role mappings beyond /admin
- Implement fine-grained resource-level permissions

### Use Certificate Authority Infrastructure
- AuthorityBuilder, AuthorityFilesystemBuilder, AuthorityKeychainBuilder are defined but unused
- Consider using for advanced TLS certificate management scenarios

## Files Modified Since Original Assessment

The original task file incorrectly identified the following as missing (all are actually implemented):
- ~~JWT expiry validation~~ - IMPLEMENTED at validation.rs:430-450
- ~~Authorization checks~~ - PARTIALLY IMPLEMENTED (needs PUT check completion)
- ~~HTTPS enforcement~~ - IMPLEMENTED at proxy_impl.rs:171-188
- ~~Token encryption~~ - IMPLEMENTED and ACTIVELY USED
- ~~Password validation~~ - IMPLEMENTED via validate_encryption_passphrase()
- ~~Secure token infrastructure~~ - IMPLEMENTED in api/peers.rs

## Definition of Done (Corrected)

### Core Security ✅
- [x] JWT expiry timestamp validated in parse_jwt_claims()
- [x] JWT issued_at checked to prevent future-dated tokens
- [x] HTTPS requirement enforced when require_https: true
- [x] Token encryption/decryption used for sensitive data
- [x] Password validation enforces strong policies

### Authorization (Incomplete) ⚠️
- [x] Role checks implemented for /admin endpoints
- [x] Permission checks implemented for DELETE operations
- [ ] **MISSING: Permission checks for PUT operations**
- [x] 403 Forbidden returned when role/permission missing
- [x] Username logged in audit trail

### Security Configuration (Incomplete) ⚠️
- [x] HTTPS requirement enforced from config
- [x] Token expiry duration respected from config
- [ ] **MISSING: Auth attempt rate limiting per IP**

## Implementation Priority

**HIGH PRIORITY (Complete DoD):**
1. Add PUT method permission check (15 min)
2. Implement auth attempt rate limiting (30 min)

**MEDIUM PRIORITY (Expand scope):**
3. Add PATCH/POST permission checks
4. Implement configurable path-to-role mappings

**LOW PRIORITY (Infrastructure):**
5. Use CA builders for advanced cert management
