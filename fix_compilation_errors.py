#!/usr/bin/env python3
"""
Systematically fix all compilation errors identified in cargo check
"""

import os
import re
from pathlib import Path

# Base directory
BASE_DIR = Path("/Volumes/samsung_t9/paraphym/packages/candle/src")

print("=" * 80)
print("FIXING COMPILATION ERRORS - SYSTEMATIC APPROACH")
print("=" * 80)

# Fix 1: Remove duplicate tests module in orchestration.rs
print("\n[1/10] Fixing duplicate tests module in orchestration.rs...")
orchestration_file = BASE_DIR / "domain/chat/orchestration.rs"
if orchestration_file.exists():
    with open(orchestration_file, 'r') as f:
        content = f.read()
    
    # Find both test modules
    lines = content.split('\n')
    first_test_start = None
    second_test_start = None
    
    for i, line in enumerate(lines):
        if line.strip().startswith('mod tests {') or line.strip() == '#[cfg(test)]':
            if first_test_start is None:
                first_test_start = i
            else:
                second_test_start = i
                break
    
    if second_test_start:
        # Find end of second test module
        brace_count = 0
        second_test_end = second_test_start
        for i in range(second_test_start, len(lines)):
            if '{' in lines[i]:
                brace_count += lines[i].count('{')
            if '}' in lines[i]:
                brace_count -= lines[i].count('}')
            if brace_count == 0 and i > second_test_start:
                second_test_end = i
                break
        
        # Remove second test module
        new_lines = lines[:second_test_start] + lines[second_test_end+1:]
        new_content = '\n'.join(new_lines)
        
        with open(orchestration_file, 'w') as f:
            f.write(new_content)
        print(f"   ✓ Removed duplicate tests module (lines {second_test_start}-{second_test_end})")
    else:
        print("   ℹ No duplicate found")
else:
    print("   ✗ File not found")

# Fix 2: Add From<String> implementation for MemoryError
print("\n[2/10] Adding From<String> impl for MemoryError...")
error_file = BASE_DIR / "memory/utils/error.rs"
if error_file.exists():
    with open(error_file, 'r') as f:
        content = f.read()
    
    # Check if From<String> already exists
    if 'impl From<String> for Error' not in content:
        # Find where to add the impl (after other From impls)
        # Add before the end of file
        new_impl = '''
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
'''
        # Insert before final closing braces/EOF
        content = content.rstrip() + '\n' + new_impl + '\n'
        
        with open(error_file, 'w') as f:
            f.write(content)
        print("   ✓ Added From<String> and From<&str> implementations")
    else:
        print("   ℹ From<String> already exists")
else:
    print("   ✗ File not found")

# Fix 3: Fix progresshub ZeroOneOrMany::None to ::Zero in qwen3_coder
print("\n[3/10] Fixing ZeroOneOrMany::None to ::Zero in qwen3_coder.rs...")
qwen_file = BASE_DIR / "providers/qwen3_coder.rs"
if qwen_file.exists():
    with open(qwen_file, 'r') as f:
        content = f.read()
    
    # Replace None with Zero for progresshub ZeroOneOrMany
    content = re.sub(
        r'progresshub::types::ZeroOneOrMany::None',
        r'progresshub::types::ZeroOneOrMany::Zero',
        content
    )
    
    with open(qwen_file, 'w') as f:
        f.write(content)
    print("   ✓ Fixed ZeroOneOrMany::None → ::Zero")
else:
    print("   ✗ File not found")

# Fix 4: Fix kimi_k2 and qwen3_coder parameter types
print("\n[4/10] Fixing from_downloaded_model parameter types...")
for provider_file in [BASE_DIR / "providers/kimi_k2.rs", BASE_DIR / "providers/qwen3_coder.rs"]:
    if provider_file.exists():
        with open(provider_file, 'r') as f:
            content = f.read()
        
        # Change Vec<progresshub::DownloadResult> to progresshub::types::OneOrMany
        content = re.sub(
            r'results:\s*Vec<progresshub::DownloadResult>',
            r'results: progresshub::types::OneOrMany<progresshub::DownloadResult>',
            content
        )
        
        with open(provider_file, 'w') as f:
            f.write(content)
        print(f"   ✓ Fixed parameter type in {provider_file.name}")
    else:
        print(f"   ✗ {provider_file.name} not found")

# Fix 5: Remove unused imports
print("\n[5/10] Removing unused imports...")
# domain/agent/role.rs - remove CandleQwen3CoderProvider
role_file = BASE_DIR / "domain/agent/role.rs"
if role_file.exists():
    with open(role_file, 'r') as f:
        content = f.read()
    
    # Remove CandleQwen3CoderProvider from import
    content = re.sub(
        r'use crate::providers::\{CandleKimiK2Provider,\s*CandleQwen3CoderProvider\};',
        r'use crate::providers::CandleKimiK2Provider;',
        content
    )
    
    with open(role_file, 'w') as f:
        f.write(content)
    print("   ✓ Removed unused CandleQwen3CoderProvider import")
else:
    print("   ✗ File not found")

# embedding_factory.rs - remove EmbeddingModel
factory_file = BASE_DIR / "memory/vector/embedding_factory.rs"
if factory_file.exists():
    with open(factory_file, 'r') as f:
        content = f.read()
    
    # Remove EmbeddingModel from import
    content = re.sub(
        r'use crate::memory::vector::embedding_model::\{EmbeddingModel,\s*AnyEmbeddingModel\};',
        r'use crate::memory::vector::embedding_model::AnyEmbeddingModel;',
        content
    )
    
    with open(factory_file, 'w') as f:
        f.write(content)
    print("   ✓ Removed unused EmbeddingModel import")
else:
    print("   ✗ File not found")

# Fix 6: Fix unused variable in generator.rs
print("\n[6/10] Fixing unused variable warning in generator.rs...")
generator_file = BASE_DIR / "core/generation/generator.rs"
if generator_file.exists():
    with open(generator_file, 'r') as f:
        content = f.read()
    
    # Prefix sender with underscore
    content = re.sub(
        r'return AsyncStream::with_channel\(move \|sender\|',
        r'return AsyncStream::with_channel(move |_sender|',
        content
    )
    
    with open(generator_file, 'w') as f:
        f.write(content)
    print("   ✓ Prefixed unused sender with underscore")
else:
    print("   ✗ File not found")

print("\n" + "=" * 80)
print("PHASE 1 COMPLETE - Simple fixes applied")
print("=" * 80)
print("\nNext: Run cargo check to see remaining errors")