# `packages/simd/src/logits/constraints/schema.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: simd
- **File Hash**: 0e321c86  
- **Timestamp**: 2025-10-10T02:15:58.222698+00:00  
- **Lines of Code**: 366

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 366 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Orphaned Methods


### `string()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 371)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a string constraint with optional pattern and length
    pub fn string(
        vocabulary: Arc<SchemaVocabulary>,
        pattern: Option<String>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `number()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 361)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a number constraint with optional range
    pub fn number(
        vocabulary: Arc<SchemaVocabulary>,
        min: Option<f64>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `null()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 345)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a null constraint
    pub fn null(vocabulary: Arc<SchemaVocabulary>) -> AnyResult<SchemaConstraint> {
        SchemaConstraintBuilder::new(vocabulary)
            .from_predefined(&PredefinedSchema::Null)
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `boolean()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 339)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a boolean constraint (true/false)
    pub fn boolean(vocabulary: Arc<SchemaVocabulary>) -> AnyResult<SchemaConstraint> {
        SchemaConstraintBuilder::new(vocabulary)
            .from_predefined(&PredefinedSchema::Boolean)
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `simple_object()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 410)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a simple object constraint
    pub fn simple_object(
        vocabulary: Arc<SchemaVocabulary>,
        properties: Vec<(String, SchemaType)>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `from_rust_type()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 424)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create constraint from Rust type
    pub fn from_rust_type<T>(vocabulary: Arc<SchemaVocabulary>) -> AnyResult<SchemaConstraint>
    where
        T: JsonSchema + Serialize,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `integer()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 351)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create an integer constraint with optional range
    pub fn integer(
        vocabulary: Arc<SchemaVocabulary>,
        min: Option<i64>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `array()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 395)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create an array constraint
    pub fn array(
        vocabulary: Arc<SchemaVocabulary>,
        items: SchemaType,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `string_enum()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema.rs` (line 386)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create an enum constraint from string values
    pub fn string_enum(
        vocabulary: Arc<SchemaVocabulary>,
        values: Vec<String>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym