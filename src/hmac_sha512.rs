use hmac::{Hmac, Mac};
use sha2::Sha512;
use std::error::Error;

/// Computes HMAC-SHA-512 with the given `key` and `data`.
/// The function accepts both `&str` and `Vec<u8>` inputs for `key` and `data`.
pub fn hmac_sha512(key: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut mac = Hmac::<Sha512>::new_from_slice(key.as_ref())?;
    mac.update(data.as_ref());
    Ok(mac.finalize().into_bytes().to_vec())
}

#[test]
fn test_hmac_sha512() {
    use std::process::Command;
    use std::env;

    let key = "supersecretkey";
    let data = "The quick brown fox jumps over the lazy dog";

    // Rust HMAC-SHA512 computation
    let rust_hmac = hmac_sha512(key, data).expect("Failed to compute HMAC-SHA512 in Rust");

    // Construct an absolute path to the TypeScript file
    let mut ts_script_path = env::current_dir().expect("Failed to get current directory");
    ts_script_path.push("src/ts_lib/hmac_sha512.ts");

    // Print the path for debugging purposes
    println!("Using TypeScript script path: {:?}", ts_script_path);

    // Invoke TypeScript script using tsx, passing parameters as arguments
    let output = Command::new("npx")
        .arg("tsx")
        .arg(ts_script_path)
        .arg(key)
        .arg(data)
        .output()
        .expect("Failed to execute TypeScript script");

    assert!(
        output.status.success(),
        "TypeScript script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Get and decode the TypeScript output (hexadecimal)
    let ts_hmac_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let ts_hmac = hex::decode(&ts_hmac_hex).expect("Invalid hex output from TypeScript script");

    // Compare the Rust and TypeScript HMAC-SHA512 outputs
    assert_eq!(
        rust_hmac.as_slice(),
        ts_hmac.as_slice(),
        "HMAC-SHA512 outputs do not match"
    );
}
