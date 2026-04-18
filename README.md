# Voxeltron Rust Plugin SDK

SDK for building Voxeltron plugins in Rust.

This in-repo SDK surface is Apache-2.0 licensed and should be treated as `0.x` pre-stable while the dedicated public SDK repo is being prepared.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
voxeltron-plugin-sdk = { path = "path/to/sdk/rust" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Quick Start

```rust
use voxeltron_plugin_sdk::prelude::*;
use voxeltron_plugin_sdk::{host, Context, HookResult};

#[no_mangle]
pub extern "C" fn handle_hook(input: &str) -> String {
    let ctx: Context = match serde_json::from_str(input) {
        Ok(ctx) =&gt; ctx,
        Err(e) =&gt; {
            host::log(LogLevel::Error, &amp;format!("Parse error: {}", e));
            return r#"{"error": "Invalid context"}"#.to_string();
        }
    };

    match ctx.hook.as_str() {
        "on_deploy_success" =&gt; on_deploy_success(ctx),
        _ =&gt; Ok(()),
    };

    r#"{"status": "ok"}"#.to_string()
}

fn on_deploy_success(ctx: Context) -&gt; HookResult&lt;()&gt; {
    let project_id = ctx.project_id.as_deref().unwrap_or("unknown");
    host::log(LogLevel::Info, &amp;format!("Deploy succeeded: {}", project_id));
    Ok(())
}
```

## Building

Compile to WASM:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

## API Reference

### Types

- `Context`: Hook context with project_id, deployment_id, data
- `HookResult&lt;T&gt;`: Result type for hooks (Ok(T) or Err(String))
- `PluginInfo`: Plugin metadata

### Host Functions

- `host::log(level, message)`: Log a message
- `host::get_config(key)`: Get configuration value
- `host::send_webhook(url, body)`: Send HTTP webhook

## License

Apache-2.0
