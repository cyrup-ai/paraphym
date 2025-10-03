//! Schema introspection module for upstream GraphQL servers
//!
//! This module provides schema introspection capabilities for SweetMCP Pingora
//! to extract real schema information from upstream GraphQL servers and cache
//! it for fragment resolution.


use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::types::{GraphQLTypeInfo, GraphQLTypeKind};

/// GraphQL introspection query to get full schema information
const INTROSPECTION_QUERY: &str = r#"
query IntrospectionQuery {
  __schema {
    queryType { name }
    mutationType { name }
    subscriptionType { name }
    types {
      ...FullType
    }
  }
}

fragment FullType on __Type {
  kind
  name
  description
  fields(includeDeprecated: true) {
    name
    description
    args {
      ...InputValue
    }
    type {
      ...TypeRef
    }
    isDeprecated
    deprecationReason
  }
  inputFields {
    ...InputValue
  }
  interfaces {
    ...TypeRef
  }
  enumValues(includeDeprecated: true) {
    name
    description
    isDeprecated
    deprecationReason
  }
  possibleTypes {
    ...TypeRef
  }
}

fragment InputValue on __InputValue {
  name
  description
  type { ...TypeRef }
  defaultValue
}

fragment TypeRef on __Type {
  kind
  name
  ofType {
    kind
    name
    ofType {
      kind
      name
      ofType {
        kind
        name
        ofType {
          kind
          name
          ofType {
            kind
            name
            ofType {
              kind
              name
              ofType {
                kind
                name
              }
            }
          }
        }
      }
    }
  }
}
"#;

/// Introspection response structures
#[derive(Debug, Deserialize)]
struct IntrospectionResponse {
    data: IntrospectionData,
}

#[derive(Debug, Deserialize)]
struct IntrospectionData {
    #[serde(rename = "__schema")]
    schema: SchemaData,
}

#[derive(Debug, Deserialize)]
struct SchemaData {
    #[serde(rename = "queryType")]
    query_type: Option<TypeRef>,
    #[serde(rename = "mutationType")]
    mutation_type: Option<TypeRef>,
    #[serde(rename = "subscriptionType")]
    subscription_type: Option<TypeRef>,
    types: Vec<TypeData>,
}

#[derive(Debug, Deserialize)]
pub struct TypeData {
    pub kind: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<FieldData>>,
    pub interfaces: Option<Vec<TypeRef>>,
    #[serde(rename = "possibleTypes")]
    pub possible_types: Option<Vec<TypeRef>>,
    #[serde(rename = "enumValues")]
    pub enum_values: Option<Vec<EnumValueData>>,
    #[serde(rename = "inputFields")]
    pub input_fields: Option<Vec<InputValueData>>,
}

#[derive(Debug, Deserialize)]
pub struct FieldData {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub field_type: TypeRef,
    pub args: Vec<InputValueData>,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRef {
    pub kind: String,
    pub name: Option<String>,
    #[serde(rename = "ofType")]
    pub of_type: Option<Box<TypeRef>>,
}

#[derive(Debug, Deserialize)]
pub struct EnumValueData {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputValueData {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub input_type: TypeRef,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
}

/// Cached schema information for an upstream server
#[derive(Debug, Clone)]
pub struct CachedSchema {
    pub types: HashMap<String, GraphQLTypeInfo>,
    pub cached_at: Instant,
    pub ttl: Duration,
}

impl CachedSchema {
    /// Check if the cached schema is still valid
    pub fn is_valid(&self) -> bool {
        self.cached_at.elapsed() < self.ttl
    }
}

/// Schema introspection manager for upstream GraphQL servers
pub struct SchemaIntrospector {
    client: Client,
    cache: Arc<RwLock<HashMap<String, CachedSchema>>>,
    default_ttl: Duration,
}

impl SchemaIntrospector {
    /// Create a new schema introspector
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client for schema introspection")?;

        Ok(Self {
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Duration::from_secs(3600), // 1 hour default TTL
        })
    }

    /// Create a new schema introspector with custom settings
    pub fn with_settings(timeout: Duration, cache_ttl: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .context("Failed to create HTTP client for schema introspection")?;

        Ok(Self {
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: cache_ttl,
        })
    }

    /// Get schema information for an upstream GraphQL server
    pub async fn get_schema(&self, upstream_url: &str) -> Result<HashMap<String, GraphQLTypeInfo>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(upstream_url)
                && cached.is_valid() {
                    debug!("Using cached schema for upstream: {}", upstream_url);
                    return Ok(cached.types.clone());
                }
        }

        // Cache miss or expired, perform introspection
        info!(
            "Performing schema introspection for upstream: {}",
            upstream_url
        );
        let schema_types = self.introspect_schema(upstream_url).await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(
                upstream_url.to_string(),
                CachedSchema {
                    types: schema_types.clone(),
                    cached_at: Instant::now(),
                    ttl: self.default_ttl,
                },
            );
        }

        Ok(schema_types)
    }

    /// Perform schema introspection against an upstream GraphQL server
    async fn introspect_schema(
        &self,
        upstream_url: &str,
    ) -> Result<HashMap<String, GraphQLTypeInfo>> {
        let request_body = json!({
            "query": INTROSPECTION_QUERY,
            "variables": {}
        });

        let response = self
            .client
            .post(upstream_url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send introspection query")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Introspection query failed with status: {}",
                response.status()
            ));
        }

        let introspection_response: IntrospectionResponse = response
            .json()
            .await
            .context("Failed to parse introspection response")?;

        self.parse_introspection_response(introspection_response)
    }

    /// Parse introspection response into GraphQLTypeInfo structures
    fn parse_introspection_response(
        &self,
        response: IntrospectionResponse,
    ) -> Result<HashMap<String, GraphQLTypeInfo>> {
        let mut schema_types = HashMap::new();

        for type_data in response.data.schema.types {
            if let Some(type_name) = &type_data.name {
                // Skip introspection types
                if type_name.starts_with("__") {
                    continue;
                }

                let type_info = self.convert_type_data_to_info(&type_data)?;
                schema_types.insert(type_name.clone(), type_info);
            }
        }

        info!(
            "Parsed {} types from introspection response",
            schema_types.len()
        );
        Ok(schema_types)
    }

    /// Convert TypeData to GraphQLTypeInfo
    pub fn convert_type_data_to_info(&self, type_data: &TypeData) -> Result<GraphQLTypeInfo> {
        let name = type_data
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Type data missing name"))?
            .clone();

        let kind = match type_data.kind.as_str() {
            "OBJECT" => GraphQLTypeKind::Object,
            "INTERFACE" => GraphQLTypeKind::Interface,
            "UNION" => GraphQLTypeKind::Union,
            "ENUM" => GraphQLTypeKind::Enum,
            "SCALAR" => GraphQLTypeKind::Scalar,
            "INPUT_OBJECT" => GraphQLTypeKind::InputObject,
            _ => {
                warn!("Unknown GraphQL type kind: {}", type_data.kind);
                GraphQLTypeKind::Object // Default fallback
            }
        };

        let fields = type_data
            .fields
            .as_ref()
            .map(|fields| fields.iter().map(|f| f.name.clone()).collect())
            .unwrap_or_default();

        let interfaces = type_data
            .interfaces
            .as_ref()
            .map(|interfaces| interfaces.iter().filter_map(|i| i.name.clone()).collect())
            .unwrap_or_default();

        let possible_types = type_data
            .possible_types
            .as_ref()
            .map(|possible_types| {
                possible_types
                    .iter()
                    .filter_map(|pt| pt.name.clone())
                    .collect()
            })
            .unwrap_or_default();

        Ok(GraphQLTypeInfo {
            name,
            kind,
            fields,
            interfaces,
            possible_types,
        })
    }

    /// Clear expired entries from the cache
    pub async fn cleanup_cache(&self) {
        let mut cache = self.cache.write().await;
        let initial_size = cache.len();

        cache.retain(|_, cached_schema| cached_schema.is_valid());

        let final_size = cache.len();
        if initial_size > final_size {
            debug!(
                "Cleaned up {} expired schema cache entries",
                initial_size - final_size
            );
        }
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        let total = cache.len();
        let valid = cache.values().filter(|c| c.is_valid()).count();
        (total, valid)
    }
}

impl Default for SchemaIntrospector {
    fn default() -> Self {
        // Use a basic configuration that should never fail
        Self {
            client: Client::new(), // Use default client without custom timeout
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Duration::from_secs(3600),
        }
    }
}

