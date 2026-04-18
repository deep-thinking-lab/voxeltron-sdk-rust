// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
//! Voxeltron Plugin SDK for Rust
//!
//! This SDK provides the necessary types and macros for building
//! Voxeltron plugins using Rust and compiling to WASM.
//!
//! # Example
//!
//! ```rust
//! use voxeltron_plugin_sdk::{plugin, hook, Context};
//!
//! #[plugin(name = "my-plugin", version = "1.0.0")]
//! pub struct MyPlugin;
//!
//! #[hook("on_deploy_start")]
//! fn on_deploy_start(ctx: Context) -> Result<(), String> {
//!     println!("Deployment started: {:?}", ctx.project_id);
//!     Ok(())
//! }
//! ```

#![no_std]

extern crate alloc;

use alloc::string::String;
use serde::{Deserialize, Serialize};

pub use extism_pdk::*;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
}

/// Context passed to hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Hook name being fired
    pub hook: String,
    /// Project ID (if applicable)
    pub project_id: Option<String>,
    /// Deployment ID (if applicable)
    pub deployment_id: Option<String>,
    /// Additional context data
    pub data: serde_json::Value,
}

impl Context {
    /// Create a new context
    pub fn new(hook: impl Into<String>) -> Self {
        Self {
            hook: hook.into(),
            project_id: None,
            deployment_id: None,
            data: serde_json::Value::Null,
        }
    }

    /// Set project ID
    pub fn with_project_id(mut self, id: impl Into<String>) -> Self {
        self.project_id = Some(id.into());
        self
    }

    /// Set deployment ID
    pub fn with_deployment_id(mut self, id: impl Into<String>) -> Self {
        self.deployment_id = Some(id.into());
        self
    }

    /// Set context data
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self
    }
}

/// Hook result type
pub type HookResult<T> = Result<T, String>;

/// Host functions for plugins to call
pub mod host {
    use super::*;

    /// Log a message
    pub fn log(level: LogLevel, message: &str) {
        unsafe {
            extism::log(level as i32, message.as_ptr(), message.len() as u64);
        }
    }

    /// Get plugin configuration
    pub fn get_config(key: &str) -> Option<String> {
        let ptr = unsafe { extism::config_get(key.as_ptr(), key.len() as u64) };

        if ptr == 0 {
            None
        } else {
            Some(extism::memory::load_string(ptr))
        }
    }

    /// Send a webhook
    pub fn send_webhook(url: &str, body: &str) -> Result<(), String> {
        let url_ptr = extism::memory::store_bytes(url.as_bytes());
        let body_ptr = extism::memory::store_bytes(body.as_bytes());

        let result = unsafe {
            extism::host_send_webhook(
                url_ptr.offset(),
                url.len() as u64,
                body_ptr.offset(),
                body.len() as u64,
            )
        };

        if result == 0 {
            Ok(())
        } else {
            Err("Failed to send webhook".into())
        }
    }

    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    pub enum LogLevel {
        Debug = 0,
        Info = 1,
        Warn = 2,
        Error = 3,
    }

    mod extism {
        use super::*;

        extern "C" {
            pub fn log(level: i32, msg: *const u8, len: u64);
            pub fn config_get(key: *const u8, len: u64) -> u64;
            pub fn host_send_webhook(url: u64, url_len: u64, body: u64, body_len: u64) -> i32;
        }

        pub mod memory {
            use super::*;

            pub fn store_bytes(bytes: &[u8]) -> MemoryHandle {
                let ptr = unsafe { alloc(bytes.len() as u64) };
                unsafe {
                    store_u8(ptr, bytes.as_ptr(), bytes.len() as u64);
                }
                MemoryHandle {
                    offset: ptr,
                }
            }

            pub fn load_string(ptr: u64) -> String {
                let len = unsafe { length(ptr) };
                let mut bytes = alloc::vec![0u8; len as usize];
                unsafe {
                    load_u8(ptr, bytes.as_mut_ptr(), len);
                }
                String::from_utf8(bytes).unwrap_or_default()
            }

            extern "C" {
                fn alloc(len: u64) -> u64;
                fn store_u8(ptr: u64, src: *const u8, len: u64);
                fn load_u8(ptr: u64, dst: *mut u8, len: u64);
                fn length(ptr: u64) -> u64;
            }
        }

        pub struct MemoryHandle {
            pub offset: u64,
        }

        impl MemoryHandle {
            pub fn offset(&self) -> u64 {
                self.offset
            }
        }
    }
}

/// Re-export commonly used items
pub mod prelude {
    pub use crate::host::{self, LogLevel};
    pub use crate::{Context, HookResult, PluginInfo};
    pub use serde::{Deserialize, Serialize};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_builder() {
        let ctx = Context::new("on_deploy_start")
            .with_project_id("proj-123")
            .with_deployment_id("deploy-456")
            .with_data(serde_json::json!({"foo": "bar"}));

        assert_eq!(ctx.hook, "on_deploy_start");
        assert_eq!(ctx.project_id, Some("proj-123".to_string()));
        assert_eq!(ctx.deployment_id, Some("deploy-456".to_string()));
    }
}
