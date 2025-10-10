# `forks/surrealdb/tests/cli_integration.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: surrealdb
- **File Hash**: bd74c9c6  
- **Timestamp**: 2025-10-10T02:16:01.057831+00:00  
- **Lines of Code**: 1715

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1715 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 116
  - stubby method name
  - tmp_file

```rust
		info!("* Export to file");
		let exported = {
			let exported = common::tmp_file("exported.surql");
			let args =
				format!("export --conn http://{addr} {creds} --ns {ns} --db {db} {exported}");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 293
  - stubby method name
  - tmp_file

```rust
		info!("* Root user can do exports");
		let exported = {
			let exported = common::tmp_file("exported.surql");
			let args = format!(
				"export --conn http://{addr} {creds} --ns {throwaway} --db {throwaway} {exported}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 880
  - stubby method name
  - tmp_file

```rust
		info!("* Can't do imports");
		{
			let tmp_file = common::tmp_file("exported.surql");
			File::create(&tmp_file).expect("failed to create tmp file");
			let args = format!(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 904
  - stubby method name
  - tmp_file

```rust
			// Start the server with an import file containing the following query:
			let query = r#"DEFINE ACCESS OVERWRITE admin ON ROOT TYPE JWT URL "https://www.surrealdb.com/jwks.json";"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 952
  - stubby method name
  - tmp_file

```rust
			"#
			);
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 977
  - stubby method name
  - tmp_file

```rust
				DEFINE USER ON ROOT test PASSWORD "secret"; -- Invalid
			"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 999
  - stubby method name
  - tmp_file

```rust
		{
			// Start the server with an import file which does not exist.
			let import_file = common::tmp_file("import.surql");
			let res = common::start_server_with_import_file(&import_file).await;
			match res {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 116
  - stubby variable name
  - tmp_file

```rust
		info!("* Export to file");
		let exported = {
			let exported = common::tmp_file("exported.surql");
			let args =
				format!("export --conn http://{addr} {creds} --ns {ns} --db {db} {exported}");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 293
  - stubby variable name
  - tmp_file

```rust
		info!("* Root user can do exports");
		let exported = {
			let exported = common::tmp_file("exported.surql");
			let args = format!(
				"export --conn http://{addr} {creds} --ns {throwaway} --db {throwaway} {exported}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 880
  - stubby variable name
  - tmp_file

```rust
		info!("* Can't do imports");
		{
			let tmp_file = common::tmp_file("exported.surql");
			File::create(&tmp_file).expect("failed to create tmp file");
			let args = format!(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 881
  - stubby variable name
  - tmp_file

```rust
		{
			let tmp_file = common::tmp_file("exported.surql");
			File::create(&tmp_file).expect("failed to create tmp file");
			let args = format!(
				"import --conn http://{addr} {creds} --ns {throwaway} --db {throwaway} {tmp_file}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 883
  - stubby variable name
  - tmp_file

```rust
			File::create(&tmp_file).expect("failed to create tmp file");
			let args = format!(
				"import --conn http://{addr} {creds} --ns {throwaway} --db {throwaway} {tmp_file}",
				throwaway = Ulid::new()
			);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 904
  - stubby variable name
  - tmp_file

```rust
			// Start the server with an import file containing the following query:
			let query = r#"DEFINE ACCESS OVERWRITE admin ON ROOT TYPE JWT URL "https://www.surrealdb.com/jwks.json";"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 952
  - stubby variable name
  - tmp_file

```rust
			"#
			);
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 977
  - stubby variable name
  - tmp_file

```rust
				DEFINE USER ON ROOT test PASSWORD "secret"; -- Invalid
			"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 999
  - stubby variable name
  - tmp_file

```rust
		{
			// Start the server with an import file which does not exist.
			let import_file = common::tmp_file("import.surql");
			let res = common::start_server_with_import_file(&import_file).await;
			match res {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1108
  - stubby variable name
  - temp_dir

```rust
	#[test]
	fn validate_found_no_files() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		temp_dir.child("file.txt").touch().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1110
  - stubby variable name
  - temp_dir

```rust
		let temp_dir = assert_fs::TempDir::new().unwrap();

		temp_dir.child("file.txt").touch().unwrap();

		assert!(common::run_in_dir("validate", &temp_dir).output().is_err());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1112
  - stubby variable name
  - temp_dir

```rust
		temp_dir.child("file.txt").touch().unwrap();

		assert!(common::run_in_dir("validate", &temp_dir).output().is_err());
	}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1117
  - stubby variable name
  - temp_dir

```rust
	#[test]
	fn validate_succeed_for_valid_surql_files() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		let statement_file = temp_dir.child("statement.surql");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1119
  - stubby variable name
  - temp_dir

```rust
		let temp_dir = assert_fs::TempDir::new().unwrap();

		let statement_file = temp_dir.child("statement.surql");

		statement_file.touch().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1124
  - stubby variable name
  - temp_dir

```rust
		statement_file.write_str("CREATE thing:success;").unwrap();

		common::run_in_dir("validate", &temp_dir).output().unwrap();
	}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1129
  - stubby variable name
  - temp_dir

```rust
	#[test]
	fn validate_failed_due_to_invalid_glob_pattern() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		const WRONG_GLOB_PATTERN: &str = "**/*{.txt";
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1135
  - stubby variable name
  - temp_dir

```rust
		let args = format!("validate \"{WRONG_GLOB_PATTERN}\"");

		assert!(common::run_in_dir(&args, &temp_dir).output().is_err());
	}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1140
  - stubby variable name
  - temp_dir

```rust
	#[test]
	fn validate_failed_due_to_invalid_surql_files_syntax() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		let statement_file = temp_dir.child("statement.surql");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1142
  - stubby variable name
  - temp_dir

```rust
		let temp_dir = assert_fs::TempDir::new().unwrap();

		let statement_file = temp_dir.child("statement.surql");

		statement_file.touch().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1147
  - stubby variable name
  - temp_dir

```rust
		statement_file.write_str("CREATE $thing WHERE value = '';").unwrap();

		assert!(common::run_in_dir("validate", &temp_dir).output().is_err());
	}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1875
  - stubby variable name
  - temp_file

```rust
		{
			let path = format!("surrealkv:{}", tempfile::tempdir().unwrap().path().display());
			let temp_file = tempfile::NamedTempFile::new().unwrap();
			let res = common::start_server(StartServerArguments {
				path: Some(path),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1879
  - stubby variable name
  - temp_file

```rust
				path: Some(path),
				args: "".to_owned(),
				temporary_directory: Some(format!("{}", temp_file.path().display())),
				..Default::default()
			})
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1892
  - stubby variable name
  - temp_file

```rust
				}
			}
			temp_file.close().unwrap();
		}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1899
  - stubby variable name
  - temp_dir

```rust
		{
			let path = format!("surrealkv:{}", tempfile::tempdir().unwrap().path().display());
			let temp_dir = tempfile::tempdir().unwrap();
			let (_, mut server) = common::start_server(StartServerArguments {
				path: Some(path),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1903
  - stubby variable name
  - temp_dir

```rust
				path: Some(path),
				args: "".to_owned(),
				temporary_directory: Some(format!("{}", temp_dir.path().display())),
				..Default::default()
			})
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1908
  - stubby variable name
  - temp_dir

```rust
			.await
			.unwrap();
			temp_dir.close().unwrap();
			server.finish().unwrap();
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 903
  - hardcoded URL
  - 

```rust
		{
			// Start the server with an import file containing the following query:
			let query = r#"DEFINE ACCESS OVERWRITE admin ON ROOT TYPE JWT URL "https://www.surrealdb.com/jwks.json";"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 914
  - hardcoded URL
  - 

```rust
			let output = common::run(&args).input("INFO FOR ROOT").output().expect("success");
			assert!(output.contains(
				r#"DEFINE ACCESS admin ON ROOT TYPE JWT URL 'https://www.surrealdb.com/jwks.json'"#
			));
			// Modify the resource that was imported.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 919
  - hardcoded URL
  - 

```rust
			common::run(&args)
				.input(
					"DEFINE ACCESS OVERWRITE admin ON ROOT TYPE JWT URL 'https://www.example.com/jwks.json'",
				)
				.output()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 926
  - hardcoded URL
  - 

```rust
			let output = common::run(&args).input("INFO FOR ROOT").output().expect("success");
			assert!(output.contains(
				r#"DEFINE ACCESS admin ON ROOT TYPE JWT URL 'https://www.example.com/jwks.json'"#
			));
			// Restart the server.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 936
  - hardcoded URL
  - 

```rust
			let output = common::run(&args).input("INFO FOR ROOT").output().expect("success");
			assert!(output.contains(
				r#"DEFINE ACCESS admin ON ROOT TYPE JWT URL 'https://www.surrealdb.com/jwks.json'"#
			));
			server.finish().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1261
  - hardcoded URL
  - 

```rust
			);

			let query = "RETURN http::get('http://127.0.0.1/');\n\n";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 264
  - actual
  - 

```rust
		let output = server.kill().output().err().unwrap();

		// Test the crt/key args but the keys are self signed so don't actually connect.
		assert!(output.contains("Started web server"), "couldn't start web server: {output}");
	}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 31: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn version_command() {
		common::run("version").output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 36: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn version_flag_short() {
		common::run("-V").output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 41: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn version_flag_long() {
		common::run("--version").output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 46: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn help_command() {
		common::run("help").output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 51: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn help_flag_short() {
		common::run("-h").output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 56: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn help_flag_long() {
		common::run("--help").output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 78: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
		let creds = ""; // Anonymous user
		let ns = Ulid::new();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 89: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"sql --conn http://{addr} {creds} --ns {ns} --db {db} --multi --hide-welcome"
			);
			let output = common::run(&args).input("CREATE any:any;\n").output().unwrap();
			assert!(output.contains("Development builds are not intended for production use"));
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 98: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"sql --conn http://{addr} {creds} --ns {ns} --db {db} --multi --hide-welcome"
			);
			let output = common::run(&args).input("CREATE thing:one;\n").output().unwrap();
			assert!(output.contains("[[{ id: thing:one }]]\n\n"), "failed to send sql: {args}");
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 139: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"sql --conn http://{addr} {creds} --ns {ns} --db {db2} --pretty --hide-welcome"
			);
			let output = common::run(&args).input("SELECT * FROM thing;\n").output().unwrap();
			let output = remove_debug_info(output);
			let (line1, rest) = output.split_once('\n').expect("response to have multiple lines");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 160: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				)
				.output()
				.unwrap();

			assert!(output.contains("[1, 2, 3]"), "missing success in {output}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 180: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				)
				.output()
				.unwrap();

			assert!(output.contains("thing:success"), "missing success in {output}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 208: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				)
				.output()
				.unwrap();

			assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 259: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();

		std::thread::sleep(std::time::Duration::from_millis(10000));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 262: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

		std::thread::sleep(std::time::Duration::from_millis(10000));
		let output = server.kill().output().err().unwrap();

		// Test the crt/key args but the keys are self signed so don't actually connect.
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 271: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	async fn with_root_auth() {
		// Commands with credentials when auth is enabled, should succeed
		let (addr, mut server) = common::start_server_with_defaults().await.unwrap();
		let creds = format!("--user {USER} --pass {PASS}");
		let sql_args = format!("sql --conn http://{addr} --multi --pretty");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 312: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		}

		server.finish().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 318: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	async fn with_auth_level() {
		// Commands with credentials for different auth levels
		let (addr, mut server) = common::start_server_with_defaults().await.unwrap();
		let creds = format!("--user {USER} --pass {PASS}");
		let ns = Ulid::new();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 510: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			);
		}
		server.finish().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 516: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	async fn with_auth_level_token() {
		// Commands with token for different auth levels
		let (addr, mut server) = common::start_server_with_defaults().await.unwrap();
		let ac = "test";
		let ns = Ulid::new();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 548: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.as_ref(),
		)
		.unwrap();
		let token_root = match encode(&header, &claims_root, &key_enc) {
			Ok(enc) => enc,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 567: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.as_ref(),
		)
		.unwrap();
		let token_ns = match encode(&header, &claims_ns, &key_enc) {
			Ok(enc) => enc,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 587: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.as_ref(),
		)
		.unwrap();
		let token_db = match encode(&header, &claims_db, &key_enc) {
			Ok(enc) => enc,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 608: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.as_ref(),
		)
		.unwrap();
		let token_record = match encode(&header, &claims_record, &key_enc) {
			Ok(enc) => enc,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 839: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		}

		server.finish().unwrap();
	}

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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	async fn with_anon_auth() {
		// Commands without credentials when auth is enabled, should fail
		let (addr, mut server) = common::start_server_with_defaults().await.unwrap();
		let creds = ""; // Anonymous user
		let sql_args = format!("sql --conn http://{addr} --multi --pretty");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 892: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			);
		}
		server.finish().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 905: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let query = r#"DEFINE ACCESS OVERWRITE admin ON ROOT TYPE JWT URL "https://www.surrealdb.com/jwks.json";"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
			let (addr, mut server) =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 906: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
			let (addr, mut server) =
				common::start_server_with_import_file(&import_file).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 908: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			file.write_all(query.as_bytes()).unwrap();
			let (addr, mut server) =
				common::start_server_with_import_file(&import_file).await.unwrap();
			// Define connection arguments.
			let args = format!("sql --conn http://{addr} --user {USER} --pass {PASS}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 929: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			));
			// Restart the server.
			server.finish().unwrap();
			let (addr, mut server) =
				common::start_server_with_import_file(&import_file).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 931: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			server.finish().unwrap();
			let (addr, mut server) =
				common::start_server_with_import_file(&import_file).await.unwrap();
			// Verify that the resource has been recreated correctly.
			let args = format!("sql --conn http://{addr} --user {USER} --pass {PASS}");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				r#"DEFINE ACCESS admin ON ROOT TYPE JWT URL 'https://www.surrealdb.com/jwks.json'"#
			));
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 953: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			);
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
			let (addr, mut server) =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 954: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
			let (addr, mut server) =
				common::start_server_with_import_file(&import_file).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 956: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			file.write_all(query.as_bytes()).unwrap();
			let (addr, mut server) =
				common::start_server_with_import_file(&import_file).await.unwrap();
			// Verify that the file has been imported correctly.
			let args =
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&args).input("SELECT * FROM user").output().expect("success");
			assert!(output.contains(r#"{ id: user:1 }"#));
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 978: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			"#;
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
			let res = common::start_server_with_import_file(&import_file).await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 979: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let import_file = common::tmp_file("import.surql");
			let mut file = File::create(&import_file).unwrap();
			file.write_all(query.as_bytes()).unwrap();
			let res = common::start_server_with_import_file(&import_file).await;
			match res {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 988: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			}
			// Verify that no data has been created on the datastore.
			let (addr, mut server) = common::start_server_with_defaults().await.unwrap();
			let args =
				format!("sql --conn http://{addr} --user {USER} --pass {PASS} --namespace {ns}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 993: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&args).input("INFO FOR ROOT").output().expect("success");
			assert!(!output.contains(r#"DEFINE USER test ON ROOT PASSHASH"#));
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1020: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
		let creds = ""; // Anonymous user

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1034: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				.input("DEFINE TABLE thing TYPE ANY CHANGEFEED 1s;\n")
				.output()
				.unwrap();
			let output = remove_debug_info(output);
			assert_eq!(output, "[NONE]\n\n".to_owned(), "failed to send sql: {args}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1047: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				.input("BEGIN TRANSACTION; CREATE thing:one; COMMIT;\n")
				.output()
				.unwrap();
			let output = remove_debug_info(output);
			assert_eq!(
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				.input("SHOW CHANGES FOR TABLE thing SINCE 0 LIMIT 10;\n")
				.output()
				.unwrap();
			let output = remove_debug_info(output).replace('\n', "");
			let allowed = [
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1086: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				})
				.ok_or(format!("Output didnt match an example output: {output}"))
				.unwrap();
		};

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1099: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				.input("SHOW CHANGES FOR TABLE thing SINCE 0 LIMIT 10;\n")
				.output()
				.unwrap();
			let output = remove_debug_info(output);
			assert_eq!(output, "[[]]\n\n".to_owned(), "failed to send sql: {args}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1103: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert_eq!(output, "[[]]\n\n".to_owned(), "failed to send sql: {args}");
		}
		server.finish().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1108: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn validate_found_no_files() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		temp_dir.child("file.txt").touch().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1110: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		let temp_dir = assert_fs::TempDir::new().unwrap();

		temp_dir.child("file.txt").touch().unwrap();

		assert!(common::run_in_dir("validate", &temp_dir).output().is_err());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1117: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn validate_succeed_for_valid_surql_files() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		let statement_file = temp_dir.child("statement.surql");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1121: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		let statement_file = temp_dir.child("statement.surql");

		statement_file.touch().unwrap();
		statement_file.write_str("CREATE thing:success;").unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1122: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

		statement_file.touch().unwrap();
		statement_file.write_str("CREATE thing:success;").unwrap();

		common::run_in_dir("validate", &temp_dir).output().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1124: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		statement_file.write_str("CREATE thing:success;").unwrap();

		common::run_in_dir("validate", &temp_dir).output().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1129: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn validate_failed_due_to_invalid_glob_pattern() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		const WRONG_GLOB_PATTERN: &str = "**/*{.txt";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1140: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test]
	fn validate_failed_due_to_invalid_surql_files_syntax() {
		let temp_dir = assert_fs::TempDir::new().unwrap();

		let statement_file = temp_dir.child("statement.surql");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1144: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		let statement_file = temp_dir.child("statement.surql");

		statement_file.touch().unwrap();
		statement_file.write_str("CREATE $thing WHERE value = '';").unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1145: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

		statement_file.touch().unwrap();
		statement_file.write_str("CREATE $thing WHERE value = '';").unwrap();

		assert!(common::run_in_dir("validate", &temp_dir).output().is_err());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1153: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test(tokio::test)]
	async fn test_server_graceful_shutdown() {
		let (_, mut server) = common::start_server_with_defaults().await.unwrap();

		info!("* Send SIGINT signal");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1181: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	#[test(tokio::test)]
	async fn test_server_second_signal_handling() {
		let (addr, mut server) = common::start_server_without_auth().await.unwrap();

		// Create a long-lived WS connection so the server don't shutdown gracefully
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1241: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			futures::future::join(async { send_future.await.unwrap_err() }, signal_send_fut).await;

		server.finish().unwrap();
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1254: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1262: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN http::get('http://127.0.0.1/');\n\n";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(
				output.contains("Access to network target '127.0.0.1:80' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1269: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN function() { return '1' };";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(
				output.contains("Scripting functions are not allowed")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1276: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			);

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1288: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1296: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'http::get' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1303: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN function() { return '1' };";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(
				output.contains("Scripting functions are not allowed")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1309: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1321: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1329: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1333: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN function() { return '1' };";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(output.contains("['1']"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1336: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("['1']"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1346: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1354: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN function() { return '1' };";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(
				output.contains("Scripting functions are not allowed")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1360: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1370: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1378: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1381: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1393: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1401: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1404: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1414: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::lowercase('SURREALDB');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::lowercase' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1428: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			);

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1440: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1448: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::lowercase('SURREALDB');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::lowercase' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1455: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1458: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1468: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1476: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::lowercase('SURREALDB');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::lowercase' is not allowed"),
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1486: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1496: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1504: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::lowercase('SURREALDB');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::lowercase' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1511: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1514: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1526: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1534: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::lowercase('SURREALDB');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::lowercase' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1541: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1544: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			assert!(output.contains("[3]"), "unexpected output: {output:?}");

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1556: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1564: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::lowercase('SURREALDB');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::lowercase' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1571: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN string::len('123');\n\n".to_string();
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output.contains("Function 'string::len' is not allowed"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1577: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			);

			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1587: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1595: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1597: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1607: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1615: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1617: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1627: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1635: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1641: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1651: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1659: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1665: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1675: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1683: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1685: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1695: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1703: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1709: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1720: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1728: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1730: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(output.contains("['surrealdb-"), "unexpected output: {output:?}");
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1741: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1749: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1755: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1765: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1773: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = format!("RETURN http::get('http://{addr}/version');\n\n");
			let output = common::run(&cmd).input(&query).output().unwrap();
			assert!(
				output
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1779: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1790: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1798: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN 1;\n\n";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(output.contains("[1]"), "unexpected output: {output:?}");
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1800: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(output.contains("[1]"), "unexpected output: {output:?}");
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1811: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1819: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN 1;\n\n";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(
				output.contains("Not enough permissions to perform this action"),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1824: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				"unexpected output: {output:?}"
			);
			server.finish().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1835: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();

			let cmd = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1843: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

			let query = "RETURN 1;\n\n";
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(output.contains("[1]"), "unexpected output: {output:?}");
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1845: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			let output = common::run(&cmd).input(query).output().unwrap();
			assert!(output.contains("[1]"), "unexpected output: {output:?}");
			server.finish().unwrap();
		}
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1863: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			match res {
				Ok((_, mut server)) => {
					server.finish().unwrap();
					panic!("Should not be ok!");
				}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1875: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		{
			let path = format!("surrealkv:{}", tempfile::tempdir().unwrap().path().display());
			let temp_file = tempfile::NamedTempFile::new().unwrap();
			let res = common::start_server(StartServerArguments {
				path: Some(path),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1885: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			match res {
				Ok((_, mut server)) => {
					server.finish().unwrap();
					panic!("Should not be ok!");
				}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1892: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				}
			}
			temp_file.close().unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1899: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		{
			let path = format!("surrealkv:{}", tempfile::tempdir().unwrap().path().display());
			let temp_dir = tempfile::tempdir().unwrap();
			let (_, mut server) = common::start_server(StartServerArguments {
				path: Some(path),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1907: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			})
			.await
			.unwrap();
			temp_dir.close().unwrap();
			server.finish().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1908: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.await
			.unwrap();
			temp_dir.close().unwrap();
			server.finish().unwrap();
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1909: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.unwrap();
			temp_dir.close().unwrap();
			server.finish().unwrap();
		}
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1922: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
				.input("select * from foo;\n")
				.output()
				.unwrap();
			let output = remove_debug_info(output);
			assert_eq!(output.matches("foo:").count(), 1);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1937: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();

		// Connect the client
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1947: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		// Start a slow query
		let query = "SLEEP 1200ms;\n";
		let _ = common::run(&cmd).input(query).output().unwrap();

		// Extract the stderr
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1950: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

		// Extract the stderr
		let stderr = server.finish().unwrap().stderr();

		// Check the log is present
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym