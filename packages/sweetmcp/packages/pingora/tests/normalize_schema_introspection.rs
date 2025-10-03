use sweetmcp::normalize::schema_introspection::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_schema_introspector_creation() {
    let introspector = SchemaIntrospector::new().expect("Failed to create introspector");
    let (total, valid) = introspector.cache_stats().await;
    assert_eq!(total, 0);
    assert_eq!(valid, 0);
}

#[tokio::test]
async fn test_cached_schema_validity() {
    let schema = CachedSchema {
        types: HashMap::new(),
        cached_at: Instant::now(),
        ttl: Duration::from_secs(1),
    };

    assert!(schema.is_valid());

    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert!(!schema.is_valid());
}

#[test]
fn test_type_kind_conversion() {
    use sweetmcp::normalize::schema_introspection::{TypeData, FieldData, TypeRef};
    use sweetmcp::normalize::types::GraphQLTypeKind;
    
    let introspector = SchemaIntrospector::new().expect("Failed to create introspector");

    let type_data = TypeData {
        kind: "OBJECT".to_string(),
        name: Some("User".to_string()),
        description: None,
        fields: Some(vec![FieldData {
            name: "id".to_string(),
            description: None,
            field_type: TypeRef {
                kind: "SCALAR".to_string(),
                name: Some("ID".to_string()),
                of_type: None,
            },
            args: vec![],
            is_deprecated: false,
            deprecation_reason: None,
        }]),
        interfaces: None,
        possible_types: None,
        enum_values: None,
        input_fields: None,
    };

    let type_info = introspector.convert_type_data_to_info(&type_data).unwrap();
    assert_eq!(type_info.name, "User");
    assert_eq!(type_info.kind, GraphQLTypeKind::Object);
    assert_eq!(type_info.fields, vec!["id"]);
}
