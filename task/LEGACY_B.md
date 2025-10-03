# LEGACY_B: Type Alias Removal

## OBJECTIVE
Remove ALL 7 backward compatibility type aliases. This is an UNRELEASED library - type aliases serve no purpose except developer laziness.

## SCOPE
All type aliases exist solely for "backward compatibility" in an unreleased library. Delete them and update call sites to use canonical types.

## SUBTASK 1: Remove Prompt type alias
**File:** `packages/candle/src/domain/prompt/mod.rs:45`
```rust
// DELETE:
pub type Prompt = CandlePrompt;
```
**Migration:** `grep -r "use.*Prompt[^a-zA-Z]" packages/candle/src` then replace `Prompt` → `CandlePrompt`

## SUBTASK 2: Remove Usage type alias  
**File:** `packages/candle/src/domain/model/usage.rs:106`
```rust
// DELETE:
pub type Usage = CandleUsage;
```
**Migration:** Replace `Usage` → `CandleUsage` (ensure not conflicting with std types)

## SUBTASK 3: Remove CompletionChunk type alias
**File:** `packages/candle/src/domain/context/chunk.rs:285`
```rust
// DELETE:
pub type CompletionChunk = CandleCompletionChunk;
```
**Migration:** `CompletionChunk` → `CandleCompletionChunk`

## SUBTASK 4: Remove FileLoader type alias
**File:** `packages/candle/src/domain/context/loader.rs:266`
```rust
// DELETE:
pub type FileLoader<T> = LoaderImpl<T>;
```
**Migration:** `FileLoader<T>` → `LoaderImpl<T>`

## SUBTASK 5: Remove RelationshipSchema type alias
**File:** `packages/candle/src/memory/schema/relationship_schema.rs:12`
```rust
// DELETE:
pub type RelationshipSchema = Relationship;
```
**Migration:** `RelationshipSchema` → `Relationship`

## SUBTASK 6: Remove McpToolData type alias
**File:** `packages/candle/src/domain/tool/mod.rs:27`
```rust
// DELETE:
pub type McpToolData = ToolInfo;
```
**Migration:** `McpToolData` → `ToolInfo`

## SUBTASK 7: Remove CandleMcpToolData type alias
**File:** `packages/candle/src/domain/tool/mod.rs:28`
```rust
// DELETE:
pub type CandleMcpToolData = ToolInfo;
```
**Migration:** `CandleMcpToolData` → `ToolInfo`

## VALIDATION COMMANDS
```bash
# Verify no type aliases remain (expect 0 results)
grep -rn "^pub type.*=.*// Backward\|^pub type.*=.*// Legacy" packages/candle/src

# Verify compilation
cargo check -p paraphym_candle
```

## DEFINITION OF DONE
- ✅ All 7 type aliases removed
- ✅ All call sites updated to canonical types
- ✅ Code compiles without errors
- ✅ Zero "backward compat" type aliases remain

## EXECUTION ORDER
**Task 4 of 8** - Execute after LEGACY_C and LEGACY_D (after call sites updated)

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on type alias removal and call site updates
