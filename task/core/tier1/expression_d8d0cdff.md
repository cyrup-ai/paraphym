# `forks/surrealdb/crates/core/src/expr/expression.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: d8d0cdff  
- **Timestamp**: 2025-10-10T02:16:00.661446+00:00  
- **Lines of Code**: 699

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 699 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 53
  - TODO
  - 

```rust
		right: Box<Expr>,
	},
	// TODO: Factor out the call from the function expression.
	FunctionCall(Box<FunctionCall>),

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 153
  - TODO
  - 

```rust
				// replicating previous behavior.
				//
				// TODO: Fix this discrepency and weird static/non-static behavior.
				x.arguments.iter().all(|x| x.is_static())
			}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 192
  - TODO
  - 

```rust
			Expr::Literal(l) => match l {
				Literal::Strand(s) => Idiom::field(Ident::from_strand(s.clone())),
				// TODO: Null byte validity
				Literal::Datetime(d) => Idiom::field(Ident::new(d.into_raw_string()).unwrap()),
				x => Idiom::field(Ident::new(x.to_string()).unwrap()),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 345
  - TODO
  - 

```rust
			}
			Expr::Let(_) => {
				//TODO: This error needs to be improved or it needs to be rejected in the
				// parser.
				Err(ControlFlow::Err(anyhow::Error::new(Error::unreachable(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 445
  - TODO
  - 

```rust
		expr: &Expr,
	) -> FlowResult<Value> {
		// TODO: The structure here is somewhat convoluted, because knn needs to have
		// access to the expression itself instead of just the op and left/right
		// expressions we need to pass in the parent expression when encountering a
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 734
  - TODO
  - 

```rust
impl InfoStructure for Expr {
	fn structure(self) -> Value {
		// TODO: null byte validity
		Strand::new(self.to_string()).unwrap().into()
	}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 237
  - mock
  - 

```rust
			Expr::Mock(mock) => {
				// NOTE(value pr): This is a breaking change but makes the most sense without
				// having mock be part of the Value type.
				// Mock is mostly used within `CREATE |thing:20|` to create a bunch of entries
				// at one. Here it behaves similar to `CREATE
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 238
  - Mock
  - 

```rust
				// NOTE(value pr): This is a breaking change but makes the most sense without
				// having mock be part of the Value type.
				// Mock is mostly used within `CREATE |thing:20|` to create a bunch of entries
				// at one. Here it behaves similar to `CREATE
				// ([thing:1,thing:2,thing:3...])` so when we encounted mock outside of
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 240
  - mock
  - 

```rust
				// Mock is mostly used within `CREATE |thing:20|` to create a bunch of entries
				// at one. Here it behaves similar to `CREATE
				// ([thing:1,thing:2,thing:3...])` so when we encounted mock outside of
				// create we return the array here instead.
				//
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 193: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				Literal::Strand(s) => Idiom::field(Ident::from_strand(s.clone())),
				// TODO: Null byte validity
				Literal::Datetime(d) => Idiom::field(Ident::new(d.into_raw_string()).unwrap()),
				x => Idiom::field(Ident::new(x.to_string()).unwrap()),
			},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 194: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				// TODO: Null byte validity
				Literal::Datetime(d) => Idiom::field(Ident::new(d.into_raw_string()).unwrap()),
				x => Idiom::field(Ident::new(x.to_string()).unwrap()),
			},
			x => Idiom::field(Ident::new(x.to_string()).unwrap()),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 196: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				x => Idiom::field(Ident::new(x.to_string()).unwrap()),
			},
			x => Idiom::field(Ident::new(x.to_string()).unwrap()),
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


### Line 735: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn structure(self) -> Value {
		// TODO: null byte validity
		Strand::new(self.to_string()).unwrap().into()
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

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym