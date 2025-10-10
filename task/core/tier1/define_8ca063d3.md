# `forks/surrealdb/crates/core/src/syn/parser/stmt/define.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 8ca063d3  
- **Timestamp**: 2025-10-10T02:16:00.650379+00:00  
- **Lines of Code**: 1550

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1550 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 215
  - TODO
  - 

```rust
			                                                                    * the viewer role
			                                                                    * by default */
			// TODO: Move out of the parser
			token_duration: Some(Duration::from_secs(3600)), // defaults to 1 hour.
			..DefineUserStatement::default()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 325
  - TODO
  - 

```rust
		let name = self.next_token_value()?;
		expected!(self, t!("ON"));
		// TODO: Parse base should no longer take an argument.
		let base = self.parse_base()?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 389
  - TODO
  - 

```rust
									}
									t!("REFRESH") => {
										// TODO(gguillemas): Remove this once bearer access is no
										// longer experimental.
										if !self.settings.bearer_access_enabled {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 420
  - TODO
  - 

```rust
						}
						t!("BEARER") => {
							// TODO(gguillemas): Remove this once bearer access is no longer
							// experimental.
							if !self.settings.bearer_access_enabled {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 615
  - TODO
  - 

```rust
					self.pop_peek();
					res.full = true;
					// TODO: Move logic out of parser.
					if !set_table_type {
						res.table_type = TableType::Normal;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1474
  - stubby variable name
  - tmp_tables

```rust
	fn parse_graphql_config(&mut self) -> ParseResult<GraphQLConfig> {
		use graphql::{FunctionsConfig, TablesConfig};
		let mut tmp_tables = Option::<TablesConfig>::None;
		let mut tmp_fncs = Option::<FunctionsConfig>::None;
		loop {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1475
  - stubby variable name
  - tmp_fncs

```rust
		use graphql::{FunctionsConfig, TablesConfig};
		let mut tmp_tables = Option::<TablesConfig>::None;
		let mut tmp_fncs = Option::<FunctionsConfig>::None;
		loop {
			match self.peek_kind() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1480
  - stubby variable name
  - tmp_tables

```rust
				t!("NONE") => {
					self.pop_peek();
					tmp_tables = Some(TablesConfig::None);
					tmp_fncs = Some(FunctionsConfig::None);
				}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1481
  - stubby variable name
  - tmp_fncs

```rust
					self.pop_peek();
					tmp_tables = Some(TablesConfig::None);
					tmp_fncs = Some(FunctionsConfig::None);
				}
				t!("AUTO") => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1485
  - stubby variable name
  - tmp_tables

```rust
				t!("AUTO") => {
					self.pop_peek();
					tmp_tables = Some(TablesConfig::Auto);
					tmp_fncs = Some(FunctionsConfig::Auto);
				}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1486
  - stubby variable name
  - tmp_fncs

```rust
					self.pop_peek();
					tmp_tables = Some(TablesConfig::Auto);
					tmp_fncs = Some(FunctionsConfig::Auto);
				}
				t!("TABLES") => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1494
  - stubby variable name
  - tmp_tables

```rust
					match next.kind {
						t!("INCLUDE") => {
							tmp_tables =
								Some(TablesConfig::Include(self.parse_graphql_table_configs()?))
						}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1498
  - stubby variable name
  - tmp_tables

```rust
						}
						t!("EXCLUDE") => {
							tmp_tables =
								Some(TablesConfig::Include(self.parse_graphql_table_configs()?))
						}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1502
  - stubby variable name
  - tmp_tables

```rust
						}
						t!("NONE") => {
							tmp_tables = Some(TablesConfig::None);
						}
						t!("AUTO") => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1505
  - stubby variable name
  - tmp_tables

```rust
						}
						t!("AUTO") => {
							tmp_tables = Some(TablesConfig::Auto);
						}
						_ => unexpected!(self, next, "`NONE`, `AUTO`, `INCLUDE` or `EXCLUDE`"),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1518
  - stubby variable name
  - tmp_fncs

```rust
						t!("EXCLUDE") => {}
						t!("NONE") => {
							tmp_fncs = Some(FunctionsConfig::None);
						}
						t!("AUTO") => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1521
  - stubby variable name
  - tmp_fncs

```rust
						}
						t!("AUTO") => {
							tmp_fncs = Some(FunctionsConfig::Auto);
						}
						_ => unexpected!(self, next, "`NONE`, `AUTO`, `INCLUDE` or `EXCLUDE`"),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1531
  - stubby variable name
  - tmp_tables

```rust

		Ok(GraphQLConfig {
			tables: tmp_tables.unwrap_or_default(),
			functions: tmp_fncs.unwrap_or_default(),
		})
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1532
  - stubby variable name
  - tmp_fncs

```rust
		Ok(GraphQLConfig {
			tables: tmp_tables.unwrap_or_default(),
			functions: tmp_fncs.unwrap_or_default(),
		})
	}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym