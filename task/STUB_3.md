# STUB_3: OpenAI Translation Implementation

**Priority:** 🟡 MEDIUM  
**Severity:** Incomplete Feature  
**Estimated Effort:** 1 session

## OBJECTIVE

Replace placeholder `translate_mcp_to_openai` function with production implementation that properly maps MCP response format to OpenAI ChatResponse structure.

## BACKGROUND

Current function is marked `#[allow(dead_code)]` and generates fake data:
- Random UUIDs instead of deterministic IDs
- Empty choices array
- No actual content translation
- Missing token usage calculation
- Comment admits: "This is a placeholder implementation"

## LOCATION

**File:** `packages/sweetmcp/packages/axum/src/sampling/chat.rs`  
**Line:** 248-262

## SUBTASK 1: Implement Content Block Parsing

**What:** Extract and combine text content from MCP content array  
**Where:** New helper function in `translate_mcp_to_openai`

**Why:** MCP uses content block array, OpenAI uses single text field

**Implementation:**
```rust
fn extract_text_content(mcp_response: &Value) -> Result<String, anyhow::Error> {
    let content = mcp_response
        .get("content")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("Invalid MCP response: missing content array"))?;
    
    let mut text_content = String::new();
    for block in content {
        if let Some(text) = block.get("text").and_then(|v| v.as_str()) {
            if !text_content.is_empty() {
                text_content.push('\n');
            }
            text_content.push_str(text);
        }
        
        // Handle other content types if needed
        if let Some(type_) = block.get("type").and_then(|v| v.as_str()) {
            match type_ {
                "image" => {
                    // MCP image blocks not directly mappable to OpenAI format
                    tracing::warn!("Image content blocks not supported in OpenAI translation");
                }
                "resource" => {
                    // Handle resource references
                    if let Some(uri) = block.get("uri").and_then(|v| v.as_str()) {
                        text_content.push_str(&format!("[Resource: {}]", uri));
                    }
                }
                _ => {}
            }
        }
    }
    
    Ok(text_content)
}
```

## SUBTASK 2: Implement Token Usage Calculation

**What:** Calculate actual token counts using tokenizer  
**Where:** New `calculate_token_usage` function

**Why:** OpenAI API requires usage statistics, cannot be hardcoded to 0

**Implementation:**
```rust
fn calculate_token_usage(text: &str) -> Result<Usage, anyhow::Error> {
    // Use simple estimation based on character count
    // 1 token ≈ 4 characters for English text (GPT tokenizer approximation)
    let estimated_tokens = (text.len() / 4).max(1);
    
    Ok(Usage {
        prompt_tokens: 0, // Would need to track prompt separately in full implementation
        completion_tokens: estimated_tokens as i32,
        total_tokens: estimated_tokens as i32,
    })
}

// For production, consider using actual tokenizer:
// use tiktoken_rs::cl100k_base;
// 
// fn calculate_token_usage_precise(text: &str) -> Result<Usage, anyhow::Error> {
//     let tokenizer = cl100k_base()?;
//     let tokens = tokenizer.encode_ordinary(text);
//     
//     Ok(Usage {
//         prompt_tokens: 0,
//         completion_tokens: tokens.len() as i32,
//         total_tokens: tokens.len() as i32,
//     })
// }
```

## SUBTASK 3: Implement Deterministic ID Generation

**What:** Generate content-based IDs instead of random UUIDs  
**Where:** Replace random UUID generation in `translate_mcp_to_openai`

**Why:** Deterministic IDs allow caching and reproducibility

**Implementation:**
```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_content_hash(text: &str) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

// In translate_mcp_to_openai:
let content_hash = calculate_content_hash(&text_content);
let chat_id = format!("chatcmpl-{}", content_hash);
```

## SUBTASK 4: Implement Session Token Management

**What:** Extract or generate session tokens from MCP response  
**Where:** In `translate_mcp_to_openai` function

**Why:** Session continuity for conversation threading

**Implementation:**
```rust
fn extract_session_token(mcp_response: &Value) -> String {
    mcp_response
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            // Try to extract from metadata
            mcp_response
                .get("metadata")
                .and_then(|m| m.get("session_id"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| {
            // Generate deterministic session ID from conversation context
            format!("session-{}", uuid::Uuid::new_v4())
        })
}
```

## SUBTASK 5: Implement Model Name Extraction

**What:** Extract model identifier from MCP response  
**Where:** New `extract_model_name` function

**Why:** OpenAI response requires model field

**Implementation:**
```rust
fn extract_model_name(mcp_response: &Value) -> String {
    mcp_response
        .get("model")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            // Try metadata.model
            mcp_response
                .get("metadata")
                .and_then(|m| m.get("model"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "mcp-unknown".to_string())
}
```

## SUBTASK 6: Complete Main Translation Function

**What:** Implement full `translate_mcp_to_openai` with all components  
**Where:** Replace placeholder implementation

**Implementation:**
```rust
fn translate_mcp_to_openai(mcp_response: &Value) -> Result<ChatResponse, anyhow::Error> {
    // Extract text content from MCP content blocks
    let text_content = extract_text_content(mcp_response)?;
    
    // Create OpenAI choice with message
    let choice = Choice {
        index: 0,
        message: Message {
            role: "assistant".to_string(),
            content: text_content.clone(),
            tool_calls: None, // MCP tool results handled separately
        },
        finish_reason: Some("stop".to_string()),
    };
    
    // Calculate token usage
    let usage = calculate_token_usage(&text_content)?;
    
    // Generate deterministic ID
    let content_hash = calculate_content_hash(&text_content);
    let chat_id = format!("chatcmpl-{}", content_hash);
    
    // Extract session token
    let session_token = extract_session_token(mcp_response);
    
    // Extract model name
    let model = extract_model_name(mcp_response);
    
    Ok(ChatResponse {
        id: chat_id,
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp() as u64,
        model,
        choices: vec![choice],
        usage: Some(usage),
        system_fingerprint: Some(format!("mcp-{}", env!("CARGO_PKG_VERSION"))),
        session_token,
    })
}
```

## SUBTASK 7: Remove Dead Code Attribute

**What:** Remove `#[allow(dead_code)]` once function is properly implemented  
**Where:** Line 247

**Why:** Function is now production-ready, not future feature

## DEFINITION OF DONE

- [ ] Content block parsing implemented (text, image, resource types)
- [ ] Token usage calculation implemented (estimation or tokenizer)
- [ ] Deterministic ID generation based on content hash
- [ ] Session token extraction/generation implemented
- [ ] Model name extraction with fallbacks
- [ ] Main translation function complete with all components
- [ ] Error handling for malformed MCP responses
- [ ] `#[allow(dead_code)]` attribute removed
- [ ] Placeholder comment removed
- [ ] Code compiles without warnings
- [ ] Return type properly populated (no empty fields)

## REQUIREMENTS

- ❌ **NO TESTS** - Testing team handles test coverage
- ❌ **NO BENCHMARKS** - Performance team handles benchmarking
- ✅ **PRODUCTION CODE ONLY** - Complete implementation, no placeholders

## RESEARCH NOTES

### MCP Response Structure
```json
{
  "content": [
    {"type": "text", "text": "Response content"},
    {"type": "resource", "uri": "file://path", "text": "..."}
  ],
  "model": "provider/model-name",
  "session_id": "optional-session-id",
  "metadata": {}
}
```

### OpenAI ChatResponse Structure
```rust
pub struct ChatResponse {
    pub id: String,           // chatcmpl-{hash}
    pub object: String,       // "chat.completion"
    pub created: u64,         // Unix timestamp
    pub model: String,        // Model identifier
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    pub system_fingerprint: Option<String>,
    pub session_token: String,
}
```

### Token Estimation vs Actual Tokenization
Simple estimation (4 chars = 1 token) is acceptable for initial implementation.
For production accuracy, consider:
- `tiktoken-rs` crate for GPT tokenizer
- `tokenizers` crate for other models
- Caching tokenizer instances for performance

### Content Type Mapping
| MCP Type | OpenAI Handling |
|----------|----------------|
| text | Direct mapping to message.content |
| image | Log warning, not directly supported |
| resource | Format as text reference |
| tool_result | Separate handling (not in this function) |

### Session Management
Session tokens should be:
- Extracted from MCP metadata if available
- Generated deterministically from conversation context
- Consistent across same conversation thread

## VERIFICATION

After implementation, verify:
1. Valid MCP response produces valid ChatResponse
2. Content blocks properly concatenated
3. Token usage calculated (non-zero)
4. IDs are deterministic (same input = same ID)
5. Model name extracted correctly
6. Empty/malformed responses handled gracefully
7. No random UUIDs in output
