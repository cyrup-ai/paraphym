# Issue: Error Messages Lose Original Error Context

## Severity: LOW
**Impact**: Harder debugging, lost error information

## Location
Throughout `base.rs` and `loaded.rs`

## Problem Description

Many error conversions use `format!()` which converts the original error to a string, losing:
- Error type information
- Stack traces (if any)
- Structured error data
- Error chain

### Examples

```rust
// base.rs:104
let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
    .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
//                      ↑ Original error type lost
```

```rust
// loaded.rs:239
let tokens = tokenizer
    .encode(formatted_text, true)
    .map_err(|e| format!("Tokenization failed: {}", e))?;
//                      ↑ Original tokenizers::Error lost
```

```rust
// base.rs:169
let mut model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)
    .map_err(|e| format!("Failed to create Stella model: {}", e))?;
//                      ↑ Original candle_core::Error lost
```

## Impact on Debugging

When an error occurs, developers see:
```
Error: "Failed to load tokenizer: invalid JSON at line 42"
```

Instead of:
```
Error: TokenizerError::InvalidJson {
    line: 42,
    column: 15,
    path: "/path/to/tokenizer.json",
    source: serde_json::Error { ... }
}
```

The formatted string loses:
- Exact error variant
- Structured fields (line, column, path)
- Source error chain
- Type information for error matching

## Better Approach

### Option 1: Preserve Error Type with Custom Error Enum

```rust
#[derive(Debug, thiserror::Error)]
pub enum StellaError {
    #[error("Failed to load tokenizer: {0}")]
    TokenizerLoad(#[from] tokenizers::Error),
    
    #[error("Model creation failed: {0}")]
    ModelCreation(#[from] candle_core::Error),
    
    #[error("Tokenization failed: {0}")]
    Tokenization(#[source] tokenizers::Error),
    
    // ... other variants
}

// Usage:
let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
    .map_err(StellaError::TokenizerLoad)?;
```

**Pros**:
- Preserves original error
- Enables error matching
- Better error chains
- Type-safe

**Cons**:
- More code
- Requires thiserror or similar

### Option 2: Use anyhow for Error Context

```rust
use anyhow::Context;

let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
    .context("Failed to load tokenizer")?;
//  ↑ Wraps error with context, preserves original
```

**Pros**:
- Simple
- Preserves error chain
- Good error messages

**Cons**:
- Less type-safe
- Can't match on specific errors

### Option 3: Keep Original Error, Add Context

```rust
let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
    .map_err(|e| {
        log::error!("Failed to load tokenizer from {:?}: {:?}", tokenizer_path, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;
```

**Pros**:
- Preserves original error
- Adds logging for context
- No new dependencies

**Cons**:
- Verbose
- Logging might not be seen

## Current Return Type

```rust
Box<dyn std::error::Error + Send + Sync>
```

This is already a trait object, so we can return any error type. The issue is we're converting to `String` first.

## Recommendation

**Use Option 2 (anyhow)** for quick improvement:

```rust
use anyhow::{Context, Result};

fn embed(&self, text: &str, task: Option<String>) 
    -> Pin<Box<dyn Future<Output = Result<Vec<f32>>> + Send + '_>>
{
    Box::pin(async move {
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .context("Failed to load tokenizer")?;
        
        let model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)
            .context("Failed to create Stella model")?;
        
        // ... rest of code
    })
}
```

This preserves error chains while adding context, making debugging much easier.

## Examples of Lost Information

### Current (Bad)
```
Error: "Tokenization failed: sequence too long"
```

### With Proper Error Handling (Good)
```
Error: Tokenization failed

Caused by:
    0: Input sequence length 9847 exceeds maximum 8192
    1: Token IDs: [101, 2023, 2003, ...]
    2: At position 8192 in file "input.txt"
```
