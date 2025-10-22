# Issue: Missing Input Validation

## Severity: MEDIUM
**Impact**: Crashes, poor error messages, wasted computation

## Location
Throughout `base.rs` and `loaded.rs`

## Problem Description

Neither implementation validates inputs before processing:

```rust
fn embed(&self, text: &str, task: Option<String>) -> ... {
    Box::pin(async move {
        self.validate_input(&text)?;  // ← Only in base.rs, not loaded.rs!
        // ... proceed with embedding
    })
}
```

But `validate_input()` is called in base.rs, not in loaded.rs!

## Missing Validations

### 1. Empty Text
```rust
// Current: No check
let embedding = model.embed("", None).await?;
// ↑ Wastes computation, returns meaningless embedding

// Should:
if text.trim().is_empty() {
    return Err("Cannot embed empty text".into());
}
```

### 2. Text Length
```rust
// Current: Truncated silently by tokenizer
let embedding = model.embed(&"x".repeat(100_000), None).await?;
// ↑ Tokenizer truncates to 8192 tokens, no warning

// Should:
let token_count = estimate_token_count(text);
if token_count > max_tokens {
    log::warn!("Text length {} exceeds max {}, will be truncated", 
               token_count, max_tokens);
}
```

### 3. Batch Size
```rust
// Current: No check
let embeddings = model.batch_embed(&vec!["text"; 1000], None).await?;
// ↑ Might OOM, no warning

// Should:
if texts.len() > self.max_batch_size() {
    return Err(format!(
        "Batch size {} exceeds maximum {}",
        texts.len(),
        self.max_batch_size()
    ).into());
}
```

### 4. Empty Batch
```rust
// Current: No check
let embeddings = model.batch_embed(&[], None).await?;
// ↑ Wastes computation, returns empty vec

// Should:
if texts.is_empty() {
    return Ok(Vec::new());  // Fast path
}
```

### 5. Invalid UTF-8 (Already Handled)
Rust's `&str` guarantees valid UTF-8, so this is safe.

## Current validate_input() in base.rs

Looking at the trait definition, `validate_input()` is likely from `TextEmbeddingCapable`:

```rust
// Likely in trait:
fn validate_input(&self, text: &str) -> Result<(), Box<dyn Error>> {
    if text.is_empty() {
        return Err("Empty text".into());
    }
    Ok(())
}
```

But **loaded.rs doesn't call it!**

## Impact

### Without Validation

```rust
// User passes empty string
let emb = model.embed("", None).await?;
// ↑ Succeeds, returns garbage embedding
// User doesn't know there's a problem
```

### With Validation

```rust
// User passes empty string
let emb = model.embed("", None).await?;
// ↑ Returns Err("Cannot embed empty text")
// User knows immediately
```

## Recommendation

Add comprehensive validation to `LoadedStellaModel`:

```rust
impl LoadedStellaModel {
    fn validate_single_input(&self, text: &str) -> Result<(), String> {
        // Check empty
        if text.trim().is_empty() {
            return Err("Cannot embed empty text".to_string());
        }
        
        // Check length (rough estimate: 1 token ≈ 4 chars)
        let estimated_tokens = text.len() / 4;
        let max_tokens = self.info().max_input_tokens.unwrap().get() as usize;
        
        if estimated_tokens > max_tokens * 2 {
            log::warn!(
                "Text length ~{} tokens may exceed max {} tokens and will be truncated",
                estimated_tokens,
                max_tokens
            );
        }
        
        Ok(())
    }
    
    fn validate_batch_input(&self, texts: &[String]) -> Result<(), String> {
        // Check empty batch
        if texts.is_empty() {
            return Ok(());  // Allow empty batch, return empty result
        }
        
        // Check batch size
        let max_batch = self.max_batch_size();
        if texts.len() > max_batch {
            return Err(format!(
                "Batch size {} exceeds maximum {} for {:?} variant",
                texts.len(),
                max_batch,
                self.variant
            ));
        }
        
        // Check each text
        for (i, text) in texts.iter().enumerate() {
            if let Err(e) = self.validate_single_input(text) {
                return Err(format!("Invalid text at index {}: {}", i, e));
            }
        }
        
        Ok(())
    }
}

// In embed():
fn embed(&self, text: &str, task: Option<String>) -> ... {
    Box::pin(async move {
        self.validate_single_input(text)
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn Error + Send + Sync>)?;
        
        // ... rest of implementation
    })
}

// In batch_embed():
fn batch_embed(&self, texts: &[String], task: Option<String>) -> ... {
    Box::pin(async move {
        self.validate_batch_input(texts)
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn Error + Send + Sync>)?;
        
        // Fast path for empty batch
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        
        // ... rest of implementation
    })
}
```

## Benefits

1. **Fail Fast**: Catch errors before expensive computation
2. **Better Errors**: Clear messages about what's wrong
3. **Performance**: Skip processing for empty inputs
4. **Safety**: Prevent OOM from oversized batches
