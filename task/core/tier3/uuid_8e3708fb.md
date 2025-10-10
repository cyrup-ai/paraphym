# `forks/surrealdb/crates/core/src/syn/lexer/compound/uuid.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 8e3708fb  
- **Timestamp**: 2025-10-10T02:16:00.723584+00:00  
- **Lines of Code**: 60

---## Panic-Prone Code


### Line 78: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	let Some(res) = ascii_to_hex(peek) else {
		lexer.advance_span();
		let char = lexer.reader.next().unwrap();
		let char = lexer.reader.convert_to_char(char)?;
		bail!("Unexpected character `{char}` expected hexidecimal digit",@lexer.current_span());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 19: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	eat_uuid_hex(lexer, &mut uuid_buffer[0..4])?;

	lexer.expect('-')?;

	eat_uuid_hex(lexer, &mut uuid_buffer[4..6])?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 23: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	eat_uuid_hex(lexer, &mut uuid_buffer[4..6])?;

	lexer.expect('-')?;

	eat_uuid_hex(lexer, &mut uuid_buffer[6..8])?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 27: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	eat_uuid_hex(lexer, &mut uuid_buffer[6..8])?;

	lexer.expect('-')?;

	eat_uuid_hex(lexer, &mut uuid_buffer[8..10])?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 31: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	eat_uuid_hex(lexer, &mut uuid_buffer[8..10])?;

	lexer.expect('-')?;

	eat_uuid_hex(lexer, &mut uuid_buffer[10..16])?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 36: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust

	if double {
		lexer.expect('"')?;
	} else {
		lexer.expect('\'')?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 38: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		lexer.expect('"')?;
	} else {
		lexer.expect('\'')?;
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym