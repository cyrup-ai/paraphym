# `forks/pingora/pingora-core/src/protocols/http/v1/body.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 1b8f92cc  
- **Timestamp**: 2025-10-10T02:16:01.205650+00:00  
- **Lines of Code**: 979

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 979 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 28
  - TODO
  - 

```rust
use crate::utils::BufRef;

// TODO: make this dynamically adjusted
const BODY_BUFFER_SIZE: usize = 1024 * 64;
// limit how much incomplete chunk-size and chunk-ext to buffer
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 134
  - TODO
  - 

```rust
        if !buf_to_rewind.is_empty() {
            self.rewind_buf_len = buf_to_rewind.len();
            // TODO: this is still 1 copy. Make it zero
            body_buf.put_slice(buf_to_rewind);
        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 167
  - TODO
  - 

```rust

    pub fn get_body(&self, buf_ref: &BufRef) -> &[u8] {
        // TODO: these get_*() could panic. handle them better
        buf_ref.get(self.body_buf.as_ref().unwrap())
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 405
  - TODO
  - 

```rust
                match status {
                    httparse::Status::Complete((payload_index, chunk_size)) => {
                        // TODO: Check chunk_size overflow
                        trace!(
                            "Got size {chunk_size}, payload_index: {payload_index}, chunk: {:?}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 683
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"abc";
        let mut mock_io = Builder::new().read(&input[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 686
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 3));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 697
  - stubby variable name
  - mock_io

```rust
        let input1 = b"a";
        let input2 = b"bc";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 700
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 704
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 715
  - stubby variable name
  - mock_io

```rust
        let input1 = b"a";
        let input2 = b""; // simulating close
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 718
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 722
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap_err();
        assert_eq!(&ConnectionClosed, res.etype());
        assert_eq!(body_reader.body_state, ParseState::Done(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 732
  - stubby variable name
  - mock_io

```rust
        let input1 = b"a";
        let input2 = b"bcd";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 735
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 739
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 745
  - stubby variable name
  - mock_io

```rust
        // read remaining data
        body_reader.init_content_length(1, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
        assert_eq!(&input2[2..], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 755
  - stubby variable name
  - mock_io

```rust
        let input1 = b"a";
        let input2 = b"bcd";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(true);
        body_reader.init_content_length(3, b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 758
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(true);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 762
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 774
  - stubby variable name
  - mock_io

```rust
        let rewind = b"ab";
        let input = b"c";
        let mut mock_io = Builder::new().read(&input[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, rewind);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 777
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Partial(2, 1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 781
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(2, 1));
        assert_eq!(rewind, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 792
  - stubby variable name
  - mock_io

```rust
        let input1 = b"a";
        let input2 = b""; // simulating close
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 795
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 799
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(1));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 810
  - stubby variable name
  - mock_io

```rust
        let input1 = b"c";
        let input2 = b""; // simulating close
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(rewind);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 813
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 817
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(2));
        assert_eq!(rewind, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 821
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(3));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 830
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"0\r\n\r\n";
        let mut mock_io = Builder::new().read(&input[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 833
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(0));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 842
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"0;aaaa\r\n\r\n";
        let mut mock_io = Builder::new().read(&input[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 845
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(0));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 856
  - stubby variable name
  - mock_io

```rust
        let ext1 = [b'a'; 1024 * 5];
        let ext2 = [b'a'; 1024 * 3];
        let mut mock_io = Builder::new()
            .read(&chunk_size[..])
            .read(&ext1[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 864
  - stubby variable name
  - mock_io

```rust
        body_reader.init_chunked(b"");
        // read chunk-size, chunk incomplete
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext1, chunk incomplete
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 867
  - stubby variable name
  - mock_io

```rust
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext1, chunk incomplete
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext2, now oversized
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 870
  - stubby variable name
  - mock_io

```rust
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext2, now oversized
        let res = body_reader.read_body(&mut mock_io).await;
        assert!(res.is_err());
        assert_eq!(body_reader.body_state, ParseState::Done(0));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 880
  - stubby variable name
  - mock_io

```rust
        let input1 = b"1\r\na\r\n";
        let input2 = b"0\r\n\r\n";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 883
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 887
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 896
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input1 = b"1\r\na\r\n";
        let mut mock_io = Builder::new().read(&input1[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 899
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 903
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await;
        assert!(res.is_err());
        assert_eq!(body_reader.body_state, ParseState::Done(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 914
  - stubby variable name
  - mock_io

```rust
        let input1 = b"1\r\na\r\n";
        let input2 = b"0\r\n\r\n";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(rewind);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 917
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&rewind[3..4], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 921
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&rewind[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 925
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(2, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 935
  - stubby variable name
  - mock_io

```rust
        let input1 = b"1\r\na\r\n2\r\nbc\r\n";
        let input2 = b"0\r\n\r\n";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 938
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 942
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 6, 13, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(9, 2));
        assert_eq!(&input1[9..11], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 946
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input1[9..11], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(3, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 956
  - stubby variable name
  - mock_io

```rust
        let input1 = b"3\r\na";
        let input2 = b"bc\r\n0\r\n\r\n";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 959
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 963
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 4));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(&input2[0..2], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 967
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input2[0..2], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(3, 4, 9, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 977
  - stubby variable name
  - mock_io

```rust
        let input1 = b"1\r";
        let input2 = b"\na\r\n0\r\n\r\n";
        let mut mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 980
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 983
  - stubby variable name
  - mock_io

```rust
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1)); // input1 concat input2
        assert_eq!(&input2[1..2], body_reader.get_body(&res));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 987
  - stubby variable name
  - mock_io

```rust
        assert_eq!(&input2[1..2], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 6, 11, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 996
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input1 = b"1\r";
        let mut mock_io = Builder::new().read(&input1[..]).build();
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 999
  - stubby variable name
  - mock_io

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1002
  - stubby variable name
  - mock_io

```rust
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
        let res = body_reader.read_body(&mut mock_io).await;
        assert!(res.is_err());
        assert_eq!(body_reader.body_state, ParseState::Done(0));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1011
  - stubby variable name
  - mock_io

```rust
        init_log();
        let output = b"a";
        let mut mock_io = Builder::new().write(&output[..]).build();
        let mut body_writer = BodyWriter::new();
        body_writer.init_content_length(1);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1016
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_writer.body_mode, BodyMode::ContentLength(1, 0));
        let res = body_writer
            .write_body(&mut mock_io, &output[..])
            .await
            .unwrap()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1024
  - stubby variable name
  - mock_io

```rust
        // write again, over the limit
        let res = body_writer
            .write_body(&mut mock_io, &output[..])
            .await
            .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1029
  - stubby variable name
  - mock_io

```rust
        assert_eq!(res, None);
        assert_eq!(body_writer.body_mode, BodyMode::ContentLength(1, 1));
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(1));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1039
  - stubby variable name
  - mock_io

```rust
        let data = b"abcdefghij";
        let output = b"A\r\nabcdefghij\r\n";
        let mut mock_io = Builder::new()
            .write(&output[..])
            .write(&output[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1048
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_writer.body_mode, BodyMode::ChunkedEncoding(0));
        let res = body_writer
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1055
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_writer.body_mode, BodyMode::ChunkedEncoding(data.len()));
        let res = body_writer
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1064
  - stubby variable name
  - mock_io

```rust
            BodyMode::ChunkedEncoding(data.len() * 2)
        );
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, data.len() * 2);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(data.len() * 2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1073
  - stubby variable name
  - mock_io

```rust
        init_log();
        let data = b"a";
        let mut mock_io = Builder::new().write(&data[..]).write(&data[..]).build();
        let mut body_writer = BodyWriter::new();
        body_writer.init_http10();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1078
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(0));
        let res = body_writer
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1085
  - stubby variable name
  - mock_io

```rust
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(1));
        let res = body_writer
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1091
  - stubby variable name
  - mock_io

```rust
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(2));
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, 2);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(2));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 168: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    pub fn get_body(&self, buf_ref: &BufRef) -> &[u8] {
        // TODO: these get_*() could panic. handle them better
        buf_ref.get(self.body_buf.as_ref().unwrap())
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 213: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        S: AsyncRead + Unpin + Send,
    {
        let mut body_buf = self.body_buf.as_deref_mut().unwrap();
        let mut n = self.rewind_buf_len;
        self.rewind_buf_len = 0; // we only need to read rewind data once
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 267: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        S: AsyncRead + Unpin + Send,
    {
        let body_buf = self.body_buf.as_deref_mut().unwrap();
        let mut n = self.rewind_buf_len;
        self.rewind_buf_len = 0; // we only need to read rewind data once
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 304: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                if existing_buf_start == 0 {
                    // read a new buf from IO
                    let body_buf = self.body_buf.as_deref_mut().unwrap();
                    if existing_buf_end == 0 {
                        existing_buf_end = self.rewind_buf_len;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 399: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        buf_index_end: usize,
    ) -> Result<Option<BufRef>> {
        let buf = &self.body_buf.as_ref().unwrap()[buf_index_start..buf_index_end];
        let chunk_status = httparse::parse_chunk_size(buf);
        match chunk_status {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 686: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 3));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 686: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 3));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 700: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 700: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 704: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 704: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 718: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 718: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 735: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 735: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 739: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 739: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 745: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // read remaining data
        body_reader.init_content_length(1, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
        assert_eq!(&input2[2..], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 745: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // read remaining data
        body_reader.init_content_length(1, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
        assert_eq!(&input2[2..], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 758: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(true);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 758: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(true);
        body_reader.init_content_length(3, b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 762: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 762: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(1, 2));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 777: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Partial(2, 1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 777: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_content_length(3, rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::Partial(2, 1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 781: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(2, 1));
        assert_eq!(rewind, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 781: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::Partial(2, 1));
        assert_eq!(rewind, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 795: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 795: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 799: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(1));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 813: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 813: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_http10(rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 817: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(2));
        assert_eq!(rewind, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 817: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(2));
        assert_eq!(rewind, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 1));
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 821: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(body_reader.body_state, ParseState::HTTP1_0(3));
        assert_eq!(input1, body_reader.get_body(&res));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 833: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(0));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 845: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(0));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 864: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        body_reader.init_chunked(b"");
        // read chunk-size, chunk incomplete
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext1, chunk incomplete
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 867: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext1, chunk incomplete
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, Some(BufRef::new(0, 0)));
        // read ext2, now oversized
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 883: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 883: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 887: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 899: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 899: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 917: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&rewind[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 917: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(rewind);
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&rewind[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 921: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&rewind[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 921: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&rewind[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 925: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(2, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 938: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 938: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 942: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 6, 13, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(9, 2));
        assert_eq!(&input1[9..11], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 942: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 6, 13, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(9, 2));
        assert_eq!(&input1[9..11], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 946: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[9..11], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(3, 0, 0, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 959: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 959: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1));
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 963: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 4));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(&input2[0..2], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 963: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input1[3..4], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 0, 0, 4));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 2));
        assert_eq!(&input2[0..2], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 967: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input2[0..2], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(3, 4, 9, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 980: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 980: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 983: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1)); // input1 concat input2
        assert_eq!(&input2[1..2], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 983: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(3, 1)); // input1 concat input2
        assert_eq!(&input2[1..2], body_reader.get_body(&res));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 987: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(&input2[1..2], body_reader.get_body(&res));
        assert_eq!(body_reader.body_state, ParseState::Chunked(1, 6, 11, 0));
        let res = body_reader.read_body(&mut mock_io).await.unwrap();
        assert_eq!(res, None);
        assert_eq!(body_reader.body_state, ParseState::Complete(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 999: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 999: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut body_reader = BodyReader::new(false);
        body_reader.init_chunked(b"");
        let res = body_reader.read_body(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 0));
        assert_eq!(body_reader.body_state, ParseState::Chunked(0, 0, 2, 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1019: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::ContentLength(1, 1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1018: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(&mut mock_io, &output[..])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, 1);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1026: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(&mut mock_io, &output[..])
            .await
            .unwrap();
        assert_eq!(res, None);
        assert_eq!(body_writer.body_mode, BodyMode::ContentLength(1, 1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1029: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, None);
        assert_eq!(body_writer.body_mode, BodyMode::ContentLength(1, 1));
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1029: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, None);
        assert_eq!(body_writer.body_mode, BodyMode::ContentLength(1, 1));
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1051: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, data.len());
        assert_eq!(body_writer.body_mode, BodyMode::ChunkedEncoding(data.len()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1050: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, data.len());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1058: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, data.len());
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1057: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, data.len());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1064: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            BodyMode::ChunkedEncoding(data.len() * 2)
        );
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, data.len() * 2);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(data.len() * 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1064: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            BodyMode::ChunkedEncoding(data.len() * 2)
        );
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, data.len() * 2);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(data.len() * 2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1081: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1080: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, 1);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1088: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1087: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(&mut mock_io, &data[..])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res, 1);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1091: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(2));
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, 2);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1091: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, 1);
        assert_eq!(body_writer.body_mode, BodyMode::HTTP1_0(2));
        let res = body_writer.finish(&mut mock_io).await.unwrap().unwrap();
        assert_eq!(res, 2);
        assert_eq!(body_writer.body_mode, BodyMode::Complete(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 188: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust

    fn finish_body_buf(&mut self, end_of_body: usize, total_read: usize) {
        let body_buf_mut = self.body_buf.as_mut().expect("must have read body buf");
        // remove unused buffer
        body_buf_mut.truncate(total_read);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 671: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 671)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::io::Builder;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 680: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 680)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length() {
        init_log();
        let input = b"abc";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 693: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 693)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length_2() {
        init_log();
        let input1 = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 711: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 711)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length_less() {
        init_log();
        let input1 = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 728: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 728)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length_more() {
        init_log();
        let input1 = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 751: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 751)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length_overread() {
        init_log();
        let input1 = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 770: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 770)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length_rewind() {
        init_log();
        let rewind = b"ab";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 788: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 788)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_http10() {
        init_log();
        let input1 = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 805: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 805)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_http10_rewind() {
        init_log();
        let rewind = b"ab";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 827: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 827)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_zero_chunk() {
        init_log();
        let input = b"0\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 839: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 839)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_chunk_ext() {
        init_log();
        let input = b"0;aaaa\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 851: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 851)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_chunk_ext_oversize() {
        init_log();
        let chunk_size = b"0;";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 876: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 876)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_1_chunk() {
        init_log();
        let input1 = b"1\r\na\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 893: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 893)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_1_chunk_incomplete() {
        init_log();
        let input1 = b"1\r\na\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 909: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 909)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_1_chunk_rewind() {
        init_log();
        let rewind = b"1\r\nx\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 931: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 931)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_multi_chunk() {
        init_log();
        let input1 = b"1\r\na\r\n2\r\nbc\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 952: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 952)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_partial_chunk() {
        init_log();
        let input1 = b"3\r\na";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 973: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 973)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_partial_head_chunk() {
        init_log();
        let input1 = b"1\r";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 993: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 993)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_partial_head_chunk_incomplete() {
        init_log();
        let input1 = b"1\r";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1008: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 1008)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_body_cl() {
        init_log();
        let output = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1035: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 1035)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_body_chunked() {
        init_log();
        let data = b"abcdefghij";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1070: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/body.rs` (line 1070)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_body_http10() {
        init_log();
        let data = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym