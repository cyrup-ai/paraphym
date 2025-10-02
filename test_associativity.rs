// Test associativity fix
use std::collections::HashMap;
use paraphym_candle::domain::chat::templates::{
    parser::TemplateParser,
    compiler::TemplateCompiler,
    core::TemplateContext,
};

fn main() {
    let parser = TemplateParser::new();
    let compiler = TemplateCompiler::new();
    
    // Test 1: Subtraction associativity
    println!("Test 1: {{ 10 - 3 - 2 }}");
    match parser.parse("{{ 10 - 3 - 2 }}") {
        Ok(ast) => {
            println!("  Parsed AST: {:#?}", ast);
            match compiler.compile(&ast) {
                Ok(compiled) => {
                    let context = TemplateContext::new();
                    match compiled.render(&context) {
                        Ok(result) => println!("  Result: {} (expected: 5)", result),
                        Err(e) => println!("  Render error: {}", e),
                    }
                }
                Err(e) => println!("  Compile error: {}", e),
            }
        }
        Err(e) => println!("  Parse error: {}", e),
    }
    
    // Test 2: Division associativity
    println!("\nTest 2: {{ 20 / 4 / 2 }}");
    match parser.parse("{{ 20 / 4 / 2 }}") {
        Ok(ast) => {
            println!("  Parsed AST: {:#?}", ast);
            match compiler.compile(&ast) {
                Ok(compiled) => {
                    let context = TemplateContext::new();
                    match compiled.render(&context) {
                        Ok(result) => println!("  Result: {} (expected: 2.5)", result),
                        Err(e) => println!("  Render error: {}", e),
                    }
                }
                Err(e) => println!("  Compile error: {}", e),
            }
        }
        Err(e) => println!("  Parse error: {}", e),
    }
}
