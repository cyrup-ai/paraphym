# WASM_1: Implement WASM HTTPS Fetch Support

## OBJECTIVE

Replace the "for now" error stub in WASM fetch with actual HTTPS support using appropriate WASM-compatible HTTP client.

## BACKGROUND

WASM build returns a hardcoded error for HTTPS requests with a "for now" comment. This breaks fetch functionality in WebAssembly environments.

## SUBTASK 1: Research WASM HTTP Solutions

**Location:** `packages/sweetmcp/plugins/fetch/src/hyper.rs:212`

**Current State:**
```rust
#[cfg(target_family = "wasm")]
pub async fn fetch(url: &str) -> Result<String, FetchError> {
    // WASM: Return error for now as HTTPS without rustls is not straightforward
    Err(FetchError::Other("HTTPS not available in WASM build. Use firecrawl backend.".to_string()))
}
```

**Required Changes:**
- Remove error stub and "for now" comment
- Research WASM-compatible HTTP clients:
  - `reqwest` with wasm feature
  - `gloo-net` (purpose-built for WASM)
  - `web-sys` fetch API
- Choose solution based on bundle size and API compatibility
- Document choice in code comments

**Why:** WASM needs HTTP support for browser-based functionality.

## SUBTASK 2: Implement WASM Fetch

**Location:** Same file

**Required Changes:**
- Add WASM-specific HTTP client dependency with feature flag
- Implement `fetch()` using chosen WASM HTTP client
- Handle CORS requirements in browser context
- Support both HTTP and HTTPS
- Match error handling of native implementation

**Why:** WASM build needs feature parity with native build.

## SUBTASK 3: Handle WASM-Specific Constraints

**Location:** Same file

**Required Changes:**
- Add timeout handling (WASM async runtime differences)
- Handle browser security restrictions (CORS, CSP)
- Add appropriate error mapping for browser errors
- Document WASM limitations if any
- Consider browser fetch API permissions

**Why:** WASM runtime has different constraints than native.

## SUBTASK 4: Unify Native and WASM Implementations

**Location:** Same file

**Required Changes:**
- Ensure consistent error types across platforms
- Ensure consistent timeout behavior
- Ensure consistent TLS validation (where applicable)
- Share common parsing logic
- Document platform-specific behavior differences

**Why:** API should be consistent across compilation targets.

## DEFINITION OF DONE

- [ ] No hardcoded error for WASM HTTPS
- [ ] WASM fetch implementation using appropriate HTTP client
- [ ] Support for both HTTP and HTTPS in WASM
- [ ] CORS and browser security constraints handled
- [ ] Error handling consistent with native implementation
- [ ] Documentation explains WASM-specific behavior
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### WASM HTTP Client Options

**Option 1: reqwest with wasm**
- Pros: Same API as native, good compatibility
- Cons: Larger bundle size
- Feature: `features = ["wasm"]`

**Option 2: gloo-net**
- Pros: Purpose-built for WASM, smaller size
- Cons: Different API from native
- URL: https://docs.rs/gloo-net

**Option 3: web-sys fetch**
- Pros: Direct browser API, minimal overhead
- Cons: More verbose, callback-based
- Requires: `web-sys` with fetch feature

### Browser Constraints
- CORS must be handled by server (OPTIONS preflight)
- CSP may block certain requests
- Mixed content rules (HTTPS page → HTTP request blocked)
- No custom TLS configuration in browser

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Keep WASM bundle size reasonable
- Maintain API compatibility with native implementation
