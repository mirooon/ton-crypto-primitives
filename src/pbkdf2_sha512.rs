use sha2::Sha512;
use pbkdf2::pbkdf2;
use hmac::Hmac;

pub fn pbkdf2_sha512(key: &[u8], salt: &[u8], iterations: u32, key_len: usize) -> Vec<u8> {
    let mut derived_key = vec![0u8; key_len];
    pbkdf2::<Hmac<Sha512>>(key, salt, iterations, &mut derived_key)
        .expect("Failed to generate derived key");
    derived_key
}

#[test]
fn test_pbkdf2_sha512() {
    use std::process::Command;
    use std::env;

    let key = "password";
    let salt = "salt";
    let iterations = 1000;
    let key_len = 64;

    // Rust PBKDF2-SHA512 computation
    let rust_derived_key = pbkdf2_sha512(key.as_bytes(), salt.as_bytes(), iterations, key_len);

    // Construct an absolute path to the TypeScript file
    let mut ts_script_path = env::current_dir().expect("Failed to get current directory");
    ts_script_path.push("src/ts_lib/pbkdf2_sha512.ts");

    // Print the path for debugging purposes
    println!("Using TypeScript script path: {:?}", ts_script_path);

    // Invoke TypeScript script using tsx, passing parameters as arguments
    let output = Command::new("npx")
        .arg("tsx")
        .arg(ts_script_path)
        .arg(key)
        .arg(salt)
        .arg(iterations.to_string())
        .arg(key_len.to_string())
        .output()
        .expect("Failed to execute TypeScript script");

    assert!(
        output.status.success(),
        "TypeScript script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Get and decode the TypeScript output (hexadecimal)
    let ts_derived_key_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let ts_derived_key = hex::decode(&ts_derived_key_hex).expect("Invalid hex output from TypeScript script");

    // Compare the Rust and TypeScript PBKDF2-SHA512 outputs
    assert_eq!(
        rust_derived_key.as_slice(),
        ts_derived_key.as_slice(),
        "PBKDF2-SHA512 outputs do not match"
    );
}
