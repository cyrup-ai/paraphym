# NOCODE_2: Delete Agent Role Old API or Migrate to Builders

## OBJECTIVE
Eliminate ALL dead code in agent/role.rs by either deleting the old AgentRoleImpl struct entirely OR completing migration to the builders pattern.

## PRIORITY
🔴 CRITICAL - Zero dead code tolerance

## BACKGROUND
`AgentRoleImpl` struct contains 10+ fields all marked with `#[allow(dead_code)] // TODO: Implement`. This appears to be an abandoned old API since the codebase uses the builders pattern (`builders/agent_role.rs`).

**Decision Required:** Delete old API OR complete migration to it. No middle ground.

## AFFECTED FILES
- `packages/candle/src/agent/role.rs` (lines 20-75)

---

## SUBTASK 1: Verify Usage of AgentRoleImpl

**Action:** Determine if AgentRoleImpl is actively used

**Commands:**
```bash
cd /Volumes/samsung_t9/paraphym
grep -rn "AgentRoleImpl" packages/candle/src --include="*.rs" | grep -v "^packages/candle/src/agent/role.rs"
grep -rn "McpServerConfig" packages/candle/src --include="*.rs" | grep -v "^packages/candle/src/agent/role.rs"
```

**Decision Tree:**

**If ZERO external references found:**
- Proceed to SUBTASK 2 (DELETE path)

**If references exist:**
- Proceed to SUBTASK 3 (MIGRATION path)

---

## SUBTASK 2: DELETE Path - Remove Old API Entirely

**Execute if:** AgentRoleImpl has no external usage

**File:** `packages/candle/src/agent/role.rs`

**Actions:**

1. **Delete McpServerConfig struct** (lines 20-26):
```rust
// DELETE ENTIRE STRUCT:
struct McpServerConfig {
    #[allow(dead_code)] // TODO: ...
    server_type: String,
    ...
}
```

2. **Delete AgentRoleImpl struct** (lines 47-75):
```rust
// DELETE ENTIRE STRUCT:
pub struct AgentRoleImpl {
    name: String,
    #[allow(dead_code)] // TODO: ...
    completion_provider: Option<...>,
    ...
}
```

3. **Keep AgentRole trait** if it's used elsewhere

4. **Delete any impl blocks for AgentRoleImpl**

5. **Update module exports:**
   - Remove `AgentRoleImpl` from pub use statements
   - Keep only actively used items

---

## SUBTASK 3: MIGRATION Path - Integrate All Fields

**Execute if:** AgentRoleImpl IS used in production code

**This is the hard path - full implementation required**

### 3A: Completion Provider Integration

**Field:** `completion_provider: Option<Box<dyn Any + Send + Sync>>`

**Action:**
1. Replace with concrete type from builders
2. Wire into completion system
3. Test integration compiles
4. Remove `#[allow(dead_code)]`

### 3B: Contexts Integration

**Field:** `contexts: Option<ZeroOneOrMany<Box<dyn Any + Send + Sync>>>`

**Action:**
1. Implement document context loading
2. Wire into agent conversation
3. Use contexts in prompt generation
4. Remove `#[allow(dead_code)]`

### 3C: Tools Integration

**Field:** `tools: Option<ZeroOneOrMany<Box<dyn Any + Send + Sync>>>`

**Action:**
1. Wire tools into function calling system
2. Connect to UnifiedToolExecutor
3. Enable tool execution in agent loop
4. Remove `#[allow(dead_code)]`

### 3D: MCP Servers Integration

**Field:** `mcp_servers: Option<ZeroOneOrMany<McpServerConfig>>`

**Action:**
1. Implement MCP server initialization
2. Connect to MCP protocol handlers
3. Enable MCP tool discovery
4. Remove `#[allow(dead_code)]`

### 3E: Additional Params Integration

**Field:** `additional_params: Option<HashMap<String, Value>>`

**Action:**
1. Pass to provider initialization
2. Support beta features, custom options
3. Document supported parameters
4. Remove `#[allow(dead_code)]`

### 3F: Memory Integration

**Field:** `memory: Option<Box<dyn Any + Send + Sync>>`

**Action:**
1. Connect to MemoryCoordinator
2. Enable persistent conversation storage
3. Wire into memory retrieval
4. Remove `#[allow(dead_code)]`

### 3G: Metadata Integration

**Field:** `metadata: Option<HashMap<String, Value>>`

**Action:**
1. Store agent-specific metadata
2. Use in logging/monitoring
3. Expose via API if needed
4. Remove `#[allow(dead_code)]`

### 3H: Tool Result Handler Integration

**Field:** `on_tool_result_handler: Option<Box<dyn Fn(...)>>`

**Action:**
1. Implement callback invocation after tool execution
2. Pass tool results through handler
3. Enable custom result processing
4. Remove `#[allow(dead_code)]`

### 3I: Conversation Turn Handler Integration

**Field:** `on_conversation_turn_handler: Option<Box<dyn Fn(...)>>`

**Action:**
1. Implement callback invocation on each turn
2. Enable event logging
3. Support custom turn processing
4. Remove `#[allow(dead_code)]`

---

## SUBTASK 4: Verify No Dead Code Remains

**Action:** Confirm complete elimination

**Verification:**
```bash
grep -n "#\[allow(dead_code)\]" packages/candle/src/agent/role.rs
```

**Expected:** ZERO results

**If any remain:** Go back and integrate or delete

---

## DEFINITION OF DONE

### Zero Dead Code (Both Paths)
- [ ] NO `#[allow(dead_code)]` annotations in agent/role.rs
- [ ] NO TODO comments about unimplemented features
- [ ] Code compiles without warnings

### DELETE Path (if chosen)
- [ ] McpServerConfig struct deleted
- [ ] AgentRoleImpl struct deleted
- [ ] All impl blocks for AgentRoleImpl deleted
- [ ] Module exports updated
- [ ] No external references broken

### MIGRATION Path (if chosen)
- [ ] All 10+ fields actively used in code
- [ ] Each field integrated into appropriate system
- [ ] Functionality proven through compilation
- [ ] All `#[allow(dead_code)]` removed

---

## CONSTRAINTS
- ❌ DO NOT write unit tests
- ❌ DO NOT write integration tests
- ❌ DO NOT write benchmarks
- ❌ DO NOT keep unused fields "for future use"
- ✅ Choose ONE path: delete OR migrate completely
- ✅ No half-measures allowed

---

## TECHNICAL NOTES

### Determining Active Usage
Check for:
- Direct instantiation: `AgentRoleImpl::new()`
- Type annotations: `: AgentRoleImpl`
- Pattern matching: `if let AgentRoleImpl { ... }`
- Trait object casts: `as Box<dyn AgentRole>`

### Migration to Builders
If migrating, the builders pattern is in:
- `packages/candle/src/builders/agent_role.rs`

Study how it handles:
- Provider configuration
- Tool integration
- MCP servers
- Conversation management

Then integrate old API fields into that pattern.

### Safe Deletion
Before deleting, verify:
```bash
# No construction calls
grep -rn "AgentRoleImpl::new" packages/candle/src

# No type usage
grep -rn ": AgentRoleImpl" packages/candle/src

# No imports
grep -rn "use.*AgentRoleImpl" packages/candle/src
```

### Git Recovery
If deletion is wrong, code can be restored:
```bash
git checkout HEAD -- packages/candle/src/agent/role.rs
```

## RECOMMENDATION

**Likely path: DELETE**

Evidence suggests AgentRoleImpl is unused old code:
1. Builders pattern exists and is active
2. ALL fields marked dead_code
3. No obvious external references

Verify with SUBTASK 1, then proceed with confident deletion.
