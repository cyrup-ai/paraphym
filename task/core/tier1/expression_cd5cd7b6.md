# `forks/surrealdb/crates/core/src/sql/expression.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: cd5cd7b6  
- **Timestamp**: 2025-10-10T02:16:00.677660+00:00  
- **Lines of Code**: 329

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 329 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 27
  - TODO
  - 

```rust
	Table(Ident),
	Mock(Mock),
	// TODO(3.0) maybe unbox? check size.
	Block(Box<Block>),
	Constant(Constant),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 43
  - TODO
  - 

```rust
		right: Box<Expr>,
	},
	// TODO: Factor out the call from the function expression.
	FunctionCall(Box<FunctionCall>),
	Closure(Box<Closure>),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 78
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

## Panic-Prone Code


### Line 79: `.unwrap()`

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


### Line 80: `.unwrap()`

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


### Line 82: `.unwrap()`

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

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym