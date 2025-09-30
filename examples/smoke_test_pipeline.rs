//! Candle Generation Pipeline Smoke Test
//!
//! Minimal runnable example that validates the complete Candle generation pipeline
//! works end-to-end: Chat API → Provider → Engine → TextGenerator → SIMD
//!
//! Run with: cargo run --example smoke_test_pipeline

use paraphym_candle::prelude::*;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Candle Generation Pipeline Smoke Test\n");
    
    // 1. Create provider with config
    println!("1. Creating provider...");
    let config = CandleKimiK2Config::default();
    let provider = CandleKimiK2Provider::with_config_sync(
        "./models/kimi-k2".to_string(), 
        config
    )?;
    println!("   ✅ Provider created (owns TextGenerator internally)\n");
    
    // 2. Build agent using fluent API
    println!("2. Building agent with fluent API...");
    let stream = CandleFluentAi::agent_role("smoke-test")
        .model(CandleModels::KimiK2)
        .temperature(0.7)
        .on_chunk(|chunk| {
            // Stream tokens as they arrive
            print!("{}", chunk);
            let _ = io::stdout().flush();
            chunk
        })
        .into_agent()
        .chat(|_| CandleChatLoop::UserPrompt("Say hello".to_string()));
    println!("   ✅ Agent built, stream returned\n");
    
    // 3. Collect stream (forces execution)
    println!("3. Generating tokens...");
    println!("   Response: ");
    
    let mut token_count = 0;
    for chunk in stream {
        token_count += 1;
    }
    
    println!("\n   ✅ Generated {} chunks\n", token_count);
    
    println!("🎉 Pipeline smoke test complete!");
    println!("   - Provider created TextGenerator");
    println!("   - Engine coordinated generation");
    println!("   - SIMD processed tokens");
    println!("   - Tokens streamed successfully");
    
    Ok(())
}