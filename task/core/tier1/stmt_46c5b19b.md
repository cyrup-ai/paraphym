# `forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 46c5b19b  
- **Timestamp**: 2025-10-10T02:16:00.646577+00:00  
- **Lines of Code**: 2911

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 2911 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 1678
  - TODO
  - 

```rust
	// Invalid DELETE permission
	{
		// TODO(gguillemas): Providing the DELETE permission should return a parse error
		// in 3.0.0. Currently, the DELETE permission is just ignored to maintain
		// backward compatibility.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 702
  - hardcoded URL
  - 

```rust
	// With comment. Verify only.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" COMMENT "bar""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 711
  - hardcoded URL
  - 

```rust
				access_type: AccessType::Jwt(JwtAccess {
					verify: JwtAccessVerify::Jwks(JwtAccessVerifyJwks {
						url: "http://example.com/.well-known/jwks.json".to_string(),
					}),
					issue: None,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 728
  - hardcoded URL
  - 

```rust
	// Verify and symmetric issuer.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM HS384 KEY "foo""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 737
  - hardcoded URL
  - 

```rust
				access_type: AccessType::Jwt(JwtAccess {
					verify: JwtAccessVerify::Jwks(JwtAccessVerifyJwks {
						url: "http://example.com/.well-known/jwks.json".to_string(),
					}),
					issue: Some(JwtAccessIssue {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 757
  - hardcoded URL
  - 

```rust
	// Verify and symmetric issuer with custom duration.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM HS384 KEY "foo" DURATION FOR TOKEN 10s"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 766
  - hardcoded URL
  - 

```rust
				access_type: AccessType::Jwt(JwtAccess {
					verify: JwtAccessVerify::Jwks(JwtAccessVerifyJwks {
						url: "http://example.com/.well-known/jwks.json".to_string(),
					}),
					issue: Some(JwtAccessIssue {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 785
  - hardcoded URL
  - 

```rust
	// Verify and asymmetric issuer.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM PS256 KEY "foo""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 794
  - hardcoded URL
  - 

```rust
				access_type: AccessType::Jwt(JwtAccess {
					verify: JwtAccessVerify::Jwks(JwtAccessVerifyJwks {
						url: "http://example.com/.well-known/jwks.json".to_string(),
					}),
					issue: Some(JwtAccessIssue {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 814
  - hardcoded URL
  - 

```rust
	// Verify and asymmetric issuer with custom duration.
	{
		let res = syn::parse_with(r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM PS256 KEY "foo" DURATION FOR TOKEN 10s, FOR SESSION 2d"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 823
  - hardcoded URL
  - 

```rust
				access_type: AccessType::Jwt(JwtAccess {
					verify: JwtAccessVerify::Jwks(JwtAccessVerifyJwks {
						url: "http://example.com/.well-known/jwks.json".to_string(),
					}),
					issue: Some(JwtAccessIssue {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 1680
  - backward compatibility
  - 

```rust
		// TODO(gguillemas): Providing the DELETE permission should return a parse error
		// in 3.0.0. Currently, the DELETE permission is just ignored to maintain
		// backward compatibility.
		let res = syn::parse_with(
			r#"DEFINE FIELD foo ON TABLE bar PERMISSIONS FOR DELETE NONE"#.as_bytes(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 56: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap();
	let res = res.expressions.pop().unwrap();
	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 57: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	})
	.unwrap();
	let res = res.expressions.pop().unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 74: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			.expressions
			.pop()
			.unwrap();
	assert_eq!(res, TopLevelExpr::Begin);
	let res = syn::parse_with(r#"BEGIN TRANSACTION;"#.as_bytes(), async |parser, stk| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 71: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	let res =
		syn::parse_with(r#"BEGIN;"#.as_bytes(), async |parser, stk| parser.parse_query(stk).await)
			.unwrap()
			.expressions
			.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 82: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	assert_eq!(res, TopLevelExpr::Begin);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 79: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 91: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(res, Expr::Break)
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 100: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_top_level_expr(stk).await
	})
	.unwrap();
	assert_eq!(res, TopLevelExpr::Cancel);
	let res = syn::parse_with(r#"CANCEL TRANSACTION"#.as_bytes(), async |parser, stk| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 105: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_top_level_expr(stk).await
	})
	.unwrap();
	assert_eq!(res, TopLevelExpr::Cancel);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_top_level_expr(stk).await
	})
	.unwrap();
	assert_eq!(res, TopLevelExpr::Commit);
	let res = syn::parse_with(r#"COMMIT TRANSACTION"#.as_bytes(), async |parser, stk| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 119: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_top_level_expr(stk).await
	})
	.unwrap();
	assert_eq!(res, TopLevelExpr::Commit);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 128: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(res, Expr::Continue);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 139: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 175: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			parser.parse_expr_inherit(stk).await
		})
		.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 189: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 207: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 225: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 248: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 282: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();

		let Expr::Define(res) = res else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 306: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();

		let Expr::Define(res) = res else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 330: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();

		let Expr::Define(res) = res else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 359: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();

		let Expr::Define(res) = res else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 381: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();

		let Expr::Define(res) = res else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 434: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 461: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Asymmetric verify and issue.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT ALGORITHM EDDSA KEY "foo" WITH ISSUER KEY "bar""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 491: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Asymmetric verify and issue with authenticate clause.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT ALGORITHM EDDSA KEY "foo" WITH ISSUER KEY "bar" AUTHENTICATE true"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 525: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 555: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Symmetric verify and explicit duration.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT ALGORITHM HS256 KEY "foo" DURATION FOR TOKEN 10s"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 584: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Symmetric verify and explicit issue matching data.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT ALGORITHM HS256 KEY "foo" WITH ISSUER ALGORITHM HS256 KEY "foo""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 639: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 671: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 702: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// With comment. Verify only.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" COMMENT "bar""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 728: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Verify and symmetric issuer.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM HS384 KEY "foo""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 757: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Verify and symmetric issuer with custom duration.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM HS384 KEY "foo" DURATION FOR TOKEN 10s"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 785: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Verify and asymmetric issuer.
	{
		let res = syn::parse_with( r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM PS256 KEY "foo""#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 814: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Verify and asymmetric issuer with custom duration.
	{
		let res = syn::parse_with(r#"DEFINE ACCESS a ON DATABASE TYPE JWT URL "http://example.com/.well-known/jwks.json" WITH ISSUER ALGORITHM PS256 KEY "foo" DURATION FOR TOKEN 10s, FOR SESSION 2d"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 850: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();

		// Manually compare since DefineAccessStatement for record access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 905: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |p, s| p.parse_expr_inherit(s).await,
		)
		.unwrap();

		// Manually compare since DefineAccessStatement for record access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 975: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Session duration, signing and authenticate clauses are explicitly defined.
	{
		let res = syn::parse_with(r#"DEFINE ACCESS a ON DB TYPE RECORD SIGNUP true SIGNIN false AUTHENTICATE true DURATION FOR SESSION 7d"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();

		// Manually compare since DefineAccessStatement for record access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1021: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Verification with JWT is explicitly defined only with symmetric key.
	{
		let res = syn::parse_with(r#"DEFINE ACCESS a ON DB TYPE RECORD WITH JWT ALGORITHM HS384 KEY "foo" DURATION FOR TOKEN 10s, FOR SESSION 15m"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1058: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// keys.
	{
		let res = syn::parse_with(r#"DEFINE ACCESS a ON DB TYPE RECORD WITH JWT ALGORITHM PS512 KEY "foo" WITH ISSUER KEY "bar" DURATION FOR TOKEN 10s, FOR SESSION 15m"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1101: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |p,s| p.parse_expr_inherit(s).await,
		)
			.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1156: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			},
			async |p,s| p.parse_expr_inherit(s).await,
		).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1204: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// keys. Token duration is explicitly defined.
	{
		let res = syn::parse_with(r#"DEFINE ACCESS a ON DB TYPE RECORD WITH JWT ALGORITHM RS256 KEY 'foo' WITH ISSUER KEY 'bar' DURATION FOR TOKEN 10s, FOR SESSION 15m"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1275: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |p, s| p.parse_expr_inherit(s).await,
		)
		.unwrap();

		// Manually compare since DefineAccessStatement for bearer access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1317: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |p, s| p.parse_expr_inherit(s).await,
		)
		.unwrap();

		// Manually compare since DefineAccessStatement for bearer access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1359: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |p, s| p.parse_expr_inherit(s).await,
		)
		.unwrap();

		// Manually compare since DefineAccessStatement for bearer access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1401: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |p, s| p.parse_expr_inherit(s).await,
		)
		.unwrap();

		// Manually compare since DefineAccessStatement for bearer access
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1465: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			},
			async |p,s| p.parse_expr_inherit(s).await,
		).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1507: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			},
			async |p,s| p.parse_expr_inherit(s).await,
		).unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1548: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1574: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
fn parse_define_table() {
	let res =
		syn::parse_with(r#"DEFINE TABLE name DROP SCHEMAFUL CHANGEFEED 1s INCLUDE ORIGINAL PERMISSIONS FOR DELETE FULL, FOR SELECT WHERE a = 1 AS SELECT foo FROM bar GROUP BY foo"#.as_bytes(),async |parser,stk| parser.parse_expr_inherit(stk).await).unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1622: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1641: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// General
	{
		let res = syn::parse_with(r#"DEFINE FIELD foo.*[*]... ON TABLE bar FLEX TYPE option<number | array<record<foo>,10>> VALUE null ASSERT true DEFAULT false PERMISSIONS FOR UPDATE NONE, FOR CREATE WHERE true"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();

		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1685: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_expr_inherit(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1729: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1765: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1781: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	let res =
		syn::parse_with( r#"DEFINE INDEX index ON TABLE table FIELDS a MTREE DIMENSION 4 DISTANCE MINKOWSKI 5 CAPACITY 6 TYPE I16 DOC_IDS_ORDER 7 DOC_IDS_CACHE 8 MTREE_CACHE 9"#.as_bytes(),async |parser,stk| parser.parse_expr_inherit(stk).await).unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1805: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	let res =
		syn::parse_with( r#"DEFINE INDEX index ON TABLE table FIELDS a HNSW DIMENSION 128 EFC 250 TYPE F32 DISTANCE MANHATTAN M 6 M0 12 LM 0.5 EXTEND_CANDIDATES KEEP_PRUNED_CONNECTIONS"#.as_bytes(),async |parser,stk| parser.parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1832: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn parse_define_analyzer() {
	let res = syn::parse_with(r#"DEFINE ANALYZER ana FILTERS ASCII, EDGENGRAM(1,2), NGRAM(3,4), LOWERCASE, SNOWBALL(NLD), UPPERCASE TOKENIZERS BLANK, CAMEL, CLASS, PUNCT FUNCTION fn::foo::bar"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1860: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn parse_delete() {
	let res = syn::parse_with("DELETE FROM ONLY |foo:32..64| WITH INDEX index,index_2 Where 2 RETURN AFTER TIMEOUT 1s PARALLEL EXPLAIN FULL".as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1878: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn parse_delete_2() {
	let res = syn::parse_with(r#"DELETE FROM ONLY a:b->?[$][?true] WITH INDEX index,index_2 WHERE null RETURN NULL TIMEOUT 1h PARALLEL EXPLAIN"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1911: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1954: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1973: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1991: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(res, Expr::Info(Box::new(InfoStatement::Root(false))));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1997: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(res, Expr::Info(Box::new(InfoStatement::Root(false))));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2003: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(res, Expr::Info(Box::new(InfoStatement::Ns(false))));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2009: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(res, Expr::Info(Box::new(InfoStatement::Ns(false))));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2015: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2028: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2041: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2071: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |p, s| p.parse_expr_inherit(s).await,
	)
	.unwrap();

	let offset = Utc.fix();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2082: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		)
		.earliest()
		.unwrap()
		.with_timezone(&Utc);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2079: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				.unwrap()
				.and_hms_nano_opt(18, 25, 43, 51_100)
				.unwrap(),
		)
		.earliest()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2077: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		.from_local_datetime(
			&NaiveDate::from_ymd_opt(2012, 4, 23)
				.unwrap()
				.and_hms_nano_opt(18, 25, 43, 51_100)
				.unwrap(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2155: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_top_level_expr(stk).await,
	)
	.unwrap();

	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2175: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		)
		.earliest()
		.unwrap()
		.with_timezone(&Utc);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2172: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				.unwrap()
				.and_hms_nano_opt(18, 25, 43, 51_100)
				.unwrap(),
		)
		.earliest()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2170: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		.from_local_datetime(
			&NaiveDate::from_ymd_opt(2012, 4, 23)
				.unwrap()
				.and_hms_nano_opt(18, 25, 43, 51_100)
				.unwrap(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2185: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2182: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_query(stk).await,
	)
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2201: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();

	let expect = Expr::Sleep(Box::new(SleepStatement {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2217: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	let expect = TopLevelExpr::Use(UseStatement {
		ns: Some(Ident::from_strand(strand!("foo").to_owned())),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2214: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2230: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	let expect = TopLevelExpr::Use(UseStatement {
		ns: Some(Ident::from_strand(strand!("foo").to_owned())),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2227: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2243: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();

	let expect = TopLevelExpr::Use(UseStatement {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2240: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2260: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	let expect = TopLevelExpr::Use(UseStatement {
		ns: Some(Ident::from_strand(strand!("foo").to_owned())),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2257: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2273: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();

	let expect = TopLevelExpr::Use(UseStatement {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2270: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2287: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();

	let expect = TopLevelExpr::Use(UseStatement {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2284: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2300: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	let res =
		syn::parse_with(r"1s".as_bytes(), async |parser, stk| parser.parse_expr_inherit(stk).await)
			.unwrap();
	let expect = Expr::Literal(Literal::Duration(Duration(std::time::Duration::from_secs(1))));
	assert_eq!(res, expect);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2310: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();

	let expect = Expr::Throw(Box::new(Expr::Literal(Literal::Duration(Duration(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2323: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		r#"INSERT IGNORE INTO $foo (a,b,c) VALUES (1,2,3),(4,5,6) ON DUPLICATE KEY UPDATE a.b +?= null, c.d += none RETURN AFTER"#.as_bytes(),
		async |parser,stk| parser.parse_expr_inherit(stk).await
	).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2392: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2440: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2437: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_query(stk).await
	})
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2455: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	.expressions
	.pop()
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2452: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_query(stk).await,
	)
	.unwrap()
	.expressions
	.pop()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2471: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_top_level_expr(stk).await
	})
	.unwrap();
	let TopLevelExpr::Live(stmt) = res else {
		panic!()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2482: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_top_level_expr(stk).await,
	)
	.unwrap();
	let TopLevelExpr::Live(stmt) = res else {
		panic!()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2518: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_top_level_expr(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2533: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2549: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		async |parser, stk| parser.parse_expr_inherit(stk).await,
	)
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2589: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2602: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2615: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2627: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			parser.parse_expr_inherit(stk).await
		})
		.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2640: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			parser.parse_expr_inherit(stk).await
		})
		.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2653: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2665: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2679: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			parser.parse_expr_inherit(stk).await
		})
		.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2693: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			parser.parse_expr_inherit(stk).await
		})
		.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2710: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2723: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2735: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		parser.parse_expr_inherit(stk).await
	})
	.unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2748: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn parse_update() {
	let res = syn::parse_with(r#"UPDATE ONLY a->b WITH INDEX index,index_2 UNSET foo... , a->b, c[*] WHERE true RETURN DIFF TIMEOUT 1s PARALLEL EXPLAIN FULL"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2790: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn parse_upsert() {
	let res = syn::parse_with(r#"UPSERT ONLY a->b WITH INDEX index,index_2 UNSET foo... , a->b, c[*] WHERE true RETURN DIFF TIMEOUT 1s PARALLEL EXPLAIN"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
		res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2842: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2862: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2889: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2910: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2931: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2956: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2977: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2998: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 3023: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 3045: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 3067: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 3089: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 3111: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 3133: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			async |parser, stk| parser.parse_top_level_expr(stk).await,
		)
		.unwrap();
		assert_eq!(
			res,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 52: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 52)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_analyze() {
	let mut res = syn::parse_with(r#"ANALYZE INDEX b on a"#.as_bytes(), async |parser, stk| {
		parser.parse_query(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 68: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 68)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_begin() {
	let res =
		syn::parse_with(r#"BEGIN;"#.as_bytes(), async |parser, stk| parser.parse_query(stk).await)
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 87: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 87)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_break() {
	let res = syn::parse_with(r#"BREAK"#.as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 96: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 96)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_cancel() {
	let res = syn::parse_with(r#"CANCEL"#.as_bytes(), async |parser, stk| {
		parser.parse_top_level_expr(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 110: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 110)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_commit() {
	let res = syn::parse_with(r#"COMMIT"#.as_bytes(), async |parser, stk| {
		parser.parse_top_level_expr(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 124: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 124)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_continue() {
	let res = syn::parse_with(r#"CONTINUE"#.as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 133: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 133)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_create() {
	let res = syn::parse_with(
		"CREATE ONLY foo SET bar = 3, foo +?= baz RETURN VALUE foo AS bar TIMEOUT 1s PARALLEL"
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 170: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 170)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_namespace() {
	let res =
		syn::parse_with("DEFINE NAMESPACE a COMMENT 'test'".as_bytes(), async |parser, stk| {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 202: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 202)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_database() {
	let res = syn::parse_with(
		"DEFINE DATABASE a COMMENT 'test' CHANGEFEED 10m INCLUDE ORIGINAL".as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 239: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 239)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_function() {
	let res = syn::parse_with(
		r#"DEFINE FUNCTION fn::foo::bar($a: number, $b: array<bool,3>) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 274: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 274)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_user() {
	// Password.
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 426: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 426)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_access_jwt_key() {
	// With comment. Asymmetric verify only.
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 699: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 699)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_access_jwt_jwks() {
	// With comment. Verify only.
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 843: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 843)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_access_record() {
	// With comment. Nothing is explicitly defined.
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1264: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1264)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_access_bearer() {
	// For user on database.
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1543: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1543)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_param() {
	let res = syn::parse_with(
		r#"DEFINE PARAM $a VALUE { a: 1, "b": 3 } PERMISSIONS WHERE null"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1572: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1572)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_table() {
	let res =
		syn::parse_with(r#"DEFINE TABLE name DROP SCHEMAFUL CHANGEFEED 1s INCLUDE ORIGINAL PERMISSIONS FOR DELETE FULL, FOR SELECT WHERE a = 1 AS SELECT foo FROM bar GROUP BY foo"#.as_bytes(),async |parser,stk| parser.parse_expr_inherit(stk).await).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1617: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1617)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_event() {
	let res = syn::parse_with(
		r#"DEFINE EVENT event ON TABLE table WHEN null THEN null,none"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1638: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1638)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_field() {
	// General
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1713: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1713)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_index() {
	let res = syn::parse_with(
		r#"DEFINE INDEX index ON TABLE table FIELDS a,b[*] SEARCH ANALYZER ana BM25 (0.1,0.2)
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1831: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1831)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_define_analyzer() {
	let res = syn::parse_with(r#"DEFINE ANALYZER ana FILTERS ASCII, EDGENGRAM(1,2), NGRAM(3,4), LOWERCASE, SNOWBALL(NLD), UPPERCASE TOKENIZERS BLANK, CAMEL, CLASS, PUNCT FUNCTION fn::foo::bar"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1859: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1859)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_delete() {
	let res = syn::parse_with("DELETE FROM ONLY |foo:32..64| WITH INDEX index,index_2 Where 2 RETURN AFTER TIMEOUT 1s PARALLEL EXPLAIN FULL".as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1877: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1877)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_delete_2() {
	let res = syn::parse_with(r#"DELETE FROM ONLY a:b->?[$][?true] WITH INDEX index,index_2 WHERE null RETURN NULL TIMEOUT 1h PARALLEL EXPLAIN"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1906: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1906)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
pub fn parse_for() {
	let res = syn::parse_with(
		r#"FOR $foo IN (SELECT foo FROM bar) * 2 { BREAK }"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1949: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1949)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_if() {
	let res = syn::parse_with(
		r#"IF foo THEN bar ELSE IF faz THEN baz ELSE baq END"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1968: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1968)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_if_block() {
	let res = syn::parse_with(
		r#"IF foo { bar } ELSE IF faz { baz } ELSE { baq }"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1987: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 1987)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_info() {
	let res = syn::parse_with("INFO FOR ROOT".as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2053: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2053)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_select() {
	let res = syn::parse_with(
		r#"
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2150: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2150)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_show() {
	let res = syn::parse_with(
		r#"SHOW CHANGES FOR TABLE foo SINCE 1 LIMIT 10"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2197: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2197)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_sleep() {
	let res = syn::parse_with(r"SLEEP 1s".as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2210: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2210)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_use() {
	let res = syn::parse_with(r"USE NS foo".as_bytes(), async |parser, stk| {
		parser.parse_query(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2253: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2253)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_use_lowercase() {
	let res = syn::parse_with(r"use ns foo".as_bytes(), async |parser, stk| {
		parser.parse_query(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2297: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2297)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_value_stmt() {
	let res =
		syn::parse_with(r"1s".as_bytes(), async |parser, stk| parser.parse_expr_inherit(stk).await)
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2306: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2306)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_throw() {
	let res = syn::parse_with(r"THROW 1s".as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2319: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2319)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_insert() {
	let res = syn::parse_with(
		r#"INSERT IGNORE INTO $foo (a,b,c) VALUES (1,2,3),(4,5,6) ON DUPLICATE KEY UPDATE a.b +?= null, c.d += none RETURN AFTER"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2387: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2387)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_insert_select() {
	let res = syn::parse_with(
		r#"INSERT IGNORE INTO bar (select foo from baz)"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2433: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2433)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_kill() {
	let res = syn::parse_with(r#"KILL $param"#.as_bytes(), async |parser, stk| {
		parser.parse_query(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2467: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2467)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_live() {
	let res = syn::parse_with(r#"LIVE SELECT DIFF FROM $foo"#.as_bytes(), async |parser, stk| {
		parser.parse_top_level_expr(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2514: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2514)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_option() {
	let res = syn::parse_with(r#"OPTION value = true"#.as_bytes(), async |parser, stk| {
		parser.parse_top_level_expr(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2529: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2529)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_return() {
	let res = syn::parse_with(r#"RETURN RETRUN FETCH RETURN"#.as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2544: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2544)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_relate() {
	let res = syn::parse_with(
		r#"RELATE ONLY [1,2]->a:b->(CREATE foo) UNIQUE SET a += 1 RETURN NONE PARALLEL"#.as_bytes(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2585: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2585)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_remove() {
	let res = syn::parse_with(r#"REMOVE NAMESPACE ns"#.as_bytes(), async |parser, stk| {
		parser.parse_expr_inherit(stk).await
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2747: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2747)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_update() {
	let res = syn::parse_with(r#"UPDATE ONLY a->b WITH INDEX index,index_2 UNSET foo... , a->b, c[*] WHERE true RETURN DIFF TIMEOUT 1s PARALLEL EXPLAIN FULL"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2789: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2789)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_upsert() {
	let res = syn::parse_with(r#"UPSERT ONLY a->b WITH INDEX index,index_2 UNSET foo... , a->b, c[*] WHERE true RETURN DIFF TIMEOUT 1s PARALLEL EXPLAIN"#.as_bytes(),async |parser,stk| parser. parse_expr_inherit(stk).await).unwrap();
	assert_eq!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2831: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2831)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_access_grant() {
	// User
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2878: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2878)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_access_show() {
	// All
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2945: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 2945)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_access_revoke() {
	// All
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3012: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 3012)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn parse_access_purge() {
	// All
	{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `ident_field()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/stmt.rs` (line 47)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
use crate::val::{Datetime, Duration, Number, Strand, Uuid};

fn ident_field(name: &str) -> Expr {
	Expr::Idiom(Idiom(vec![Part::Field(Ident::new(name.to_string()).unwrap())]))
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym