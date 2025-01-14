use std::{env, fs, path::PathBuf, process::Command};

// Build system
const CONTRACT_ROOT: [&str; 2] = ["../erc20", "../staking-rewards"];
const CONTRACT_CARGO_TOML: [&str; 2] = ["../erc20/Cargo.toml", "../staking-rewards/Cargo.toml"];
const CONTRACT_LIB_RS: [&str; 2] = ["../erc20/src/lib.rs", "../staking-rewards/src/lib.rs"];
const BUILD_ARGS: [[&str; 4]; 2] = [
    ["build", "--release", "-p", "erc20"],
    ["build", "--release", "-p", "staking-rewards"]
];
const WASM_FILENAME: [&str; 2] = ["erc20.wasm", "staking_rewards.wasm"];
const ORIGINAL_WASM_DIR: &str = "../target/wasm32-unknown-unknown/release";
const NEW_WASM_DIR: &str = "wasm";

/// Watch a contract source files for changes given the contract's cargo.toml & lib.rs paths.
///
/// # Arguments
/// * `contract_index` - the contract's arguments index in the defined global constant variables.
fn watch_contract_changes(contract_index: usize) {
    println!("cargo:rerun-if-changed={}", CONTRACT_CARGO_TOML[contract_index]);
    println!("cargo:rerun-if-changed={}", CONTRACT_LIB_RS[contract_index]);
}

 
fn build_contract(contract_index: usize) {
    let output = Command::new("cargo")
        .current_dir(CONTRACT_ROOT[contract_index])
        .args(&BUILD_ARGS[contract_index])
        .output()
        .expect("Expected to build Wasm contracts");
    assert!(
        output.status.success(),
        "Failed to build Wasm contracts:\n{:?}",
        output
    );
}


fn move_wasm_file(contract_index: usize) {
    let new_wasm_dir = env::current_dir().unwrap().join(NEW_WASM_DIR);
    let _ = fs::create_dir(&new_wasm_dir);
    let original_wasm_file = PathBuf::from(ORIGINAL_WASM_DIR).join(WASM_FILENAME[contract_index]);
    let copied_wasm_file = new_wasm_dir.join(WASM_FILENAME[contract_index]);
    fs::copy(original_wasm_file, copied_wasm_file).unwrap();
}

fn main() {
    for contract_index in 0..CONTRACT_ROOT.len() {
        watch_contract_changes(contract_index);
        build_contract(contract_index);
        move_wasm_file(contract_index);
    } 
}