use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                println!("❌ Set project's name");
                println!("   near-tools new <name>");
                return;
            }
            let name = &args[2];
            create_project(name);
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("❌ Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                     NEAR-TOOLS v0.1                          ║");
    println!("║         Tool for simplified development on NEAR              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Commands:");
    println!("  near-tools new <name>  ———>  create a new project");
    println!("  near-tools help        ———>  show this help");
    println!();
}

fn create_project(name: &str) {
    // Validate project name (letters, numbers, hyphens, underscores only)
    if !is_valid_name(name) {
        println!("❌ Project name may include only letters, numbers, hyphens and underscores");
        return;
    }

    // Check if project already exists
    if Path::new(name).exists() {
        println!("❌ Project '{}' already exists", name);
        return;
    }

    println!("🚀 Creating project '{}'...", name);

    // Create all directories
    create_directories(name);

    // Create all files
    create_cargo_toml(name);
    create_rust_toolchain(name);
    create_lib_rs(name);
    create_deploy_sh(name);

    println!("✅ Project '{}' created!", name);
    println!();
    println!("📁 Project structure:");
    println!("   {}/", name);
    println!("   ├── Cargo.toml           # dependencies and settings");
    println!("   ├── rust-toolchain.toml  # fixed Rust version (1.86.0)");
    println!("   ├── deploy.sh            # build script");
    println!("   ├── README.md            # documentation");
    println!("   └── src/");
    println!("       └── lib.rs           # contract code");
    println!();
    println!("🔨 Build the contract:");
    println!("   cd {}", name);
    println!("   cargo build --target wasm32-unknown-unknown --release");
    println!();
    println!("🧪 Run tests:");
    println!("   cargo test");
    println!();
    println!("🚀 Deploy (replace with your account):");
    println!("   near deploy --wasmFile ./target/wasm32-unknown-unknown/release/{}.wasm --accountId YOUR_ACCOUNT.testnet", name);
}

fn is_valid_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    for c in name.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }
    true
}

fn create_directories(name: &str) {
    fs::create_dir_all(&format!("{}/src", name)).unwrap();
}

fn create_cargo_toml(name: &str) {
    let content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
near-sdk = "5.0.0"

[dev-dependencies]
near-sdk = {{ version = "5.0.0", features = ["test"] }}

[profile.release]
codegen-units = 1
lto = true
opt-level = "z"
"#, name);

    let path = format!("{}/Cargo.toml", name);
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_rust_toolchain(_name: &str) {
    let content = r#"[toolchain]
channel = "1.86.0"
targets = ["wasm32-unknown-unknown"]
"#;

    let path = format!("{}/rust-toolchain.toml", _name);
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_lib_rs(_name: &str) {
    let content = r#"use near_sdk::{near_bindgen, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/// Simple "Hello World" contract for NEAR blockchain
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct HelloContract {
    greeting: String,
}

#[near_bindgen]
impl HelloContract {
    /// Initialize contract with a greeting message
    #[init]
    pub fn new(greeting: String) -> Self {
        Self { greeting }
    }

    /// Get current greeting message
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    /// Update greeting message
    pub fn set_greeting(&mut self, new_greeting: String) {
        self.greeting = new_greeting;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id("alice.near".parse().unwrap());
        builder
    }

    #[test]
    fn test_new() {
        let context = get_context();
        testing_env!(context.build());

        let contract = HelloContract::new("Hello".to_string());
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn test_set_greeting() {
        let context = get_context();
        testing_env!(context.build());

        let mut contract = HelloContract::new("Hello".to_string());
        contract.set_greeting("Hi there!".to_string());
        assert_eq!(contract.get_greeting(), "Hi there!");
    }
}
"#;

    let path = format!("{}/src/lib.rs", _name);
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_deploy_sh(name: &str) {
    let content = format!(r#"#!/bin/bash
# Build script for NEAR contract

echo "🔨 Building contract..."
cargo build --target wasm32-unknown-unknown --release

echo ""
echo "✅ Build complete!"
echo ""
echo "📁 WASM file: ./target/wasm32-unknown-unknown/release/{}.wasm"
echo ""
echo "🚀 To deploy (replace with your account):"
echo "   near deploy --wasmFile ./target/wasm32-unknown-unknown/release/{}.wasm --accountId YOUR_ACCOUNT.testnet"
echo ""
echo "📝 After deployment, initialize the contract:"
echo "   near call YOUR_ACCOUNT.testnet new '{{\"greeting\": \"Hello NEAR!\"}}' --accountId YOUR_ACCOUNT.testnet"
"#, name, name);

    let path = format!("{}/deploy.sh", name);
    let mut file = fs::File::create(path.clone()).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    // Make script executable (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).unwrap();
    }
}
