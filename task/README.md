# Paraphym Disconnected Features - Task Index

This directory contains detailed reconnection plans for all disconnected subsystems found in the codebase.

## Quick Start

**Read this first**: [`DISCONNECTIONS_SUMMARY.md`](./DISCONNECTIONS_SUMMARY.md) - Overview of all 220 disconnections across 7 major subsystems

## Task Files by Priority

### P0 - Critical Security (Fix Immediately)

#### 1. Authentication & Authorization
**File**: [`AUTH_CLAIMS.md`](./AUTH_CLAIMS.md)  
**Items**: 34 disconnected components  
**Severity**: CRITICAL

JWT authentication validates signatures but **never checks expiry** or **enforces roles/permissions**. Token encryption exists but unused.

**Key Issues**:
- Expired tokens accepted ❌
- No role-based access control (RBAC) ❌
- No permission enforcement ❌  
- HTTPS requirement not enforced ❌
- Tokens stored in plaintext ❌

**Reconnection**: 6 steps in AUTH_CLAIMS.md

---

#### 2. TLS Certificate Validation
**File**: [`TLS_VALIDATION.md`](./TLS_VALIDATION.md)  
**Items**: 42 disconnected components  
**Severity**: HIGH

Complete certificate validation (OCSP, CRL, chain, constraints) exists but bypassed. Uses basic rustls verification only.

**Key Issues**:
- No OCSP stapling (revoked certs not detected) ❌
- No CRL checking ❌
- No certificate auto-renewal ❌
- Basic validation only ❌

**Reconnection**: Wire up OCSP/CRL in TLS handshake

---

### P1 - High Reliability (Fix Soon)

#### 3. Graceful Shutdown
**File**: [`SHUTDOWN_GRACEFUL.md`](./SHUTDOWN_GRACEFUL.md)  
**Items**: 18 disconnected components  
**Severity**: HIGH

Full shutdown infrastructure (signals, draining, mDNS goodbye, state preservation) exists but never activated.

**Key Issues**:
- SIGTERM/SIGINT handlers never registered ❌
- No connection draining ❌
- Peers not notified of shutdown ❌
- No state preservation ❌

**Reconnection**: Register signal handlers, call setup methods

---

#### 4. Rate Limiting - HybridAlgorithm
**File**: [`RATE_LIMIT_HYBRID.md`](./RATE_LIMIT_HYBRID.md)  
**Items**: 14 disconnected components  
**Severity**: MEDIUM

HybridAlgorithm (TokenBucket + SlidingWindow) fully implemented but unreachable due to missing enum variant.

**Key Issues**:
- Stricter rate limiting unavailable ❌
- RateLimiter enum missing Hybrid variant ❌
- EdgeService hardcoded to DistributedRateLimitManager ❌

**Reconnection**: Add Hybrid to enums, create RateLimitManager trait

---

### P2 - Medium Features (Fix When Needed)

#### 5. Protocol Normalization
**File**: [`PROTOCOL_NORMALIZATION.md`](./PROTOCOL_NORMALIZATION.md)  
**Items**: 32 disconnected components  
**Severity**: MEDIUM

GraphQL, Cap'n Proto, JSON-RPC → MCP conversion pipeline exists but requests bypass it entirely.

**Key Issues**:
- Cannot accept GraphQL queries ❌
- Cannot accept Cap'n Proto binary ❌
- Protocol interop impossible ❌
- 17 schema introspection fields unused ❌

**Reconnection**: Add protocol detection to request filter

---

#### 6. Edge Service Builder Pattern
**File**: [`EDGE_SERVICE_BUILDER.md`](./EDGE_SERVICE_BUILDER.md)  
**Items**: 31 disconnected components  
**Severity**: MEDIUM

Comprehensive builder pattern with presets exists but EdgeService::new() called directly everywhere.

**Key Issues**:
- Cannot swap rate limiters ❌
- No environment-specific configs ❌
- No validation before construction ❌
- Builder pattern completely bypassed ❌

**Reconnection**: Replace EdgeService::new() with builder

---

#### 7. Image Tensor Processing
**File**: [`IMAGE_TENSOR_PROCESSING.md`](./IMAGE_TENSOR_PROCESSING.md)  
**Items**: 25+ disconnected components  
**Severity**: MEDIUM

Vision model preprocessing pipeline (CLIP, LLaVA, Stable Diffusion) exists but never integrated.

**Key Issues**:
- Cannot preprocess images for vision models ❌
- No base64 image support ❌
- No URL image loading ❌
- Complete tensor pipeline unused ❌

**Reconnection**: Integrate Image builder in vision model code

---

## Architecture Patterns

All disconnections follow similar patterns:

### 1. Builder Bypassed
Advanced builder exists → direct construction used instead

### 2. Trait Objects Avoided  
Trait abstraction exists → concrete type hardcoded instead

### 3. Validation Skipped
Validation function exists → never called in flow

### 4. Enum Variant Missing
Implementation exists → enum lacks variant to reach it

### 5. Setup Methods Uncalled
Setup methods exist → never called before use

## Statistics

### By Severity
- **CRITICAL**: 34 items (AUTH_CLAIMS)
- **HIGH**: 60 items (TLS_VALIDATION: 42, SHUTDOWN: 18)
- **MEDIUM**: 96+ items (PROTOCOL: 32, BUILDER: 31, IMAGE: 25+, RATE_LIMIT: 14)

### Total: ~196 items documented (89% of 220 warnings)

## Reconnection Approach

### Phase 1: Security (Days 1-3)
1. AUTH_CLAIMS.md - Add expiry validation, enforce RBAC
2. TLS_VALIDATION.md - Enable OCSP/CRL checking

### Phase 2: Reliability (Days 4-6)
3. SHUTDOWN_GRACEFUL.md - Register signals, enable draining
4. RATE_LIMIT_HYBRID.md - Make HybridAlgorithm accessible

### Phase 3: Features (Days 7-11)
5. PROTOCOL_NORMALIZATION.md - Enable if GraphQL/Cap'n Proto needed
6. EDGE_SERVICE_BUILDER.md - Migrate to builder pattern
7. IMAGE_TENSOR_PROCESSING.md - Integrate if vision models used

## Key Insight

**This is NOT about removing "dead code"** - these are high-quality implementations that simply aren't connected. The architecture is sound, it just needs the wiring completed.

Every task file includes:
- Problem statement with root cause
- List of disconnected components
- Current vs intended code comparison
- Step-by-step reconnection instructions
- Files to modify with line numbers
- Testing criteria

## Developer Guide

### Before You Start
1. Read DISCONNECTIONS_SUMMARY.md for full context
2. Choose task based on priority (P0 → P1 → P2)
3. Read the specific task file completely
4. Follow reconnection steps in order
5. Add integration tests
6. Verify no new warnings introduced

### After Reconnection
- [ ] Run `cargo check` - verify 0 warnings for that subsystem
- [ ] Run `cargo test` - ensure tests pass
- [ ] Add integration test proving feature is now reachable
- [ ] Update documentation to reflect actual architecture
- [ ] Mark task as complete in tracking system

## Questions?

Each task file includes:
- Investigation commands to explore the codebase
- References to example code in the repo
- Links to relevant files and line numbers
- Expected behavior after reconnection

If stuck, check the "Investigation Required" section in each task file for grep commands to explore the disconnection.
