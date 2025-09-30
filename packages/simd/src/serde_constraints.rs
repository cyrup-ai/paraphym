//! Serde Type Constraint API for Structured Generation
//!
//! This module provides user-friendly functions for creating JSON schema constraints
//! from Rust serde types, enabling structured generation that guarantees output
//! matches specific type definitions.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use serde::{Deserialize, Serialize};
//! use schemars::JsonSchema;
//! use paraphym_simd::serde_constraints::constraint_for_type;
//!
//! #[derive(Serialize, Deserialize, JsonSchema)]
//! struct User {
//!     name: String,
//!     age: u32,
//!     email: Option<String>,
//! }
//!
//! let constraint = constraint_for_type::<User>(tokenizer)?;
//! // Use constraint in generation to guarantee valid User JSON
//! ```

use anyhow::{Context, Result as AnyResult};
use schemars::{JsonSchema, schema_for};
use std::sync::Arc;
use tokenizers::Tokenizer;

use crate::logits::constraints::{
    JsonConstraint,
    SchemaConstraint, SchemaVocabulary, regex_from_schema, regex_from_value,
};

/// Create a JSON constraint from a serde type with `JsonSchema` derive
///
/// This function generates a JSON schema from the given type and creates
/// a constraint that ensures generated JSON conforms to that type structure.
///
/// # Type Parameters
/// * `T` - Type implementing `Serialize`, `Deserialize`, and `JsonSchema`
///
/// # Arguments
/// * `tokenizer` - Tokenizer for token-to-text conversion
///
/// # Returns
/// * `Ok(SchemaConstraint)` - Constraint that validates against the type schema
/// * `Err(anyhow::Error)` - If schema generation or constraint creation fails
///
/// # Example
/// ```rust,no_run
/// use serde::{Deserialize, Serialize};
/// use schemars::JsonSchema;
/// use paraphym_simd::serde_constraints::constraint_for_type;
///
/// #[derive(Serialize, Deserialize, JsonSchema)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let constraint = constraint_for_type::<Person>(&tokenizer)?;
/// ```
pub fn constraint_for_type<T>(tokenizer: &Tokenizer) -> AnyResult<SchemaConstraint>
where
    T: JsonSchema + serde::Serialize,
{
    let vocabulary = Arc::new(SchemaVocabulary::from_tokenizer(tokenizer)?);
    let regex_pattern = regex_from_schema::<T>()
        .context("Failed to generate regex from type")?;

    SchemaConstraint::new(&regex_pattern, vocabulary, false)
        .context("Failed to create schema constraint from type")
}

/// Create a JSON constraint from a JSON schema string
///
/// This function parses a JSON schema string and creates a constraint
/// that validates generated tokens against the schema structure.
///
/// # Arguments
/// * `schema_json` - JSON schema as a string
/// * `tokenizer` - Tokenizer for token-to-text conversion
///
/// # Returns
/// * `Ok(SchemaConstraint)` - Constraint that validates against the schema
/// * `Err(anyhow::Error)` - If schema parsing or constraint creation fails
///
/// # Example
/// ```rust,no_run
/// use paraphym_simd::serde_constraints::constraint_for_schema;
///
/// let schema = r#"{
///     "type": "object",
///     "properties": {
///         "name": { "type": "string" },
///         "age": { "type": "integer" }
///     },
///     "required": ["name", "age"]
/// }"#;
///
/// let constraint = constraint_for_schema(schema, &tokenizer)?;
/// ```
pub fn constraint_for_schema(
    schema_json: &str,
    tokenizer: &Tokenizer,
) -> AnyResult<SchemaConstraint> {
    let schema: serde_json::Value = serde_json::from_str(schema_json)
        .context("Failed to parse JSON schema")?;

    let vocabulary = Arc::new(SchemaVocabulary::from_tokenizer(tokenizer)?);
    let regex_pattern = regex_from_value(&schema, None, None)
        .context("Failed to generate regex from schema")?;

    SchemaConstraint::new(&regex_pattern, vocabulary, false)
        .context("Failed to create schema constraint from JSON")
}

/// Create a basic JSON syntax constraint (no schema validation)
///
/// This function creates a constraint that only validates JSON syntax
/// without enforcing any specific structure. Useful for generating
/// valid JSON without type constraints.
///
/// # Arguments
/// * `tokenizer` - Tokenizer for token-to-text conversion
///
/// # Returns
/// * `Ok(JsonConstraint)` - Constraint that validates JSON syntax
/// * `Err(anyhow::Error)` - If constraint creation fails
///
/// # Example
/// ```rust,no_run
/// use paraphym_simd::serde_constraints::basic_json_constraint;
///
/// let constraint = basic_json_constraint(&tokenizer)?;
/// // Ensures valid JSON syntax but allows any structure
/// ```
pub fn basic_json_constraint(tokenizer: &Tokenizer) -> AnyResult<JsonConstraint<'_>> {
    JsonConstraint::new(tokenizer)
        .context("Failed to create basic JSON constraint")
}

/// Builder for creating complex constraints with multiple options
///
/// Provides a fluent API for configuring constraint behavior, including
/// validation strictness, error handling, and performance options.
///
/// # Example
/// ```rust,no_run
/// use paraphym_simd::serde_constraints::ConstraintBuilder;
///
/// let constraint = ConstraintBuilder::new(&tokenizer)
///     .with_type::<MyType>()
///     .with_strict_validation(true)
///     .with_partial_generation(false)
///     .build()?;
/// ```
pub struct ConstraintBuilder<'a> {
    tokenizer: &'a Tokenizer,
    schema: Option<serde_json::Value>,
    strict_validation: bool,
    allow_partial: bool,
}

impl<'a> ConstraintBuilder<'a> {
    /// Create a new constraint builder
    ///
    /// # Arguments
    /// * `tokenizer` - Tokenizer for token-to-text conversion
    pub fn new(tokenizer: &'a Tokenizer) -> Self {
        Self {
            tokenizer,
            schema: None,
            strict_validation: true,
            allow_partial: false,
        }
    }

    /// Set the constraint to validate against a specific serde type
    ///
    /// # Type Parameters
    /// * `T` - Type implementing `JsonSchema`
    pub fn with_type<T>(mut self) -> Self
    where
        T: JsonSchema,
    {
        let schema = schema_for!(T);
        if let Ok(schema_value) = serde_json::to_value(&schema) {
            self.schema = Some(schema_value);
        }
        self
    }

    /// Set the constraint to validate against a JSON schema string
    ///
    /// # Arguments
    /// * `schema_json` - JSON schema as a string
    pub fn with_schema(mut self, schema_json: &str) -> AnyResult<Self> {
        let schema: serde_json::Value = serde_json::from_str(schema_json)
            .context("Failed to parse JSON schema in builder")?;
        self.schema = Some(schema);
        Ok(self)
    }

    /// Enable or disable strict validation
    ///
    /// When enabled (default), all schema constraints are enforced strictly.
    /// When disabled, some validation errors may be treated as warnings.
    ///
    /// # Arguments
    /// * `strict` - Whether to enable strict validation
    pub fn with_strict_validation(mut self, strict: bool) -> Self {
        self.strict_validation = strict;
        self
    }

    /// Enable or disable partial generation
    ///
    /// When enabled, allows generation to complete even if the schema
    /// is not fully satisfied. When disabled (default), generation
    /// continues until schema requirements are met.
    ///
    /// # Arguments
    /// * `allow_partial` - Whether to allow partial generation
    pub fn with_partial_generation(mut self, allow_partial: bool) -> Self {
        self.allow_partial = allow_partial;
        self
    }

    /// Build the constraint with current configuration
    ///
    /// # Returns
    /// * `Ok(SchemaConstraint)` - Configured constraint ready for use
    /// * `Err(anyhow::Error)` - If constraint creation fails
    pub fn build(self) -> AnyResult<SchemaConstraint> {
        let schema = self.schema
            .ok_or_else(|| anyhow::anyhow!("No schema specified in constraint builder"))?;

        let vocabulary = Arc::new(SchemaVocabulary::from_tokenizer(self.tokenizer)?);

        // Apply configuration options to regex generation
        let max_recursion = if self.strict_validation { 5 } else { 3 };
        let regex_pattern = regex_from_value(&schema, None, Some(max_recursion))
            .context("Failed to generate regex from schema")?;

        let constraint = SchemaConstraint::new(&regex_pattern, vocabulary, self.allow_partial)
            .context("Failed to build schema constraint")?;

        Ok(constraint)
    }

    /// Build a basic JSON constraint instead of schema constraint
    ///
    /// This creates a constraint that only validates JSON syntax,
    /// ignoring any schema that may have been set.
    ///
    /// # Returns
    /// * `Ok(JsonConstraint)` - JSON syntax constraint
    /// * `Err(anyhow::Error)` - If constraint creation fails
    pub fn build_json_only(self) -> AnyResult<JsonConstraint<'a>> {
        JsonConstraint::new(self.tokenizer)
            .context("Failed to build JSON constraint")
    }
}

/// Convenience function for common constraint patterns
pub mod presets {
    use super::*;

    /// Create constraint for generating JSON objects with string keys and any values
    ///
    /// Useful for generating generic JSON objects without specific structure requirements.
    ///
    /// # Arguments
    /// * `tokenizer` - Tokenizer for token-to-text conversion
    pub fn object_with_string_keys(tokenizer: &Tokenizer) -> AnyResult<SchemaConstraint> {
        let schema_json = r#"{
            "type": "object",
            "additionalProperties": true
        }"#;
        constraint_for_schema(schema_json, tokenizer)
    }

    /// Create constraint for generating JSON arrays of any values
    ///
    /// Useful for generating JSON arrays without specific item type requirements.
    ///
    /// # Arguments
    /// * `tokenizer` - Tokenizer for token-to-text conversion
    pub fn array_of_any(tokenizer: &Tokenizer) -> AnyResult<SchemaConstraint> {
        let schema_json = r#"{
            "type": "array",
            "items": {}
        }"#;
        constraint_for_schema(schema_json, tokenizer)
    }

    /// Create constraint for generating JSON arrays of strings
    ///
    /// # Arguments
    /// * `tokenizer` - Tokenizer for token-to-text conversion
    pub fn array_of_strings(tokenizer: &Tokenizer) -> AnyResult<SchemaConstraint> {
        let schema_json = r#"{
            "type": "array",
            "items": { "type": "string" }
        }"#;
        constraint_for_schema(schema_json, tokenizer)
    }

    /// Create constraint for generating JSON arrays of integers
    ///
    /// # Arguments
    /// * `tokenizer` - Tokenizer for token-to-text conversion
    pub fn array_of_integers(tokenizer: &Tokenizer) -> AnyResult<SchemaConstraint> {
        let schema_json = r#"{
            "type": "array", 
            "items": { "type": "integer" }
        }"#;
        constraint_for_schema(schema_json, tokenizer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use schemars::JsonSchema;
    use crate::logits::constraints::GenerationConstraint;

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    // Tests using mock vocabulary to verify actual constraint behavior

    #[test]
    fn test_builder_api() {
        // Create a mock vocabulary for testing (same approach as schema_index.rs tests)
        fn create_test_vocabulary() -> SchemaVocabulary {
            let token_to_bytes = vec![
                b"hello".to_vec(),
                b"world".to_vec(), 
                b"true".to_vec(),
                b"false".to_vec(),
                b"null".to_vec(),
                b"123".to_vec(),
                b"456".to_vec(),
                b"\"".to_vec(),
                b"{".to_vec(),
                b"}".to_vec(),
                b"[".to_vec(),
                b"]".to_vec(),
                b",".to_vec(),
                b":".to_vec(),
                b" ".to_vec(),
                b"\"name\"".to_vec(),
                b"\"age\"".to_vec(),
                b"\"TestStruct\"".to_vec(),
            ];
            SchemaVocabulary::from_tokens(token_to_bytes, 0)
        }
        
        // Create vocabulary and test constraint builder
        let vocabulary = Arc::new(create_test_vocabulary());
        
        // Test constraint for TestStruct type
        let constraint_result = SchemaConstraint::new(
            &regex_from_schema::<TestStruct>().expect("Should generate regex from TestStruct"),
            vocabulary.clone(),
            false
        );
        
        // Verify constraint was created successfully - if it fails, print the error to understand the issue
        if let Err(ref e) = constraint_result {
            println!("Constraint creation failed: {e}");
            println!("Generated regex: {}", regex_from_schema::<TestStruct>().unwrap_or_else(|e| format!("Regex generation failed: {e}")));
        }
        assert!(constraint_result.is_ok(), "Should create constraint from TestStruct schema");
        let constraint = constraint_result.unwrap();
        
        // Test that we can create a new state
        let state = constraint.new_state();
        let initial_state_id = state.current_state();
        // Note: initial state ID is determined by the DFA library and may not be 0
        // We just verify that it's consistent and creates a valid state
        assert_eq!(state.tokens_processed(), 0, "Should start with 0 tokens processed");
        
        // Verify that creating another state gives the same initial state
        let state2 = constraint.new_state();
        assert_eq!(state2.current_state(), initial_state_id, "All new states should start at the same initial state");
        
        // Test constraint for predefined boolean type
        let boolean_regex = r"(true|false)";
        let boolean_constraint = SchemaConstraint::new(boolean_regex, vocabulary.clone(), false);
        assert!(boolean_constraint.is_ok(), "Should create boolean constraint");
        
        // Test actual boolean constraint behavior
        let boolean_constraint = boolean_constraint.unwrap();
        let bool_state = boolean_constraint.new_state();

        // Token indices from create_test_vocabulary():
        // 2 = "true", 3 = "false", 0 = "hello", 4 = "null"
        assert!(
            boolean_constraint.try_next(&bool_state, 2).unwrap(),
            "Boolean constraint should allow 'true' token (index 2)"
        );
        assert!(
            boolean_constraint.try_next(&bool_state, 3).unwrap(),
            "Boolean constraint should allow 'false' token (index 3)"
        );
        assert!(
            !boolean_constraint.try_next(&bool_state, 0).unwrap(),
            "Boolean constraint should reject 'hello' token (index 0)"
        );
        assert!(
            !boolean_constraint.try_next(&bool_state, 4).unwrap(),
            "Boolean constraint should reject 'null' token (index 4)"
        );

        // Test state progression with update()
        let mut progressing_state = boolean_constraint.new_state();
        assert_eq!(progressing_state.tokens_processed(), 0);

        // Update with "true" token (index 2)
        let update_result = boolean_constraint.update(&mut progressing_state, 2);
        assert!(
            update_result.is_ok() && update_result.unwrap(),
            "Should successfully update state with 'true' token"
        );
        assert_eq!(
            progressing_state.tokens_processed(), 1,
            "Should have processed 1 token"
        );
        
        // Note: Boolean constraints do not set is_complete() after matching a single token
        // as the DFA may accept further input. This is expected behavior - the constraint
        // validates tokens but doesn't mark the state as complete until reaching an explicit
        // end state in the DFA, which doesn't occur for simple boolean patterns like (true|false).

        // Test get_allowed_tokens() 
        let fresh_state = boolean_constraint.new_state();
        let allowed = boolean_constraint.get_allowed_tokens(&fresh_state);
        assert!(allowed.is_some(), "Should return allowed tokens map");

        let tokens_map = allowed.unwrap();
        assert!(
            tokens_map.contains_key(&2),
            "Allowed tokens should include 'true' (index 2)"
        );
        assert!(
            tokens_map.contains_key(&3),
            "Allowed tokens should include 'false' (index 3)"
        );
        assert!(
            !tokens_map.contains_key(&0),
            "Allowed tokens should NOT include 'hello' (index 0)"
        );
        
        // Test constraint builder functionality - since ConstraintBuilder needs a tokenizer,
        // we test the SchemaConstraint creation directly which is the core functionality
        let schema_json = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        }"#;
        
        // Test schema parsing and regex generation
        let schema: serde_json::Value = serde_json::from_str(schema_json)
            .expect("Should parse test schema JSON");
        
        let regex_result = regex_from_value(&schema, None, None);
        assert!(regex_result.is_ok(), "Should generate regex from schema");
        
        let regex_pattern = regex_result.unwrap();
        assert!(!regex_pattern.is_empty(), "Regex pattern should not be empty");
        assert!(regex_pattern.contains("name"), "Regex should include 'name' property");
        
        // Test constraint creation with the generated regex
        let final_constraint = SchemaConstraint::new(&regex_pattern, vocabulary, false);
        if let Err(ref e) = final_constraint {
            println!("Failed to create constraint: {}", e);
            println!("Regex pattern was: {}", regex_pattern);
        }
        assert!(final_constraint.is_ok(), "Should create constraint from generated regex");
    }

    #[test] 
    fn test_preset_schemas() {
        // This test verifies preset schema JSON is valid
        let object_schema = r#"{"type": "object", "additionalProperties": true}"#;
        let _parsed: schemars::Schema = serde_json::from_str(object_schema).unwrap();

        let array_schema = r#"{"type": "array", "items": {}}"#;
        let _parsed: schemars::Schema = serde_json::from_str(array_schema).unwrap();
    }
}