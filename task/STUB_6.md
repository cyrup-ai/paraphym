# STUB_6: Fix Compilation Errors in Export Implementation

## Status: 4/10 - Code structure is correct but has 4 compilation errors preventing use

## Compilation Errors to Fix

**File:** `packages/candle/src/domain/chat/commands/execution.rs`

### Error 1: Invalid Attribute (Line 49)
```
error: cannot find attribute `debug` in this scope
  --> packages/candle/src/domain/chat/commands/execution.rs:49:7
   |
49 |     #[debug(skip)]
   |       ^^^^^
```

**Fix Required:**
Remove the invalid `#[debug(skip)]` attribute. Either:
- Remove the line entirely, OR
- Use a custom Debug implementation for CommandExecutor instead of deriving it

---

### Error 2: Private Field Access (Line 630)
```
error[E0616]: field `surreal_manager` of struct `MemoryCoordinator` is private
   --> packages/candle/src/domain/chat/commands/execution.rs:630:29
    |
630 |     let mut stream = memory.surreal_manager.list_all_memories(10000, 0);
    |                             ^^^^^^^^^^^^^^^ private field
```

**Fix Required:**
The code tries to access `memory.surreal_manager` which is `pub(super)` (module-private).

**Solution Options:**
1. **Add a public method to MemoryCoordinator** (RECOMMENDED):
   ```rust
   // In packages/candle/src/memory/core/manager/coordinator/operations.rs
   pub fn list_all_memories(&self, limit: u64, offset: u64) 
       -> impl Stream<Item = Result<MemoryNode>> {
       self.surreal_manager.list_all_memories(limit, offset)
   }
   ```

2. **Or expose surreal_manager as public**:
   ```rust
   // In packages/candle/src/memory/core/manager/coordinator/lifecycle.rs:26
   pub surreal_manager: Arc<SurrealDBMemoryManager>,  // Change pub(super) to pub
   ```

---

### Error 3: Private Method Call (Line 644)
```
error[E0624]: method `convert_memory_to_domain_node` is private
   --> packages/candle/src/domain/chat/commands/execution.rs:644:39
    |
644 |         let domain_mem = match memory.convert_memory_to_domain_node(&mem) {
    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ private method
```

**Fix Required:**
The method `convert_memory_to_domain_node` is `pub(super)` in `conversions.rs`.

**Solution Options:**
1. **Make the method public** (RECOMMENDED):
   ```rust
   // In packages/candle/src/memory/core/manager/coordinator/conversions.rs:152
   pub fn convert_memory_to_domain_node(  // Change pub(super) to pub
       &self,
       memory_node: &crate::memory::core::primitives::node::MemoryNode,
   ) -> Result<crate::domain::memory::primitives::node::MemoryNode>
   ```

2. **Or create a wrapper method that returns domain nodes directly**:
   ```rust
   pub fn list_conversation_messages(&self, limit: u64) 
       -> impl Stream<Item = Result<DomainMemoryNode>>
   ```

---

### Error 4: Missing Debug Implementation
```
error[E0277]: `MemoryCoordinator` doesn't implement `std::fmt::Debug`
  --> packages/candle/src/domain/chat/commands/execution.rs:50:5
   |
50 |     memory: Option<Arc<MemoryCoordinator>>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
```

**Fix Required:**
CommandExecutor derives Debug, but MemoryCoordinator doesn't implement it.

**Solution Options:**
1. **Derive Debug for MemoryCoordinator** (EASIEST):
   ```rust
   // In packages/candle/src/memory/core/manager/coordinator/lifecycle.rs:25
   #[derive(Clone, Debug)]  // Add Debug
   pub struct MemoryCoordinator {
   ```

2. **Or use custom Debug for CommandExecutor**:
   ```rust
   // Remove #[derive(Debug)] from CommandExecutor and implement manually:
   impl std::fmt::Debug for CommandExecutor {
       fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           f.debug_struct("CommandExecutor")
               .field("memory", &"<MemoryCoordinator>")
               .finish()
       }
   }
   ```

---

## What's Already Complete ✅

- ✅ Export delegation in execute_streaming() (lines 187-212) - correct pattern
- ✅ execute_export_streaming() implementation (lines 425-629) - fully functional
- ✅ retrieve_conversation_messages() helper (lines 622-690) - correct logic
- ✅ Progress tracking (25%, 50%, 75%, 90%, 100%)
- ✅ Error handling with proper error codes
- ✅ File writing with tokio::fs::write()
- ✅ All 4 export formats supported

---

## Definition of Done

- [ ] All 4 compilation errors fixed
- [ ] `cargo check -p cyrup_candle` succeeds
- [ ] Export command writes actual files when executed
- [ ] No unwrap() or expect() calls added

---

## Implementation Priority

Fix in this order:
1. **Error 4 first** - Add Debug to MemoryCoordinator (easiest, affects whole struct)
2. **Error 1** - Remove invalid #[debug(skip)] attribute  
3. **Error 2** - Add public list_all_memories() method to MemoryCoordinator
4. **Error 3** - Make convert_memory_to_domain_node() public

Then run: `cargo check -p cyrup_candle`
