# `forks/surrealdb/crates/core/src/protocol/flatbuffers/value.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 6ae2dd11  
- **Timestamp**: 2025-10-10T02:16:00.678466+00:00  
- **Lines of Code**: 257

---## Tier 1 Infractions 


- Line 99
  - TODO
  - 

```rust
			},
			_ => {
				// TODO: DO NOT PANIC, we just need to modify the Value enum which Mees is
				// currently working on.
				// Table and Closure need to be removed from the Value enum.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 119: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			proto_fb::ValueType::Null => Ok(Value::Null),
			proto_fb::ValueType::Bool => {
				Ok(Value::Bool(input.value_as_bool().expect("Guaranteed to be a Bool").value()))
			}
			proto_fb::ValueType::Int64 => Ok(Value::Number(Number::Int(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 122: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::Int64 => Ok(Value::Number(Number::Int(
				input.value_as_int_64().expect("Guaranteed to be an Int64").value(),
			))),
			proto_fb::ValueType::Float64 => Ok(Value::Number(Number::Float(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 125: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			))),
			proto_fb::ValueType::Float64 => Ok(Value::Number(Number::Float(
				input.value_as_float_64().expect("Guaranteed to be a Float64").value(),
			))),
			proto_fb::ValueType::Decimal => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 128: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			))),
			proto_fb::ValueType::Decimal => {
				let decimal_value = input.value_as_decimal().expect("Guaranteed to be a Decimal");
				Ok(Value::Number(Number::Decimal(Decimal::from_fb(decimal_value)?)))
			}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 132: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::String => {
				let string_value = input.value_as_string().expect("Guaranteed to be a String");
				let value = string_value
					.value()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 135: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
				let value = string_value
					.value()
					.expect("String value is guaranteed to be present")
					.to_string();
				Ok(Value::Strand(Strand::new(value).context("Strand contained null byte")?))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 140: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::Bytes => {
				let bytes_value = input.value_as_bytes().expect("Guaranteed to be Bytes");
				Ok(Value::Bytes(Bytes::from_fb(bytes_value)?))
			}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 145: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			proto_fb::ValueType::RecordId => {
				let record_id_value =
					input.value_as_record_id().expect("Guaranteed to be a RecordId");
				let thing = RecordId::from_fb(record_id_value)?;
				Ok(Value::RecordId(thing))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 151: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			proto_fb::ValueType::Duration => {
				let duration_value =
					input.value_as_duration().expect("Guaranteed to be a Duration");
				let duration = Duration::from_fb(duration_value)?;
				Ok(Value::Duration(duration))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 157: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			proto_fb::ValueType::Datetime => {
				let datetime_value =
					input.value_as_datetime().expect("Guaranteed to be a Datetime");
				let dt = DateTime::<Utc>::from_fb(datetime_value)?;
				Ok(Value::Datetime(Datetime(dt)))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 162: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::Uuid => {
				let uuid_value = input.value_as_uuid().expect("Guaranteed to be a Uuid");
				let uuid = Uuid::from_fb(uuid_value)?;
				Ok(Value::Uuid(uuid))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 167: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::Object => {
				let object_value = input.value_as_object().expect("Guaranteed to be an Object");
				let object = Object::from_fb(object_value)?;
				Ok(Value::Object(object))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 172: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::Array => {
				let array_value = input.value_as_array().expect("Guaranteed to be an Array");
				let array = Array::from_fb(array_value)?;
				Ok(Value::Array(array))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 178: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			proto_fb::ValueType::Geometry => {
				let geometry_value =
					input.value_as_geometry().expect("Guaranteed to be a Geometry");
				let geometry = Geometry::from_fb(geometry_value)?;
				Ok(Value::Geometry(geometry))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 183: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::File => {
				let file_value = input.value_as_file().expect("Guaranteed to be a File");
				let file = File::from_fb(file_value)?;
				Ok(Value::File(file))
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
			}
			proto_fb::ValueType::Regex => {
				let regex_value = input.value_as_regex().expect("Guaranteed to be a Regex");
				let regex = Regex::from_fb(regex_value)?;
				Ok(Value::Regex(regex))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 193: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
			}
			proto_fb::ValueType::Range => {
				let range_value = input.value_as_range().expect("Guaranteed to be a Range");
				let range = Range::from_fb(range_value)?;
				Ok(Value::Range(Box::new(range)))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 264: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
	fn test_flatbuffers_roundtrip_value(#[case] input: Value) {
		let mut builder = flatbuffers::FlatBufferBuilder::new();
		let input_fb = input.to_fb(&mut builder).expect("Failed to convert to FlatBuffer");
		builder.finish_minimal(input_fb);
		let buf = builder.finished_data();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 268: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		let buf = builder.finished_data();
		let value_fb =
			flatbuffers::root::<proto_fb::Value>(buf).expect("Failed to read FlatBuffer");
		let value = Value::from_fb(value_fb).expect("Failed to convert from FlatBuffer");
		assert_eq!(input, value, "Roundtrip conversion failed for input: {:?}", input);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 269: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		let value_fb =
			flatbuffers::root::<proto_fb::Value>(buf).expect("Failed to read FlatBuffer");
		let value = Value::from_fb(value_fb).expect("Failed to convert from FlatBuffer");
		assert_eq!(input, value, "Roundtrip conversion failed for input: {:?}", input);
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


### Line 206: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/protocol/flatbuffers/value.rs` (line 206)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;
	use std::ops::Bound;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 262: `#[rstest]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/protocol/flatbuffers/value.rs` (line 262)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[case::regex(Value::Regex(Regex::from_str("/^[a-z]+$/").unwrap()))]

	fn test_flatbuffers_roundtrip_value(#[case] input: Value) {
		let mut builder = flatbuffers::FlatBufferBuilder::new();
		let input_fb = input.to_fb(&mut builder).expect("Failed to convert to FlatBuffer");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym