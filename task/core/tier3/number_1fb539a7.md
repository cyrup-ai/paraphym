# `forks/surrealdb/crates/core/src/syn/lexer/compound/number.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 1fb539a7  
- **Timestamp**: 2025-10-10T02:16:00.676511+00:00  
- **Lines of Code**: 376

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 376 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 160: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	if has_ident_after(lexer) {
		let char = lexer.reader.next().unwrap();
		let char = lexer.reader.convert_to_char(char)?;
		bail!("Invalid token, found unexpected character `{char}` after number token", @lexer.current_span())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 215: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	if has_ident_after(lexer) {
		let char = lexer.reader.next().unwrap();
		let char = lexer.reader.convert_to_char(char)?;
		bail!("Invalid token, found unexpected character `{char} after integer token", @lexer.current_span())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 276: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	if has_ident_after(lexer) {
		let char = lexer.reader.next().unwrap();
		let char = lexer.reader.convert_to_char(char)?;
		bail!("Invalid token, found invalid character `{char}` after number token", @lexer.current_span())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 398: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	if has_ident_after(lexer) {
		let char = lexer.reader.next().unwrap();
		let char = lexer.reader.convert_to_char(char)?;
		bail!("Invalid token, found invalid character `{char}` after duration suffix", @lexer.current_span())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 151: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	if !lexer.eat(b'f') {
		if lexer.eat(b'd') {
			lexer.expect('e')?;
			lexer.expect('c')?;
			kind = NumberKind::Decimal;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 152: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		if lexer.eat(b'd') {
			lexer.expect('e')?;
			lexer.expect('c')?;
			kind = NumberKind::Decimal;
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 360: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	let suffix = match lexer.reader.next() {
		Some(b'n') => {
			lexer.expect('s')?;
			DurationSuffix::Nano
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 364: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		}
		Some(b'u') => {
			lexer.expect('s')?;
			DurationSuffix::Micro
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 385: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
				bail!("Invalid duration token, expected a duration suffix found `{char}`",@lexer.current_span())
			}
			lexer.expect('s')?;
			DurationSuffix::Micro
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


### `numeric()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/lexer/compound/number.rs` (line 97)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Tokens which can start with digits: Number or Duration.
pub fn numeric(lexer: &mut Lexer, start: Token) -> Result<Numeric, SyntaxError> {
	match start.kind {
		t!("-") | t!("+") => number(lexer, start),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `numeric_kind()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/lexer/compound/number.rs` (line 61)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Tokens which can start with digits: Number or Duration.
/// Like numeric but holds off on parsing the a number into a specific value.
pub fn numeric_kind(lexer: &mut Lexer, start: Token) -> Result<NumericKind, SyntaxError> {
	match start.kind {
		t!("-") | t!("+") => match number_kind(lexer, start)? {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `float()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/lexer/compound/number.rs` (line 244)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Generic integer parsing method,
/// works for all unsigned integers.
pub fn float<I>(lexer: &mut Lexer, start: Token) -> Result<I, SyntaxError>
where
	I: FromStr<Err = ParseFloatError>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `integer()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/lexer/compound/number.rs` (line 201)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Generic integer parsing method,
/// works for all unsigned integers.
pub fn integer<I>(lexer: &mut Lexer, start: Token) -> Result<I, SyntaxError>
where
	I: FromStr<Err = ParseIntError>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym