# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-proc-macros/src/lib.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-proc-macros
- **File Hash**: 355b2faa  
- **Timestamp**: 2025-10-10T02:15:58.912338+00:00  
- **Lines of Code**: 105

---## Panic-Prone Code


### Line 22: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // Determine output function name
    let fn_name = iter.next().unwrap();

    // Separator between name and body with state changes
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 61: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
fn state_entry_stream(iter: &mut Peekable<token_stream::IntoIter>) -> TokenStream {
    // Origin state name
    let state = iter.next().unwrap();

    // Token stream with all the byte->target mappings
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 122: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
/// Get next target state and action.
fn target_change(iter: &mut Peekable<token_stream::IntoIter>) -> (TokenTree, TokenTree) {
    let target_state = iter.next().unwrap();

    // Separator between state and action
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 127: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    expect_punct(iter, ',');

    let target_action = iter.next().unwrap();

    (target_state, target_action)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 163: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let literal = literal.to_string();
            if let Some(prefix) = literal.strip_prefix("0x") {
                usize::from_str_radix(prefix, 16).unwrap()
            } else {
                literal.parse::<usize>().unwrap()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 165: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                usize::from_str_radix(prefix, 16).unwrap()
            } else {
                literal.parse::<usize>().unwrap()
            }
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `generate_state_changes()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-proc-macros/src/lib.rs` (line 16)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Create a `const fn` which will return an array with all state changes.
#[proc_macro]
pub fn generate_state_changes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Convert from proc_macro -> proc_macro2
    let item: TokenStream = item.into();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym