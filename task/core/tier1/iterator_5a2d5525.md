# `forks/surrealdb/crates/core/src/dbs/iterator.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 5a2d5525  
- **Timestamp**: 2025-10-10T02:16:00.657449+00:00  
- **Lines of Code**: 704

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 704 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 179
  - TODO
  - 

```rust
			}
			Expr::Idiom(x) => {
				// TODO: This needs to be structured better.
				// match against what previously would be an edge.
				if x.len() != 2 {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 202
  - TODO
  - 

```rust
					&& lookup.expr.is_none()
				{
					// TODO: Do we support `RETURN a:b` here? What do we do when it is not of the
					// right type?
					let from = match from.compute(stk, ctx, opt, doc).await {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 473
  - TODO
  - 

```rust
						&& lookup.expr.is_none()
					{
						// TODO: Do we support `RETURN a:b` here? What do we do when it is not of
						// the right type?
						let from = match from.compute(stk, ctx, opt, doc).await {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 174
  - stubby method name
  - prepare_mock

```rust
		// Match the values
		match val {
			Expr::Mock(v) => self.prepare_mock(stm_ctx, v).await?,
			Expr::Table(v) => {
				self.prepare_table(ctx, opt, stk, planner, stm_ctx, v.clone()).await?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 282
  - stubby method name
  - prepare_mock

```rust

	/// Prepares a value for processing
	pub(crate) async fn prepare_mock(
		&mut self,
		ctx: &StatementContext<'_>,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 440
  - stubby method name
  - prepare_mock

```rust
		for v in v {
			match v {
				Expr::Mock(v) => self.prepare_mock(stm_ctx, v).await?,
				Expr::Table(v) => {
					self.prepare_table(ctx, opt, stk, planner, stm_ctx, v.clone()).await?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 174
  - stubby variable name
  - prepare_mock

```rust
		// Match the values
		match val {
			Expr::Mock(v) => self.prepare_mock(stm_ctx, v).await?,
			Expr::Table(v) => {
				self.prepare_table(ctx, opt, stk, planner, stm_ctx, v.clone()).await?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 282
  - stubby variable name
  - prepare_mock

```rust

	/// Prepares a value for processing
	pub(crate) async fn prepare_mock(
		&mut self,
		ctx: &StatementContext<'_>,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 440
  - stubby variable name
  - prepare_mock

```rust
		for v in v {
			match v {
				Expr::Mock(v) => self.prepare_mock(stm_ctx, v).await?,
				Expr::Table(v) => {
					self.prepare_table(ctx, opt, stk, planner, stm_ctx, v.clone()).await?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 907
  - MOCK
  - 

```rust
		for (count, v) in mem::take(&mut self.entries).into_iter().enumerate() {
			v.iterate(stk, ctx, &opt, stm, self, distinct.as_mut()).await?;
			// MOCK can create a large collection of iterators,
			// we need to make space for possible cancellations
			if ctx.is_done(count % 100 == 0).await? {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 34
  - actual
  - 

```rust
	/// SurrealQL value, object, or array of values.
	Value(Value),
	/// An iterable which does not actually fetch the record
	/// data from storage. This is used in CREATE statements
	/// where we attempt to write data without first checking
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 529
  - actual
  - 

```rust
		// without FULL
		let mut plan = Plan::new(ctx, stm, &self.entries, &self.results);
		// Check if we actually need to process and iterate over the results
		if plan.do_iterate {
			if let Some(e) = &mut plan.explanation {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym