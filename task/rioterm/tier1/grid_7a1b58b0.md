# `packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rioterm
- **File Hash**: 7a1b58b0  
- **Timestamp**: 2025-10-10T02:15:58.900631+00:00  
- **Lines of Code**: 5631

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 5631 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 1337
  - TODO
  - 

```rust
        self.current = new_key;

        // TODO: Needs to validate this
        // In case the new context does not have down
        // it means it's the last one, for this case
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2546
  - TODO
  - 

```rust

        // Fifth context should not have any margin x
        // TODO:
        // assert_eq!(contexts[4].val.dimension.margin.x, 0.);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2554
  - TODO
  - 

```rust
        assert_eq!(contexts[1].val.dimension.margin.x, 10.);
        // Fourth context should not have any margin x
        // TODO:
        // assert_eq!(contexts[3].val.dimension.margin.x, 0.);
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2713
  - TODO
  - 

```rust

        // Fifth context should not have any margin x
        // TODO: Removal
        // grid.remove_current();
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3315
  - TODO
  - 

```rust
        grid.resize(1200.0, 600.0);

        // TODO: Finish test
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6329
  - FIX
  - 

```rust
        );

        // THE FIX TEST: Move divider left from panel 3
        // Before the fix, this would return false because panel 3 couldn't find horizontal relationships
        let move_amount = 50.0;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2046
  - stubby method name
  - mock_context

```rust
        assert_eq!(context_dimension.lines, 88);
        let rich_text_id = 0;
        let context = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2100
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2113
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2159
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2225
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2238
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2307
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2358
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2446
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2459
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2484
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2497
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2510
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2584
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2597
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2610
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2623
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 4;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2636
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 5;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2747
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2760
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2806
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2900
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2913
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2939
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2952
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3106
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3119
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3165
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3231
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3244
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3257
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3344
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3357
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3432
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3445
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3531
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3544
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3579
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3637
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3650
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3725
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3738
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3824
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3837
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3872
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3930
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3943
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3956
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3969
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 4;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3982
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 5;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3995
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 6;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4166
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4179
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4192
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 3;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4205
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 4;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4290
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 5;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4303
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 6;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4316
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 7;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4400
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 5;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4413
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 6;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4498
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 0;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4511
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 1;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4550
  - stubby method name
  - mock_context

```rust
            let rich_text_id = 2;
            (
                create_mock_context(
                    VoidListener {},
                    WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4588
  - stubby method name
  - mock_context

```rust
        let context_dimension = ContextDimension::default();
        let context =
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4651
  - stubby method name
  - mock_context

```rust
        // Create a complex grid structure
        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4658
  - stubby method name
  - mock_context

```rust
        // Add multiple splits to create a complex structure
        for i in 1..=5 {
            let context = create_mock_context(
                VoidListener {},
                WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4702
  - stubby method name
  - mock_context

```rust

        let context =
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4731
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4738
  - stubby method name
  - mock_context

```rust
        // Add a split
        let context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4762
  - stubby method name
  - mock_context

```rust
        let context_dimension = ContextDimension::default();
        let context =
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4794
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4801
  - stubby method name
  - mock_context

```rust
        // Create many splits
        for i in 1..=20 {
            let context = create_mock_context(
                VoidListener {},
                WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4854
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4861
  - stubby method name
  - mock_context

```rust
        // Add a split
        let context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4929
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4939
  - stubby method name
  - mock_context

```rust
        // Add a split down
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_down(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4973
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4980
  - stubby method name
  - mock_context

```rust
        // Add a split down
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_down(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5013
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5023
  - stubby method name
  - mock_context

```rust
        // Add a split right
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5080
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5087
  - stubby method name
  - mock_context

```rust
        // Add a split right
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5144
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5151
  - stubby method name
  - mock_context

```rust
        // Add splits
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5155
  - stubby method name
  - mock_context

```rust

        let third_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 2, context_dimension);
        grid.split_down(third_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5185
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5192
  - stubby method name
  - mock_context

```rust
        // Create a complex layout: split right, then split down on the right side
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5196
  - stubby method name
  - mock_context

```rust

        let third_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 2, context_dimension);
        grid.split_down(third_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5229
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5236
  - stubby method name
  - mock_context

```rust
        // Test with zero amount
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5267
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5280
  - stubby method name
  - mock_context

```rust
        // Add only a vertical split (down)
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_down(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5313
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5320
  - stubby method name
  - mock_context

```rust
        // Create multiple splits
        for i in 1..6 {
            let context = create_mock_context(
                VoidListener {},
                WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5371
  - stubby method name
  - mock_context

```rust

        let mut grid = ContextGrid::<VoidListener>::new(
            create_mock_context(VoidListener {}, WindowId::from(0), 0, context_dimension),
            margin,
            [0., 0., 0., 0.],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5378
  - stubby method name
  - mock_context

```rust
        // Add a horizontal split
        let second_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 1, context_dimension);
        grid.split_right(second_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5402
  - stubby method name
  - mock_context

```rust
        // Test with vertical split
        let third_context =
            create_mock_context(VoidListener {}, WindowId::from(0), 2, context_dimension);
        grid.split_down(third_context);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5447
  - stubby method name
  - mock_context

```rust

        let context =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);

        let margin = Delta {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5492
  - stubby method name
  - mock_context

```rust
        );

        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5499
  - stubby method name
  - mock_context

```rust
        );

        let second_context = create_mock_context(
            VoidListener,
            WindowId::from(1),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5563
  - stubby method name
  - mock_context

```rust
        );

        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5570
  - stubby method name
  - mock_context

```rust
        );

        let second_context = create_mock_context(
            VoidListener,
            WindowId::from(1),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5623
  - stubby method name
  - mock_context

```rust

        // Create separate contexts instead of trying to clone
        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5630
  - stubby method name
  - mock_context

```rust
        );

        let second_context = create_mock_context(
            VoidListener,
            WindowId::from(1),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5637
  - stubby method name
  - mock_context

```rust
        );

        let third_context = create_mock_context(
            VoidListener,
            WindowId::from(2),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5645
  - stubby method name
  - mock_context

```rust

        let fourth_context =
            create_mock_context(VoidListener, WindowId::from(3), 4, context_dimension);

        let margin = Delta {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5716
  - stubby method name
  - mock_context

```rust

        let context =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);

        let grid = ContextGrid::<VoidListener>::new(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5743
  - stubby method name
  - mock_context

```rust
        );

        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5751
  - stubby method name
  - mock_context

```rust

        let second_context =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);

        let mut grid = ContextGrid::<VoidListener>::new(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5793
  - stubby method name
  - mock_context

```rust
        );

        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5801
  - stubby method name
  - mock_context

```rust

        let second_context =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);

        let mut grid = ContextGrid::<VoidListener>::new(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5845
  - stubby method name
  - mock_context

```rust
        );

        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5853
  - stubby method name
  - mock_context

```rust

        let second_context =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);

        let mut grid = ContextGrid::<VoidListener>::new(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5895
  - stubby method name
  - mock_context

```rust
        );

        let first_context = create_mock_context(
            VoidListener,
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5903
  - stubby method name
  - mock_context

```rust

        let second_context =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);

        let mut grid = ContextGrid::<VoidListener>::new(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5951
  - stubby method name
  - mock_context

```rust
        // Create contexts
        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5953
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 5955
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        // Build layout: |1|2/3|
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6059
  - stubby method name
  - mock_context

```rust
        // Create contexts
        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6061
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6063
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        // Build layout: |1|2/3|
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6125
  - stubby method name
  - mock_context

```rust

        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6127
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6173
  - stubby method name
  - mock_context

```rust

        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let mut grid =
            ContextGrid::<VoidListener>::new(context1, margin, [1.0, 1.0, 1.0, 1.0]);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6201
  - stubby method name
  - mock_context

```rust

        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6203
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6271
  - stubby method name
  - mock_context

```rust
        // Create the |1|2/3| layout step by step
        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6273
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6275
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6415
  - stubby method name
  - mock_context

```rust

        // Create a simple context for testing
        let context = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6435
  - stubby method name
  - mock_context

```rust

        // Create second context and split right
        let second_context = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6454
  - stubby method name
  - mock_context

```rust

        // Create third context and split down from right panel
        let third_context = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6480
  - stubby method name
  - mock_context

```rust

        // Create initial context
        let context = create_mock_context(
            VoidListener {},
            WindowId::from(0), // unique route_id
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6497
  - stubby method name
  - mock_context

```rust

        // Create |1|2| layout
        let context2 = create_mock_context(
            VoidListener {},
            WindowId::from(0), // unique route_id
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6513
  - stubby method name
  - mock_context

```rust

        // Create |1|2/3| layout (split panel 2 down)
        let context3 = create_mock_context(
            VoidListener {},
            WindowId::from(0), // unique route_id
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6559
  - stubby method name
  - mock_context

```rust

        // Create initial context
        let context = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6576
  - stubby method name
  - mock_context

```rust

        // Create |1|2|3| layout
        let context2 = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6591
  - stubby method name
  - mock_context

```rust
        let panel2_key = grid.current;

        let context3 = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6640
  - stubby method name
  - mock_context

```rust

        // Create initial context
        let context = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6663
  - stubby method name
  - mock_context

```rust

        // Create right child
        let context2 = create_mock_context(
            VoidListener {},
            WindowId::from(0),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6705
  - stubby method name
  - mock_context

```rust
        // Create contexts for |1|2/3| layout
        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6707
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6709
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6806
  - stubby method name
  - mock_context

```rust
        // Create contexts for |1|2/3| layout
        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6808
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6810
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6895
  - stubby method name
  - mock_context

```rust

        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6897
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6899
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
        let context4 =
            create_mock_context(VoidListener, WindowId::from(3), 4, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6901
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
        let context4 =
            create_mock_context(VoidListener, WindowId::from(3), 4, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6976
  - stubby method name
  - mock_context

```rust

        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6978
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 6980
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 7033
  - stubby method name
  - mock_context

```rust
        // Create the exact layout described: |1|2/3|
        let context1 =
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 7035
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(0), 1, context_dimension);
        let context2 =
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 7037
  - stubby method name
  - mock_context

```rust
            create_mock_context(VoidListener, WindowId::from(1), 2, context_dimension);
        let context3 =
            create_mock_context(VoidListener, WindowId::from(2), 3, context_dimension);

        let mut grid =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 769
  - Fallback
  - 

```rust
        }

        // Fallback to grid margin if parent not found
        self.margin
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1112
  - Fallback
  - 

```rust
        }

        // Fallback: just remove the item
        self.inner.remove(&to_be_removed);
        if let Some(first_key) = self.inner.keys().next() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2991
  - actual
  - 

```rust
        let contexts = grid.contexts_ordered();

        // Debug: print actual values
        println!("Debug test_split_right_with_margin after fourth split:");
        for (i, context) in contexts.iter().enumerate() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3602
  - actual
  - 

```rust
        assert_eq!(grid.current().dimension.height, 296.);

        // Remove the current should actually make right being down
        grid.remove_current();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3895
  - actual
  - 

```rust
        assert_eq!(grid.current().dimension.width, 296.);

        // Remove the current should actually make down being down
        grid.remove_current();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 4269
  - actual
  - 

```rust
        // |3.----|.4----|

        // Remove the current should actually make right being down
        grid.remove_current();
        let current_index = grid.current_index();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 7063
  - actual
  - 

```rust
        assert!(move_result, "Moving divider left should work");

        // Check that panel 3's width actually changed
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 1081: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            if self.inner.contains_key(&right_val) {
                let (right_width, to_be_removed_width) = {
                    let right_item = self.inner.get(&right_val).unwrap();
                    let to_be_removed_item = self.inner.get(&to_be_removed).unwrap();
                    (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1082: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let (right_width, to_be_removed_width) = {
                    let right_item = self.inner.get(&right_val).unwrap();
                    let to_be_removed_item = self.inner.get(&to_be_removed).unwrap();
                    (
                        right_item.val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1374: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    if parent.down == Some(current_key) {
                        let (current_height, parent_height) = {
                            let current_item = self.inner.get(&current_key).unwrap();
                            let parent_item = self.inner.get(&parent_key).unwrap();
                            (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1375: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                        let (current_height, parent_height) = {
                            let current_item = self.inner.get(&current_key).unwrap();
                            let parent_item = self.inner.get(&parent_key).unwrap();
                            (
                                current_item.val.dimension.height,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1422: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            if self.inner.contains_key(&down_child_key) {
                let (current_height, down_height) = {
                    let current_item = self.inner.get(&current_key).unwrap();
                    let down_item = self.inner.get(&down_child_key).unwrap();
                    (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1423: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let (current_height, down_height) = {
                    let current_item = self.inner.get(&current_key).unwrap();
                    let down_item = self.inner.get(&down_child_key).unwrap();
                    (
                        current_item.val.dimension.height,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1482: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    if parent.down == Some(current_key) {
                        let (current_height, parent_height) = {
                            let current_item = self.inner.get(&current_key).unwrap();
                            let parent_item = self.inner.get(&parent_key).unwrap();
                            (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1483: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                        let (current_height, parent_height) = {
                            let current_item = self.inner.get(&current_key).unwrap();
                            let parent_item = self.inner.get(&parent_key).unwrap();
                            (
                                current_item.val.dimension.height,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1530: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            if self.inner.contains_key(&down_child_key) {
                let (current_height, down_height) = {
                    let current_item = self.inner.get(&current_key).unwrap();
                    let down_item = self.inner.get(&down_child_key).unwrap();
                    (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1531: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let (current_height, down_height) = {
                    let current_item = self.inner.get(&current_key).unwrap();
                    let down_item = self.inner.get(&down_child_key).unwrap();
                    (
                        current_item.val.dimension.height,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1645: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if let (Some(left_key), Some(right_key)) = (left_split, right_split) {
            let (left_width, right_width) = {
                let left_item = self.inner.get(&left_key).unwrap();
                let right_item = self.inner.get(&right_key).unwrap();
                (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1646: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let (left_width, right_width) = {
                let left_item = self.inner.get(&left_key).unwrap();
                let right_item = self.inner.get(&right_key).unwrap();
                (
                    left_item.val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1760: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if let (Some(left_key), Some(right_key)) = (left_split, right_split) {
            let (left_width, right_width) = {
                let left_item = self.inner.get(&left_key).unwrap();
                let right_item = self.inner.get(&right_key).unwrap();
                (
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1761: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let (left_width, right_width) = {
                let left_item = self.inner.get(&left_key).unwrap();
                let right_item = self.inner.get(&right_key).unwrap();
                (
                    left_item.val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 5412: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            })
            .map(|(key, _item)| key)
            .unwrap()
            .clone();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 5409: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .iter()
            .find(|(_key, item)| {
                item.down.is_some() && item.down.unwrap() == grid.current
            })
            .map(|(key, _item)| key)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 5983: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial widths
        let initial_panel1_width =
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 5985: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 5987: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panel 2 and 3 should have the same width (they're in the same vertical stack)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6000: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Check that the widths changed correctly
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6001: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Check that the widths changed correctly
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6002: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panel 1 should shrink, panels 2 and 3 should expand
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6032: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Should be back to approximately original widths
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6033: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Should be back to approximately original widths
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6034: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        assert!((final_panel1_width - initial_panel1_width).abs() < 1.0);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6081: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial widths
        let initial_panel1_width =
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6083: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6085: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Move divider left from panel 2
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6095: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Check that the widths changed correctly
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6096: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Check that the widths changed correctly
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6097: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panel 1 should shrink, panel 2 and 3 should expand equally
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6218: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial heights
        let initial_panel1_height =
            grid.inner.get(&panel1_key).unwrap().val.dimension.height;
        let initial_panel2_height =
            grid.inner.get(&panel2_key).unwrap().val.dimension.height;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6220: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.height;
        let initial_panel2_height =
            grid.inner.get(&panel2_key).unwrap().val.dimension.height;

        // Move divider up (shrink bottom panel, expand top panel)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6229: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );

        let new_panel1_height = grid.inner.get(&panel1_key).unwrap().val.dimension.height;
        let new_panel2_height = grid.inner.get(&panel2_key).unwrap().val.dimension.height;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6230: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let new_panel1_height = grid.inner.get(&panel1_key).unwrap().val.dimension.height;
        let new_panel2_height = grid.inner.get(&panel2_key).unwrap().val.dimension.height;

        // Top panel should expand, bottom panel should shrink
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6243: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let final_panel1_height =
            grid.inner.get(&panel1_key).unwrap().val.dimension.height;
        let final_panel2_height =
            grid.inner.get(&panel2_key).unwrap().val.dimension.height;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6245: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.height;
        let final_panel2_height =
            grid.inner.get(&panel2_key).unwrap().val.dimension.height;

        // Should be back to approximately original heights
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6317: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial widths
        let initial_panel1_width =
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6319: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6321: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panels 2 and 3 should have the same width (they're in the same vertical column)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6338: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Verify the changes
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6339: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Verify the changes
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6340: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panel 1 should shrink
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6390: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Should be back to approximately original widths
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6391: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Should be back to approximately original widths
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6392: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        assert!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6429: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut grid = ContextGrid::new(context, Delta::default(), [0.0, 0.0, 0.0, 0.0]);
        let root_key = grid.root.unwrap();

        // Verify root has no parent
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6494: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut grid = ContextGrid::new(context, Delta::default(), [0.0, 0.0, 0.0, 0.0]);
        let panel1_key = grid.root.unwrap();

        // Create |1|2| layout
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6573: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut grid = ContextGrid::new(context, Delta::default(), [0.0, 0.0, 0.0, 0.0]);
        let panel1_key = grid.root.unwrap();

        // Create |1|2|3| layout
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6623: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // (the exact behavior depends on removal logic, but parent references should be consistent)
        if grid.inner.contains_key(&panel3_key) {
            let panel3_parent = grid.inner.get(&panel3_key).unwrap().parent;
            if let Some(parent_key) = panel3_parent {
                assert!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6654: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut grid = ContextGrid::new(context, Delta::default(), [0.0, 0.0, 0.0, 0.0]);
        let root_key = grid.root.unwrap();

        // Test root margin calculation (should use grid margin)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6725: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial widths
        let initial_panel1_width =
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6727: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let initial_panel2_width =
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6729: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panels 2 and 3 should start with the same width (they're in the same vertical stack)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6742: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );

        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6743: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6744: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let new_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let new_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Panel 1 should shrink
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6777: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );

        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6778: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6779: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let final_panel1_width = grid.inner.get(&panel1_key).unwrap().val.dimension.width;
        let final_panel2_width = grid.inner.get(&panel2_key).unwrap().val.dimension.width;
        let final_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Should be back to approximately original widths
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6829: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        grid.current = panel2_key;
        let initial_widths_2 = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6830: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let initial_widths_2 = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6831: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6837: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let after_panel2_widths = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6838: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let after_panel2_widths = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6839: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6848: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        grid.current = panel3_key;
        let initial_widths_3 = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6849: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let initial_widths_3 = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6850: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6856: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let after_panel3_widths = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6857: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let after_panel3_widths = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6858: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
        );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6927: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial widths
        let _initial_widths = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6928: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let _initial_widths = (
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
            grid.inner.get(&panel4_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6929: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel1_key).unwrap().val.dimension.width,
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
            grid.inner.get(&panel4_key).unwrap().val.dimension.width,
        );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6930: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            grid.inner.get(&panel2_key).unwrap().val.dimension.width,
            grid.inner.get(&panel3_key).unwrap().val.dimension.width,
            grid.inner.get(&panel4_key).unwrap().val.dimension.width,
        );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6940: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if move_result {
            let new_widths = (
                grid.inner.get(&panel1_key).unwrap().val.dimension.width,
                grid.inner.get(&panel2_key).unwrap().val.dimension.width,
                grid.inner.get(&panel3_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6941: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let new_widths = (
                grid.inner.get(&panel1_key).unwrap().val.dimension.width,
                grid.inner.get(&panel2_key).unwrap().val.dimension.width,
                grid.inner.get(&panel3_key).unwrap().val.dimension.width,
                grid.inner.get(&panel4_key).unwrap().val.dimension.width,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6942: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                grid.inner.get(&panel1_key).unwrap().val.dimension.width,
                grid.inner.get(&panel2_key).unwrap().val.dimension.width,
                grid.inner.get(&panel3_key).unwrap().val.dimension.width,
                grid.inner.get(&panel4_key).unwrap().val.dimension.width,
            );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6943: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                grid.inner.get(&panel2_key).unwrap().val.dimension.width,
                grid.inner.get(&panel3_key).unwrap().val.dimension.width,
                grid.inner.get(&panel4_key).unwrap().val.dimension.width,
            );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6995: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Calculate initial total width
        let initial_total = grid.inner.get(&panel1_key).unwrap().val.dimension.width
            + grid.inner.get(&panel2_key).unwrap().val.dimension.width; // Panel 3 shares width with Panel 2

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 6996: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Calculate initial total width
        let initial_total = grid.inner.get(&panel1_key).unwrap().val.dimension.width
            + grid.inner.get(&panel2_key).unwrap().val.dimension.width; // Panel 3 shares width with Panel 2

        // Move divider and check total width is preserved
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 7002: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert!(grid.move_divider_left(50.0));

        let new_total = grid.inner.get(&panel1_key).unwrap().val.dimension.width
            + grid.inner.get(&panel2_key).unwrap().val.dimension.width; // Panel 3 shares width with Panel 2

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 7003: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let new_total = grid.inner.get(&panel1_key).unwrap().val.dimension.width
            + grid.inner.get(&panel2_key).unwrap().val.dimension.width; // Panel 3 shares width with Panel 2

        assert!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 7056: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Record initial width of panel 3
        let initial_panel3_width =
            grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // Move divider left (this should now work and change panel 3's width)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 7064: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Check that panel 3's width actually changed
        let new_panel3_width = grid.inner.get(&panel3_key).unwrap().val.dimension.width;

        // This is the key assertion - panel 3's width should have changed!
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 1981: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 1981)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    clippy::clone_on_copy
)]
pub mod test {
    use super::*;
    // Easier to test big structs
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1990: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 1990)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_compute() {
        // (1000. / ((74. / 2.)=37.))
        // (1000./37.)=27.027
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2024: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2024)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_single_context_respecting_margin_and_no_quad_creation() {
        let margin = Delta {
            x: 10.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2075: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2075)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_split_right() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2200: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2200)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_split_right_with_margin() {
        let margin = Delta {
            x: 20.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2421: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2421)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_split_right_with_margin_inside_parent() {
        let margin = Delta {
            x: 20.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2559: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2559)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_split_down_with_margin_inside_parent() {
        let margin = Delta {
            x: 20.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2719: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2719)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[test]
    // https://github.com/raphamorim/rio/issues/760
    fn test_split_issue_760() {
        let width = 1200.;
        let height = 800.;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2875: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 2875)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_right_with_margin() {
        let margin = Delta {
            x: 20.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3081: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3081)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_split_down() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3206: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3206)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_resize() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3319: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3319)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_right_without_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3407: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3407)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_right_with_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3506: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3506)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_right_with_down_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3612: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3612)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_down_without_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3700: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3700)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_down_with_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3799: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3799)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_down_with_right_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3905: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 3905)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_context_with_parent_but_down_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4141: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4141)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove_context_without_parents_but_with_right_and_down_children() {
        let margin = Delta {
            x: 0.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4472: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4472)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_select_current_based_on_mouse() {
        let mut mouse = Mouse::default();
        let margin = Delta {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4584: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4584)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_edge_case_empty_grid() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4600: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4600)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_edge_case_invalid_dimensions() {
        // Test with zero dimensions
        let (cols, lines) =
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4635: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4635)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_edge_case_complex_removal_scenario() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4693: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4693)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[test]
    #[allow(clippy::field_reassign_with_default, clippy::bool_comparison)]
    fn test_edge_case_dimension_updates_with_invalid_data() {
        let margin = Delta::default();
        let mut context_dimension = ContextDimension::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4716: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4716)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_edge_case_mouse_selection_with_invalid_coordinates() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4758: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4758)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_edge_case_navigation_with_empty_or_invalid_states() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4779: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4779)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_stress_test_many_splits() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4839: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4839)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_edge_case_resize_with_extreme_values() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4876: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4876)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_dimension_calculation_edge_cases() {
        // Test with very small positive values
        let (cols, lines) = compute(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4914: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4914)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_up_basic() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4958: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4958)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_down_basic() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 4998: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 4998)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_left_basic() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5065: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5065)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_right_basic() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5129: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5129)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_minimum_size_constraints() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5170: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5170)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_complex_layout() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5214: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5214)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_edge_cases() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5252: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5252)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_no_adjacent_splits() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5298: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5298)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_stress_test() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5356: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5356)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_preserves_total_space() {
        let margin = Delta::default();
        let context_dimension = ContextDimension::build(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5433: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5433)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_position_calculation_single_context() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5467: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5467)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_position_calculation_after_split_right() {
        let first_context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5538: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5538)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_position_calculation_after_split_down() {
        let first_context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5609: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5609)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_position_calculation_complex_layout() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5702: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5702)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_scaled_padding_consistency() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5730: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5730)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_right_updates_positions() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5780: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5780)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_down_updates_positions() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5832: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5832)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_left_updates_positions() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5882: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5882)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_move_divider_up_updates_positions() {
        let context_dimension = ContextDimension::build(
            600.,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 5934: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 5934)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_in_complex_layout() {
        // Test the |1|2/3| layout where panel 3 should be able to move horizontal dividers
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6042: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6042)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_from_panel2_in_complex_layout() {
        // Test the |1|2/3| layout where panel 2 should also be able to move horizontal dividers
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6109: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6109)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_limits() {
        // Test that divider movement respects minimum width limits
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6157: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6157)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_single_panel() {
        // Test that divider movement fails gracefully with only one panel
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6185: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6185)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_vertical_divider_movement() {
        // Test vertical divider movement in a simple horizontal split
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6253: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6253)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_fix_for_complex_layout() {
        // This test specifically addresses the issue where panel 3 in |1|2/3| layout
        // couldn't move horizontal dividers. This was the main bug we fixed.
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6409: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6409)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parent_references_basic() {
        use crate::context::create_mock_context;
        use rio_backend::event::WindowId;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6474: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6474)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parent_references_complex_layout() {
        use crate::context::create_mock_context;
        use rio_backend::event::WindowId;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6553: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6553)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parent_references_after_removal() {
        use crate::context::create_mock_context;
        use rio_backend::event::WindowId;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6634: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6634)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_find_node_margin_optimization() {
        use crate::context::create_mock_context;
        use rio_backend::event::WindowId;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6688: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6688)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_vertical_stack_width_propagation() {
        // Test that moving horizontal dividers correctly updates width for all panels in vertical stacks
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6789: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6789)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_from_different_panels_in_stack() {
        // Test that divider movement works the same regardless of which panel in the stack is selected
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6879: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6879)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_complex_vertical_stack() {
        // Test with a more complex vertical stack: |1|2/3| where we add another panel to the right
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 6959: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 6959)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_divider_movement_preserves_total_width() {
        // Test that divider movement preserves the total width of the grid
        let margin = Delta::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 7012: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/context/grid.rs` (line 7012)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_issue_panel3_width_changes_when_moving_divider() {
        // Regression test for the specific issue:
        // "if i have two vertical tabs and in the second i have two horizontals.
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym