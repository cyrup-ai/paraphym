# LEGACY_F: Environment Variable Fallbacks

## OBJECTIVE
Remove environment variable fallbacks for old MCP_DB_* naming. Use only DB_* variables - no fallbacks in an UNRELEASED library.

## SCOPE
File: `packages/sweetmcp/packages/axum/src/db/from_env_doc.rs`

## BACKGROUND
Classic lazy backward compat: renamed env vars from MCP_DB_* to DB_*, but kept fallbacks instead of updating configs. In an unreleased library, this is pure laziness.

## SUBTASK 1: Remove DB_URL fallbacks
**Location:** Lines 30-32

**BEFORE:**
```rust
let url = std::env::var("DB_URL")
    .ok()
    .or_else(|| std::env::var("MCP_DB_WS_ENDPOINT").ok())
    .or_else(|| std::env::var("MCP_DB_TIKV_ENDPOINT").ok());
```

**AFTER:**
```rust
let url = std::env::var("DB_URL").ok();
```

## SUBTASK 2: Remove DB_NAMESPACE fallbacks
**Location:** Lines 34-37

**BEFORE:**
```rust
let namespace = std::env::var("DB_NAMESPACE")
    .ok()
    .or_else(|| std::env::var("MCP_DB_WS_NS").ok())
    .or_else(|| std::env::var("MCP_DB_TIKV_NS").ok())
    .or_else(|| Some("mcp".to_string()));
```

**AFTER:**
```rust
let namespace = std::env::var("DB_NAMESPACE")
    .ok()
    .or_else(|| Some("mcp".to_string()));
```

## SUBTASK 3: Remove DB_DATABASE fallbacks
**Location:** Lines 39-42

**BEFORE:**
```rust
let database = std::env::var("DB_DATABASE")
    .ok()
    .or_else(|| std::env::var("MCP_DB_WS_DB").ok())
    .or_else(|| std::env::var("MCP_DB_TIKV_DB").ok())
    .or_else(|| Some("chat_sessions".to_string()));
```

**AFTER:**
```rust
let database = std::env::var("DB_DATABASE")
    .ok()
    .or_else(|| Some("chat_sessions".to_string()));
```

## SUBTASK 4: Remove DB_USERNAME fallbacks
**Location:** Lines 44-46

**BEFORE:**
```rust
let username = std::env::var("DB_USERNAME")
    .ok()
    .or_else(|| std::env::var("MCP_DB_WS_USER").ok())
    .or_else(|| std::env::var("MCP_DB_TIKV_USER").ok());
```

**AFTER:**
```rust
let username = std::env::var("DB_USERNAME").ok();
```

## SUBTASK 5: Remove DB_PASSWORD fallbacks
**Location:** Lines 48-50

**BEFORE:**
```rust
let password = std::env::var("DB_PASSWORD")
    .ok()
    .or_else(|| std::env::var("MCP_DB_WS_PASS").ok())
    .or_else(|| std::env::var("MCP_DB_TIKV_PASS").ok());
```

**AFTER:**
```rust
let password = std::env::var("DB_PASSWORD").ok();
```

## SUBTASK 6: Update documentation comment
**Location:** Lines 3-14

Remove references to MCP_DB_* variables. Update to:
```rust
/// Construct a DatabaseConfig from environment variables.
///
/// Supported environment variables:
/// - DB_ENGINE: "memory", "localkv", "surrealkv", "tikv", "websocket"
/// - DB_PATH: file path for LocalKv/SurrealKv
/// - DB_URL: URL for remote engines (WebSocket, TiKv)
/// - DB_NAMESPACE: namespace (default: "mcp")
/// - DB_DATABASE: database name (default: "chat_sessions")
/// - DB_USERNAME: username for authentication
/// - DB_PASSWORD: password for authentication
/// - DB_RUN_MIGRATIONS: "true" or "false"
```

## VALIDATION COMMANDS
```bash
# Verify no MCP_DB_ references remain
grep -n "MCP_DB_" packages/sweetmcp/packages/axum/src/db/from_env_doc.rs
# Expected: 0 results

# Verify compilation
cargo check -p sweetmcp_axum
```

## DEFINITION OF DONE
- ✅ All MCP_DB_* fallbacks removed
- ✅ Only DB_* environment variables used
- ✅ Documentation updated
- ✅ Code compiles without errors

## EXECUTION ORDER
**Task 6 of 8** - Independent, can execute anytime after LEGACY_E

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on environment variable cleanup
