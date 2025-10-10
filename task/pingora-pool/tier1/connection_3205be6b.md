# `forks/pingora/pingora-pool/src/connection.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-pool
- **File Hash**: 3205be6b  
- **Timestamp**: 2025-10-10T02:16:01.424992+00:00  
- **Lines of Code**: 407

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 407 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 81
  - TODO
  - 

```rust
    // to avoid race between 2 evictions on the queue
    hot_queue_remove_lock: Mutex<()>,
    // TODO: store the GroupKey to avoid hash collision?
}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 165
  - TODO
  - 

```rust
/// be picked up by another user/request.
pub struct ConnectionPool<S> {
    // TODO: n-way pools to reduce lock contention
    pool: RwLock<HashMap<GroupKey, Arc<PoolNode<PoolConnection<S>>>>>,
    lru: Lru<ID, ConnectionMeta>,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 289
  - TODO
  - 

```rust
            _ = notify_evicted.notified() => {
                debug!("idle connection is being evicted");
                // TODO: gracefully close the connection?
                return
            }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 331
  - TODO
  - 

```rust
            _ = notify_evicted.notified() => {
                debug!("idle connection is being evicted");
                // TODO: gracefully close the connection?
            }
            _ = notify_closed.changed() => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 449
  - stubby variable name
  - mock_io1

```rust
    async fn test_read_close() {
        let meta1 = ConnectionMeta::new(101, 1);
        let mock_io1 = Arc::new(AsyncMutex::new(Builder::new().read(b"garbage").build()));
        let meta2 = ConnectionMeta::new(102, 2);
        let mock_io2 = Arc::new(AsyncMutex::new(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 451
  - stubby variable name
  - mock_io2

```rust
        let mock_io1 = Arc::new(AsyncMutex::new(Builder::new().read(b"garbage").build()));
        let meta2 = ConnectionMeta::new(102, 2);
        let mock_io2 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 455
  - stubby variable name
  - mock_io3

```rust
        ));
        let meta3 = ConnectionMeta::new(101, 3);
        let mock_io3 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 459
  - stubby variable name
  - mock_io1

```rust
        ));
        let cp: ConnectionPool<Arc<AsyncMutex<Mock>>> = ConnectionPool::new(3);
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 460
  - stubby variable name
  - mock_io2

```rust
        let cp: ConnectionPool<Arc<AsyncMutex<Mock>>> = ConnectionPool::new(3);
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone());

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 461
  - stubby variable name
  - mock_io3

```rust
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone());

        let closed_item = tokio::select! {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 464
  - stubby variable name
  - mock_io1

```rust

        let closed_item = tokio::select! {
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, None, c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, None, c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, None, c3, u3) => {debug!("notifier3"); 3},
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 465
  - stubby variable name
  - mock_io2

```rust
        let closed_item = tokio::select! {
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, None, c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, None, c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, None, c3, u3) => {debug!("notifier3"); 3},
        };
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 466
  - stubby variable name
  - mock_io3

```rust
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, None, c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, None, c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, None, c3, u3) => {debug!("notifier3"); 3},
        };
        assert_eq!(closed_item, 1);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 470
  - stubby variable name
  - mock_io3

```rust
        assert_eq!(closed_item, 1);

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 471
  - stubby variable name
  - mock_io1

```rust

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 477
  - stubby variable name
  - mock_io1

```rust
    async fn test_read_timeout() {
        let meta1 = ConnectionMeta::new(101, 1);
        let mock_io1 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 481
  - stubby variable name
  - mock_io2

```rust
        ));
        let meta2 = ConnectionMeta::new(102, 2);
        let mock_io2 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 485
  - stubby variable name
  - mock_io3

```rust
        ));
        let meta3 = ConnectionMeta::new(101, 3);
        let mock_io3 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 489
  - stubby variable name
  - mock_io1

```rust
        ));
        let cp: ConnectionPool<Arc<AsyncMutex<Mock>>> = ConnectionPool::new(3);
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 490
  - stubby variable name
  - mock_io2

```rust
        let cp: ConnectionPool<Arc<AsyncMutex<Mock>>> = ConnectionPool::new(3);
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone());

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 491
  - stubby variable name
  - mock_io3

```rust
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone());

        let closed_item = tokio::select! {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 494
  - stubby variable name
  - mock_io1

```rust

        let closed_item = tokio::select! {
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(1)), c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(2)), c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(3)), c3, u3) => {debug!("notifier3"); 3},
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 495
  - stubby variable name
  - mock_io2

```rust
        let closed_item = tokio::select! {
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(1)), c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(2)), c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(3)), c3, u3) => {debug!("notifier3"); 3},
        };
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 496
  - stubby variable name
  - mock_io3

```rust
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(1)), c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(2)), c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, Some(Duration::from_secs(3)), c3, u3) => {debug!("notifier3"); 3},
        };
        assert_eq!(closed_item, 1);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 500
  - stubby variable name
  - mock_io3

```rust
        assert_eq!(closed_item, 1);

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 501
  - stubby variable name
  - mock_io1

```rust

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 507
  - stubby variable name
  - mock_io1

```rust
    async fn test_evict_poll() {
        let meta1 = ConnectionMeta::new(101, 1);
        let mock_io1 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 511
  - stubby variable name
  - mock_io2

```rust
        ));
        let meta2 = ConnectionMeta::new(102, 2);
        let mock_io2 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 515
  - stubby variable name
  - mock_io3

```rust
        ));
        let meta3 = ConnectionMeta::new(101, 3);
        let mock_io3 = Arc::new(AsyncMutex::new(
            Builder::new().wait(Duration::from_secs(99)).build(),
        ));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 519
  - stubby variable name
  - mock_io1

```rust
        ));
        let cp: ConnectionPool<Arc<AsyncMutex<Mock>>> = ConnectionPool::new(2);
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone()); // 1 should be evicted at this point
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 520
  - stubby variable name
  - mock_io2

```rust
        let cp: ConnectionPool<Arc<AsyncMutex<Mock>>> = ConnectionPool::new(2);
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone()); // 1 should be evicted at this point

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 521
  - stubby variable name
  - mock_io3

```rust
        let (c1, u1) = cp.put(&meta1, mock_io1.clone());
        let (c2, u2) = cp.put(&meta2, mock_io2.clone());
        let (c3, u3) = cp.put(&meta3, mock_io3.clone()); // 1 should be evicted at this point

        let closed_item = tokio::select! {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 524
  - stubby variable name
  - mock_io1

```rust

        let closed_item = tokio::select! {
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, None, c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, None, c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, None, c3, u3) => {debug!("notifier3"); 3},
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 525
  - stubby variable name
  - mock_io2

```rust
        let closed_item = tokio::select! {
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, None, c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, None, c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, None, c3, u3) => {debug!("notifier3"); 3},
        };
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 526
  - stubby variable name
  - mock_io3

```rust
            _ = cp.idle_poll(mock_io1.try_lock_owned().unwrap(), &meta1, None, c1, u1) => {debug!("notifier1"); 1},
            _ = cp.idle_poll(mock_io2.try_lock_owned().unwrap(), &meta1, None, c2, u2) => {debug!("notifier2"); 2},
            _ = cp.idle_poll(mock_io3.try_lock_owned().unwrap(), &meta1, None, c3, u3) => {debug!("notifier3"); 3},
        };
        assert_eq!(closed_item, 1);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 530
  - stubby variable name
  - mock_io3

```rust
        assert_eq!(closed_item, 1);

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 531
  - stubby variable name
  - mock_io1

```rust

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 110: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        };
        // unwrap is safe since we just found it
        let connection = connections.remove(&id).unwrap();
        /* NOTE: we don't resize or drop empty connections hashmap
         * We may want to do it if they consume too much memory
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 387: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        cp.put(&meta3, value3.clone());

        let found_b = cp.get(&meta2.key).unwrap();
        assert_eq!(found_b, value2);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 390: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(found_b, value2);

        let found_a1 = cp.get(&meta1.key).unwrap();
        let found_a2 = cp.get(&meta1.key).unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 391: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let found_a1 = cp.get(&meta1.key).unwrap();
        let found_a2 = cp.get(&meta1.key).unwrap();

        assert!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 413: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        cp.pop_closed(&meta1);

        let found_a1 = cp.get(&meta1.key).unwrap();
        assert_eq!(found_a1, value3);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 440: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(closed_item, 1);

        let found_a1 = cp.get(&meta1.key).unwrap();
        assert_eq!(found_a1, value3);
        assert_eq!(cp.get(&meta1.key), None)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 470: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(closed_item, 1);

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 500: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(closed_item, 1);

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 530: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(closed_item, 1);

        let _ = cp.get(&meta1.key).unwrap(); // mock_io3 should be selected
        assert!(cp.get(&meta1.key).is_none()) // mock_io1 should already be removed by idle_poll
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 368: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 368)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 375: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 375)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_lookup() {
        let meta1 = ConnectionMeta::new(101, 1);
        let value1 = "v1".to_string();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 399: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 399)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_pop() {
        let meta1 = ConnectionMeta::new(101, 1);
        let value1 = "v1".to_string();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 421: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 421)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_eviction() {
        let meta1 = ConnectionMeta::new(101, 1);
        let value1 = "v1".to_string();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 447: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 447)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to read.")]
    async fn test_read_close() {
        let meta1 = ConnectionMeta::new(101, 1);
        let mock_io1 = Arc::new(AsyncMutex::new(Builder::new().read(b"garbage").build()));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 475: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 475)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_read_timeout() {
        let meta1 = ConnectionMeta::new(101, 1);
        let mock_io1 = Arc::new(AsyncMutex::new(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 505: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 505)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_evict_poll() {
        let meta1 = ConnectionMeta::new(101, 1);
        let mock_io1 = Arc::new(AsyncMutex::new(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `read_with_timeout()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-pool/src/connection.rs` (line 346)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

async fn read_with_timeout<S>(
    mut connection: OwnedMutexGuard<S>,
    timeout_duration: Option<Duration>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym