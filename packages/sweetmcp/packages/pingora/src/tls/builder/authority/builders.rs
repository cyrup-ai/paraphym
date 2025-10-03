//! Base builder for certificate authority operations
//!
//! This module provides the entry point for creating different types of
//! certificate authority builders (filesystem, keychain, remote).


#![allow(dead_code)]

use std::path::Path;

use super::filesystem::AuthorityFilesystemBuilder;
use super::keychain::AuthorityKeychainBuilder;
use super::remote::AuthorityRemoteBuilder;

/// Builder for certificate authority operations
#[derive(Debug, Clone)]
pub struct AuthorityBuilder {
    name: String,
}

impl AuthorityBuilder {
    /// Create a new authority builder with the specified name
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Work with filesystem-based certificate authority
    pub fn path<P: AsRef<Path>>(self, path: P) -> AuthorityFilesystemBuilder {
        AuthorityFilesystemBuilder {
            name: self.name,
            path: path.as_ref().to_path_buf(),
            common_name: None,
            key_size: 2048,
        }
    }

    /// Work with system keychain certificate authority
    #[must_use]
    pub fn keychain(self) -> AuthorityKeychainBuilder {
        AuthorityKeychainBuilder { name: self.name }
    }

    /// Work with remote certificate authority
    pub fn url<S: Into<String>>(self, url: S) -> AuthorityRemoteBuilder {
        AuthorityRemoteBuilder::new(self.name, url.into())
    }
}
