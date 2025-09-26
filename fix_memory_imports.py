#!/usr/bin/env python3
"""
Automated import refactoring script for memory package migration to candle.

This script handles:
1. Adding memory module declaration to lib.rs
2. Transforming external paraphym_memory imports to paraphym_candle::memory
3. Transforming internal crate:: imports in memory files to crate::memory::
4. Handling complex multiline imports and mixed imports
"""

import os
import re
from pathlib import Path
from typing import List, Tuple

def is_memory_file(file_path: Path, candle_src: Path) -> bool:
    """Check if a file is part of the migrated memory code."""
    try:
        relative_path = file_path.relative_to(candle_src)
        return str(relative_path).startswith('memory/')
    except ValueError:
        return False

def add_memory_module_to_lib(lib_rs_path: Path) -> bool:
    """Add 'pub mod memory;' to lib.rs if not already present."""
    if not lib_rs_path.exists():
        print(f"Warning: {lib_rs_path} not found")
        return False
    
    content = lib_rs_path.read_text()
    
    # Check if memory module is already declared
    if re.search(r'^\s*pub\s+mod\s+memory\s*;', content, re.MULTILINE):
        print("Memory module already declared in lib.rs")
        return False
    
    # Find a good place to insert the memory module declaration
    # Look for other pub mod declarations
    pub_mod_pattern = r'(^\s*pub\s+mod\s+\w+\s*;)'
    matches = list(re.finditer(pub_mod_pattern, content, re.MULTILINE))
    
    if matches:
        # Insert after the last pub mod declaration
        last_match = matches[-1]
        insert_pos = last_match.end()
        new_content = (
            content[:insert_pos] + 
            '\n/// Memory system with cognitive features and vector storage\npub mod memory;' +
            content[insert_pos:]
        )
    else:
        # Insert after the initial comments and before first pub declaration
        # Find the end of the initial comment block
        lines = content.split('\n')
        insert_line = 0
        
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('//') and not line.strip().startswith('#!'):
                insert_line = i
                break
        
        lines.insert(insert_line, '/// Memory system with cognitive features and vector storage')
        lines.insert(insert_line + 1, 'pub mod memory;')
        lines.insert(insert_line + 2, '')
        new_content = '\n'.join(lines)
    
    lib_rs_path.write_text(new_content)
    print("Added memory module declaration to lib.rs")
    return True

def transform_external_imports(content: str) -> str:
    """Transform paraphym_memory:: imports to paraphym_candle::memory::"""
    
    # Pattern to match use paraphym_memory:: statements
    pattern = r'use\s+paraphym_memory::'
    replacement = r'use paraphym_candle::memory::'
    
    new_content = re.sub(pattern, replacement, content)
    
    # Count changes for reporting
    changes = len(re.findall(pattern, content))
    if changes > 0:
        print(f"  → Transformed {changes} external paraphym_memory imports")
    
    return new_content

def transform_internal_memory_imports(content: str) -> str:
    """Transform crate:: imports to crate::memory:: in memory files."""
    
    # This is more complex because we need to handle various patterns:
    # 1. Simple: use crate::cognitive::types;
    # 2. Complex: use crate::{Error, memory::manager::SurrealDBMemoryManager};
    # 3. Multiline imports
    
    changes = 0
    
    # Pattern 1: Simple crate:: imports that don't already have memory::
    # use crate::cognitive:: -> use crate::memory::cognitive::
    pattern1 = r'use\s+crate::(?!memory::)(\w+(?:::\w+)*)'
    def replace1(match):
        nonlocal changes
        changes += 1
        return f'use crate::memory::{match.group(1)}'
    
    new_content = re.sub(pattern1, replace1, content)
    
    # Pattern 2: Complex imports with braces - use crate::{...}
    # This is trickier because we need to parse the contents
    pattern2 = r'use\s+crate::\{([^}]+)\}'
    def replace2(match):
        nonlocal changes
        items = match.group(1)
        
        # Split items and process each one
        item_list = [item.strip() for item in items.split(',')]
        new_items = []
        
        for item in item_list:
            item = item.strip()
            if not item:
                continue
                
            # If item doesn't start with memory::, add memory:: prefix
            if not item.startswith('memory::') and not item in ['Error', 'Result']:
                # Special handling for root-level items like Error
                if '::' not in item or item.startswith('memory'):
                    new_items.append(f'memory::{item}')
                else:
                    new_items.append(f'memory::{item}')
            else:
                new_items.append(item)
        
        if new_items != item_list:
            changes += 1
        
        return f'use crate::{{{", ".join(new_items)}}}'
    
    new_content = re.sub(pattern2, replace2, new_content)
    
    if changes > 0:
        print(f"  → Transformed {changes} internal crate:: imports")
    
    return new_content

def process_rust_file(file_path: Path, candle_src: Path) -> bool:
    """Process a single Rust file for import transformations."""
    
    try:
        content = file_path.read_text()
        original_content = content
        
        # Always transform external paraphym_memory imports
        content = transform_external_imports(content)
        
        # Only transform internal crate:: imports if this is a memory file
        if is_memory_file(file_path, candle_src):
            content = transform_internal_memory_imports(content)
        
        # Write back if changed
        if content != original_content:
            file_path.write_text(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Main function to run the import refactoring."""
    
    # Determine paths
    script_dir = Path(__file__).parent
    candle_src = script_dir / 'packages' / 'candle' / 'src'
    lib_rs = candle_src / 'lib.rs'
    
    if not candle_src.exists():
        print(f"Error: Candle src directory not found at {candle_src}")
        return
    
    print("🔄 Starting memory import refactoring...")
    print(f"📁 Working directory: {candle_src}")
    
    # Step 1: Add memory module to lib.rs
    print("\n1️⃣ Adding memory module declaration to lib.rs...")
    add_memory_module_to_lib(lib_rs)
    
    # Step 2: Process all Rust files
    print("\n2️⃣ Processing Rust files...")
    
    rust_files = list(candle_src.rglob('*.rs'))
    processed_files = 0
    changed_files = 0
    
    for rust_file in rust_files:
        relative_path = rust_file.relative_to(candle_src)
        print(f"📄 Processing: {relative_path}")
        
        if process_rust_file(rust_file, candle_src):
            changed_files += 1
        
        processed_files += 1
    
    print(f"\n✅ Refactoring complete!")
    print(f"📊 Processed {processed_files} files")
    print(f"🔧 Modified {changed_files} files")
    print(f"\n🎯 Next steps:")
    print(f"   1. Run 'cargo check' to verify imports")
    print(f"   2. Fix any remaining import issues manually")
    print(f"   3. Update any re-exports in lib.rs as needed")

if __name__ == '__main__':
    main()
