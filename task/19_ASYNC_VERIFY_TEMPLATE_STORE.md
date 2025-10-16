# Task: Verify and Convert TemplateStore if Needed

## Location
`packages/candle/src/domain/chat/templates/cache/store.rs` lines 10-45

## Problem (Potential)
```rust
pub trait TemplateStore: Send + Sync {
    fn store(&self, template: &ChatTemplate) -> TemplateResult<()>;
    fn get(&self, name: &str) -> TemplateResult<Option<ChatTemplate>>;
    fn delete(&self, name: &str) -> TemplateResult<bool>;
    fn list(&self) -> TemplateResult<Vec<String>>;
    fn exists(&self, name: &str) -> TemplateResult<bool>;
}
```

**Question**: Do these methods do I/O?
- If in-memory cache only → OK as sync
- If database/file storage → MUST be async

## Investigation Required
1. Find all implementations of `TemplateStore`
2. Check if they do I/O (file system, database, network)
3. If they do I/O, convert to async

## Async Conversion (if needed)
```rust
pub trait TemplateStore: Send + Sync {
    async fn store(&self, template: &ChatTemplate) -> TemplateResult<()>;
    async fn get(&self, name: &str) -> TemplateResult<Option<ChatTemplate>>;
    async fn delete(&self, name: &str) -> TemplateResult<bool>;
    async fn list(&self) -> TemplateResult<Vec<String>>;
    async fn exists(&self, name: &str) -> TemplateResult<bool>;
}
```

## Steps
1. Search for `impl TemplateStore`
2. Check each implementation for I/O operations
3. If I/O found: Convert trait and all impls to async
4. If no I/O: Mark as verified-sync and close task

## Priority
🟡 **MEDIUM** - Needs investigation first

## Status
⏳ TODO - Investigation required
