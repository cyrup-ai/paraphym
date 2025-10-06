use base64::Engine;
use extism_pdk::*;
use log::{debug, trace, warn};
use serde_json::Value;
use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};
use sweetmcp_plugin_builder::prelude::*;
use sweetmcp_plugin_builder::{CallToolResult, Ready};

/// Hash computation logic
fn compute_hash(data: &str, algorithm: &str) -> Result<String, String> {
    debug!("Hashing {} bytes with algorithm: {}", data.len(), algorithm);
    
    match algorithm {
        "sha256" => {
            debug!("Using SHA-256 hash algorithm");
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            let result = format!("{:x}", hasher.finalize());
            trace!("SHA-256 hash computed: {} chars", result.len());
            Ok(result)
        }
        "sha512" => {
            debug!("Using SHA-512 hash algorithm");
            let mut hasher = Sha512::new();
            hasher.update(data.as_bytes());
            let result = format!("{:x}", hasher.finalize());
            trace!("SHA-512 hash computed: {} chars", result.len());
            Ok(result)
        }
        "sha384" => {
            debug!("Using SHA-384 hash algorithm");
            let mut hasher = Sha384::new();
            hasher.update(data.as_bytes());
            let result = format!("{:x}", hasher.finalize());
            trace!("SHA-384 hash computed: {} chars", result.len());
            Ok(result)
        }
        "sha224" => {
            debug!("Using SHA-224 hash algorithm");
            let mut hasher = Sha224::new();
            hasher.update(data.as_bytes());
            let result = format!("{:x}", hasher.finalize());
            trace!("SHA-224 hash computed: {} chars", result.len());
            Ok(result)
        }
        "sha1" => {
            warn!("SHA-1 is cryptographically weak and deprecated, consider using SHA-256 or higher");
            debug!("Using SHA-1 hash algorithm");
            let mut hasher = Sha1::new();
            hasher.update(data.as_bytes());
            let result = format!("{:x}", hasher.finalize());
            trace!("SHA-1 hash computed: {} chars", result.len());
            Ok(result)
        }
        "md5" => {
            warn!("MD5 is cryptographically weak and broken, consider using SHA-256 or higher");
            debug!("Using MD5 hash algorithm");
            let digest = md5::compute(data.as_bytes());
            let result = format!("{:x}", digest);
            trace!("MD5 hash computed: {} chars", result.len());
            Ok(result)
        }
        "base64" => {
            debug!("Encoding data in base64 format");
            trace!("Starting base64 encoding for {} bytes", data.len());
            let encoded = base64::engine::general_purpose::STANDARD.encode(data.as_bytes());
            trace!("Base64 encoding complete: {} chars", encoded.len());
            Ok(encoded)
        }
        "base32" => {
            debug!("Encoding data in base32 format");
            trace!("Starting base32 encoding for {} bytes", data.len());
            let encoded =
                base32::encode(base32::Alphabet::Rfc4648 { padding: true }, data.as_bytes());
            trace!("Base32 encoding complete: {} chars", encoded.len());
            Ok(encoded)
        }
        _ => {
            warn!("Unsupported algorithm requested: {}", algorithm);
            Err(format!("Unsupported algorithm: {}", algorithm))
        }
    }
}

/// Hash tool using plugin-builder
struct HashTool;

impl McpTool for HashTool {
    const NAME: &'static str = "hash";

    fn description(builder: DescriptionBuilder) -> DescriptionBuilder {
        builder
            .does("Generate cryptographic hashes and encoded formats from input data")
            .when("you need to create SHA hashes for security verification (sha256, sha512, sha384, sha224, sha1)")
            .when("you need to generate MD5 checksums for file integrity")
            .when("you need to encode data in base64 format for transmission")
            .when("you need to encode data in base32 format for URLs or identifiers")
            .when("you need to verify data integrity before storage or transmission")
            .perfect_for("data integrity checks, password verification, API authentication, and encoding binary data for text protocols")
    }

    fn schema(builder: SchemaBuilder) -> Value {
        builder
            .required_string("data", "data to convert to hash or encoded format")
            .required_enum(
                "algorithm",
                "algorithm to use for hashing or encoding",
                &[
                    "sha256", "sha512", "sha384", "sha224", "sha1", "md5", "base32", "base64",
                ],
            )
            .build()
    }

    fn execute(args: Value) -> Result<CallToolResult, Error> {
        let data = args
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::msg("data parameter required"))?;

        let algorithm = args
            .get("algorithm")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::msg("algorithm parameter required"))?;

        match compute_hash(data, algorithm) {
            Ok(result) => Ok(ContentBuilder::text(result)),
            Err(e) => Err(Error::msg(e)),
        }
    }
}

/// Create the plugin instance
#[allow(dead_code)]
fn plugin() -> McpPlugin<Ready> {
    mcp_plugin("hash")
        .description("Cryptographic hashing and encoding operations with support for SHA family, MD5, base64, and base32")
        .tool::<HashTool>()
        .serve()
}

// Generate standard MCP entry points
sweetmcp_plugin_builder::generate_mcp_functions!(plugin);
