# VISION_3: LLaVA Vision-Language Provider

## OBJECTIVE

Implement the LLaVA (Large Language and Vision Assistant) provider to enable vision-language understanding including visual question answering, image description, and multi-turn image-grounded conversations.

---

## BACKGROUND

**Current State**:
- ClipVisionProvider exists for image embeddings
- No vision-language models implemented
- CompletionProvider enum is text-only

**What This Task Accomplishes**:
- First vision-language provider
- Visual question answering capability
- Image-grounded chat functionality
- Two-stage ImageNet normalization pattern

---

## SUBTASK 1: Create LLaVA Configuration and Provider Struct

**File**: `packages/candle/src/providers/llava.rs` (NEW)

**What to Create**:
```rust
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::{
    clip::ClipVisionModel,
    llama::Llama,
};
use crate::builders::image::{Image, ResizeFilter};

/// LLaVA model configuration
/// 
/// Default settings match LLaVA 1.5 with 336×336 images
/// and ImageNet normalization parameters.
#[derive(Clone)]
pub struct LLaVAConfig {
    /// Image input size (336 for LLaVA 1.5)
    pub image_size: usize,
    /// ImageNet mean for normalization
    pub image_mean: [f32; 3],
    /// ImageNet std for normalization
    pub image_std: [f32; 3],
}

impl Default for LLaVAConfig {
    fn default() -> Self {
        Self {
            image_size: 336,
            image_mean: [0.48145466, 0.4578275, 0.40821073],
            image_std: [0.26862954, 0.2613026, 0.2757771],
        }
    }
}

/// LLaVA vision-language provider
/// 
/// Combines CLIP vision encoder with LLaMA language model
/// for image understanding and visual question answering.
#[derive(Clone)]
pub struct LLaVAProvider {
    vision_model: ClipVisionModel,
    language_model: Llama,
    config: LLaVAConfig,
    device: Device,
}

impl LLaVAProvider {
    pub fn from_pretrained(model_path: &str, device: Device) -> Result<Self, String> {
        let config = LLaVAConfig::default();
        
        // Load vision encoder (CLIP)
        let vision_vb = VarBuilder::from_mmaped_safetensors(
            &[format!("{}/vision_model.safetensors", model_path).into()],
            candle_core::DType::F32,
            &device
        ).map_err(|e| format!("Failed to load vision model: {}", e))?;
        
        let vision_model = ClipVisionModel::new(vision_vb, &Default::default())
            .map_err(|e| format!("Failed to create vision model: {}", e))?;
        
        // Load language model (LLaMA)
        let language_vb = VarBuilder::from_mmaped_safetensors(
            &[format!("{}/language_model.safetensors", model_path).into()],
            candle_core::DType::F32,
            &device
        ).map_err(|e| format!("Failed to load language model: {}", e))?;
        
        let language_model = Llama::load(language_vb, &Default::default())
            .map_err(|e| format!("Failed to create language model: {}", e))?;
        
        Ok(Self {
            vision_model,
            language_model,
            config,
            device,
        })
    }
}
```

**Why**: Establishes LLaVA architecture with vision + language components.

**Definition of Done**:
- ✅ LLaVAConfig struct with ImageNet normalization params
- ✅ LLaVAProvider struct with vision and language models
- ✅ `from_pretrained()` loads both models successfully
- ✅ Default config matches LLaVA 1.5 specifications

---

## SUBTASK 2: Implement Image Preprocessing Methods

**File**: `packages/candle/src/providers/llava.rs`

**What to Add**:
```rust
impl LLaVAProvider {
    /// Preprocess image from file path using LLaVA pattern
    /// 
    /// LLaVA uses two-stage normalization:
    /// 1. [0,255] → [0,1] via normalize_unsigned()
    /// 2. (x - mean) / std via normalize_with()
    async fn preprocess_image(&self, image_path: &str) -> Result<Tensor, String> {
        Image::from_path(image_path)
            .resize(
                self.config.image_size,
                self.config.image_size,
                ResizeFilter::CatmullRom  // LLaVA uses CatmullRom (bicubic)
            )
            .normalize_unsigned()  // Step 1: [0,255] → [0,1]
            .normalize_with(       // Step 2: ImageNet normalization
                self.config.image_mean,
                self.config.image_std
            )
            .to_tensor(&self.device)
            .await
    }

    /// Preprocess image from URL
    async fn preprocess_image_url(&self, url: &str) -> Result<Tensor, String> {
        Image::from_url(url)
            .resize(self.config.image_size, self.config.image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(self.config.image_mean, self.config.image_std)
            .to_tensor(&self.device)
            .await
    }

    /// Preprocess image from base64
    async fn preprocess_image_base64(&self, base64: &str) -> Result<Tensor, String> {
        Image::from_base64(base64)
            .resize(self.config.image_size, self.config.image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(self.config.image_mean, self.config.image_std)
            .to_tensor(&self.device)
            .await
    }
}
```

**Why**: LLaVA requires specific two-stage normalization (different from CLIP).

**Critical**: Must use `.normalize_unsigned()` then `.normalize_with()` (two stages).

**Definition of Done**:
- ✅ `preprocess_image()` uses CatmullRom filter
- ✅ Two-stage normalization applied correctly
- ✅ URL and base64 variants implemented
- ✅ Image builder is ONLY preprocessing mechanism

---

## SUBTASK 3: Implement Visual Question Answering

**File**: `packages/candle/src/providers/llava.rs`

**What to Add**:
```rust
impl LLaVAProvider {
    /// Answer question about an image
    /// 
    /// Takes image path and question, returns text answer.
    /// Internally preprocesses image, encodes to features,
    /// and generates response with language model.
    pub async fn ask(
        &self,
        image_path: &str,
        question: &str
    ) -> Result<String, String> {
        // 1. Preprocess image
        let image_tensor = self.preprocess_image(image_path).await?;
        
        // 2. Extract vision features
        let vision_features = self.vision_model
            .forward(&image_tensor.unsqueeze(0)?)
            .map_err(|e| format!("Vision encoding failed: {}", e))?;
        
        // 3. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);
        
        // 4. Tokenize prompt
        let tokens = self.tokenize(&prompt)?;
        
        // 5. Generate response with vision context
        let output_tokens = self.language_model
            .forward_with_vision(&tokens, &vision_features)
            .map_err(|e| format!("Generation failed: {}", e))?;
        
        // 6. Decode tokens to text
        self.decode(&output_tokens)
    }

    /// Answer question about image from URL
    pub async fn ask_url(
        &self,
        image_url: &str,
        question: &str
    ) -> Result<String, String> {
        let image_tensor = self.preprocess_image_url(image_url).await?;
        
        let vision_features = self.vision_model
            .forward(&image_tensor.unsqueeze(0)?)
            .map_err(|e| format!("Vision encoding failed: {}", e))?;
        
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);
        let tokens = self.tokenize(&prompt)?;
        let output_tokens = self.language_model
            .forward_with_vision(&tokens, &vision_features)
            .map_err(|e| format!("Generation failed: {}", e))?;
        
        self.decode(&output_tokens)
    }

    /// Helper: Tokenize text
    fn tokenize(&self, text: &str) -> Result<Tensor, String> {
        // Use language model's tokenizer
        self.language_model.tokenizer()
            .encode(text, true)
            .map_err(|e| format!("Tokenization failed: {}", e))
            .and_then(|encoding| {
                let tokens = encoding.get_ids();
                Tensor::new(tokens, &self.device)
                    .map_err(|e| format!("Failed to create token tensor: {}", e))
            })
    }

    /// Helper: Decode tokens to text
    fn decode(&self, tokens: &Tensor) -> Result<String, String> {
        let token_ids = tokens.to_vec1::<u32>()
            .map_err(|e| format!("Failed to extract token IDs: {}", e))?;
        
        self.language_model.tokenizer()
            .decode(&token_ids, true)
            .map_err(|e| format!("Decoding failed: {}", e))
    }
}
```

**Why**: Core visual question answering functionality.

**Note**: If `forward_with_vision()` doesn't exist, use `forward()` with vision features concatenated to embeddings.

**Definition of Done**:
- ✅ `ask()` answers questions about images
- ✅ `ask_url()` works for web images
- ✅ Tokenization and decoding helpers work
- ✅ Vision features properly integrated with language model
- ✅ Output is coherent text response

---

## SUBTASK 4: Implement Streaming Chat

**File**: `packages/candle/src/providers/llava.rs`

**What to Add**:
```rust
use ystream::AsyncStream;

impl LLaVAProvider {
    /// Stream chat responses token by token
    /// 
    /// Returns AsyncStream of text chunks for real-time display.
    pub async fn stream_chat(
        &self,
        image_path: &str,
        question: &str
    ) -> AsyncStream<String> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        
        let image_path = image_path.to_string();
        let question = question.to_string();
        let provider = self.clone();
        
        tokio::spawn(async move {
            // Preprocess image
            let image_tensor = match provider.preprocess_image(&image_path).await {
                Ok(t) => t,
                Err(e) => {
                    let _ = tx.send(format!("Error: {}", e));
                    return;
                }
            };
            
            // Extract vision features
            let vision_features = match provider.vision_model.forward(&image_tensor.unsqueeze(0).unwrap()) {
                Ok(f) => f,
                Err(e) => {
                    let _ = tx.send(format!("Error: {}", e));
                    return;
                }
            };
            
            // Format and tokenize prompt
            let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);
            let tokens = match provider.tokenize(&prompt) {
                Ok(t) => t,
                Err(e) => {
                    let _ = tx.send(format!("Error: {}", e));
                    return;
                }
            };
            
            // Stream generation token by token
            let mut generated_tokens = Vec::new();
            while generated_tokens.len() < 512 {  // Max 512 tokens
                let next_token = match provider.language_model.forward_next(&tokens, &vision_features, &generated_tokens) {
                    Ok(t) => t,
                    Err(e) => {
                        let _ = tx.send(format!("Error: {}", e));
                        break;
                    }
                };
                
                generated_tokens.push(next_token);
                
                // Decode and send token
                if let Ok(text) = provider.decode_token(next_token) {
                    let _ = tx.send(text);
                }
                
                // Check for EOS
                if next_token == provider.language_model.eos_token() {
                    break;
                }
            }
        });
        
        AsyncStream::new(rx)
    }

    /// Helper: Decode single token
    fn decode_token(&self, token: u32) -> Result<String, String> {
        self.language_model.tokenizer()
            .decode(&[token], false)
            .map_err(|e| format!("Token decode failed: {}", e))
    }
}
```

**Why**: Enables real-time streaming responses for better UX.

**Note**: Adapt to actual LLaMA generation API - may need incremental generation method.

**Definition of Done**:
- ✅ `stream_chat()` returns AsyncStream<String>
- ✅ Tokens streamed incrementally as generated
- ✅ EOS token detection stops generation
- ✅ Error handling in async context

---

## SUBTASK 5: Update Providers Module

**File**: `packages/candle/src/providers/mod.rs`

**What to Add**:
```rust
pub mod llava;
pub use llava::{LLaVAProvider, LLaVAConfig};
```

**Why**: Makes LLaVA provider publicly accessible.

**Definition of Done**:
- ✅ Module export added
- ✅ `use paraphym_candle::providers::LLaVAProvider;` compiles

---

## RESEARCH REFERENCES

### LLaVA Reference Implementation
- **File**: [`tmp/candle-examples/candle-examples/examples/llava/image_processor.rs`](../tmp/candle-examples/candle-examples/examples/llava/image_processor.rs)
- **Lines 105-114**: CatmullRom resize pattern
- **Lines 138-140**: First normalization (rescale to [0,1])
- **Lines 142-151**: Second normalization (ImageNet mean/std)
- **Two-stage pattern**: `rescale()` then `normalize()` with broadcast operations

### Image Builder Integration
- **File**: [`packages/candle/src/builders/image.rs`](../packages/candle/src/builders/image.rs)
- **Line 375**: `.normalize_unsigned()` - [0,255] → [0,1]
- **Line 385**: `.normalize_with(mean, std)` - ImageNet normalization
- **Lines 698-765**: Two-stage normalization implementation

### Model Architecture
- **Vision Encoder**: CLIP ViT (same as standalone CLIP)
- **Language Model**: LLaMA 7B or 13B
- **Projection**: Linear layer maps vision → language space
- **Prompt Format**: `USER: <image>\n{question}\nASSISTANT:`

---

## CRITICAL REQUIREMENTS

### ✅ Two-Stage Normalization (LLaVA-Specific)
- **MUST USE** `.normalize_unsigned()` first (scale to [0,1])
- **MUST USE** `.normalize_with(mean, std)` second (ImageNet norm)
- **DO NOT** use `.normalize_signed()` (that's CLIP-specific)
- Order matters: unsigned → with_params

### ✅ Image Builder Integration
- **MUST USE** `Image::from_path()` for file inputs
- **MUST USE** `Image::from_url()` for URL inputs
- **MUST USE** `Image::from_base64()` for base64 inputs
- **MUST USE** `.resize(336, 336, ResizeFilter::CatmullRom)`
- **NO MANUAL** image preprocessing allowed

### ✅ Vision-Language Integration
- Vision features properly concatenated/projected to language space
- `<image>` token in prompt marks where vision features inject
- Token generation includes both text and vision context

---

## DEFINITION OF DONE

1. ✅ File `packages/candle/src/providers/llava.rs` created
2. ✅ LLaVAConfig struct with ImageNet params (336×336, mean, std)
3. ✅ LLaVAProvider struct with vision and language models
4. ✅ `from_pretrained()` loads both models successfully
5. ✅ `preprocess_image()` uses two-stage normalization
6. ✅ `preprocess_image_url()` and `preprocess_image_base64()` work
7. ✅ `ask()` answers visual questions correctly
8. ✅ `ask_url()` works for web-hosted images
9. ✅ `stream_chat()` provides real-time token streaming
10. ✅ Tokenization and decoding helpers functional
11. ✅ Image builder is ONLY preprocessing mechanism
12. ✅ Module export in `providers/mod.rs` complete
13. ✅ File compiles without errors

---

## NO TESTS OR BENCHMARKS

**Do NOT create**:
- Unit tests for Q&A accuracy
- Integration tests for vision-language tasks
- Benchmark comparisons with other models
- Example conversations or test images

**Reason**: Testing team handles validation. Focus on implementation only.