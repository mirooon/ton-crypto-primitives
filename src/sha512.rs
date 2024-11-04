use sha2::{Sha512, Digest};

pub fn sha512(source: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(source);
    hasher.finalize().to_vec()
}

#[test]
fn test_sha512() {
    use std::process::Command;
    use std::env;

    let input = "The quick brown fox jumps over the lazy dog";

    // Rust SHA512 computation
    let rust_hash = sha512(input.as_bytes());

    // Construct an absolute path to the TypeScript file
    let mut ts_script_path = env::current_dir().expect("Failed to get current directory");
    ts_script_path.push("src/ts_lib/sha512.ts");

    // Print the path for debugging purposes
    println!("Using TypeScript script path: {:?}", ts_script_path);

    // Invoke TypeScript script using tsx
    let output = Command::new("npx")
        .arg("tsx")
        .arg(ts_script_path)
        .arg(input)
        .output()
        .expect("Failed to execute TypeScript script");

    assert!(
        output.status.success(),
        "TypeScript script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Get and decode the TypeScript output (hexadecimal)
    let ts_hash_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let ts_hash = hex::decode(&ts_hash_hex).expect("Invalid hex output from TypeScript script");

    // Compare the Rust and TypeScript SHA512 outputs
    assert_eq!(
        rust_hash.as_slice(),
        ts_hash.as_slice(),
        "SHA512 outputs do not match"
    );
}
