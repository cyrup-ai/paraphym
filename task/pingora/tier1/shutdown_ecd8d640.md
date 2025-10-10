# `packages/sweetmcp/packages/pingora/tests/unit/shutdown.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora
- **File Hash**: ecd8d640  
- **Timestamp**: 2025-10-10T02:15:59.796558+00:00  
- **Lines of Code**: 67

---## Tier 1 Infractions 


- Line 8
  - stubby variable name
  - temp_dir

```rust
#[tokio::test]
async fn test_shutdown_coordinator() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 10
  - stubby variable name
  - temp_dir

```rust
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
    ));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 28
  - stubby variable name
  - temp_dir

```rust
#[tokio::test]
async fn test_state_persistence() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 30
  - stubby variable name
  - temp_dir

```rust
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
    ));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 43
  - stubby variable name
  - temp_dir

```rust
    // Create new coordinator and load state
    let coordinator2 = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
    ));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 53
  - stubby variable name
  - temp_dir

```rust
#[tokio::test]
async fn test_request_guard() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 55
  - stubby variable name
  - temp_dir

```rust
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
    ));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 68
  - stubby variable name
  - temp_dir

```rust
#[tokio::test]
async fn test_shutdown_blocks_new_requests() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 70
  - stubby variable name
  - temp_dir

```rust
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
    ));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 84
  - stubby variable name
  - temp_dir

```rust
#[tokio::test]
async fn test_shutdown_signal_subscription() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 86
  - stubby variable name
  - temp_dir

```rust
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let coordinator = Arc::new(ShutdownCoordinator::new(
        temp_dir.path().to_path_buf()
    ));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym