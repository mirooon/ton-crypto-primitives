use rand::{rngs::OsRng, Rng};

/// Generates a vector of secure random bytes of the specified size.
pub fn get_secure_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = OsRng; // Secure random number generator
    let mut bytes = vec![0u8; size];
    rng.fill(&mut bytes[..]); // Fill the bytes with random data
    bytes
}

/// Generates a vector of secure random words (16-bit integers) of the specified size.
pub fn get_secure_random_words(size: usize) -> Vec<u16> {
    let mut rng = OsRng; // Secure random number generator
    let mut words = vec![0u16; size];
    rng.fill(&mut words[..]); // Fill the words with random data
    words
}

#[test]
fn test_get_secure_random_bytes() {
    use std::process::Command;
    use std::env;

    let byte_length = 32;

    // Rust secure random bytes generation
    let rust_random_bytes = get_secure_random_bytes(byte_length);

    // Construct an absolute path to the TypeScript file
    let mut ts_script_path = env::current_dir().expect("Failed to get current directory");
    ts_script_path.push("src/ts_lib/get_secure_random.ts");

    // Print the path for debugging purposes
    println!("Using TypeScript script path: {:?}", ts_script_path);

    // Invoke TypeScript script using tsx, passing the byte length as an argument
    let output = Command::new("npx")
        .arg("tsx")
        .arg(ts_script_path)
        .arg(byte_length.to_string())
        .output()
        .expect("Failed to execute TypeScript script");

    assert!(
        output.status.success(),
        "TypeScript script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Get and decode the TypeScript output (hexadecimal)
    let ts_random_bytes_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let ts_random_bytes = hex::decode(&ts_random_bytes_hex).expect("Invalid hex output from TypeScript script");

    // Check that both outputs have the correct length
    assert_eq!(
        rust_random_bytes.len(),
        ts_random_bytes.len(),
        "Length of random bytes does not match"
    );

    // Optional: Additional checks to verify randomness, format, etc.
    println!("Rust Random Bytes: {:?}", rust_random_bytes);
    println!("TS Random Bytes: {:?}", ts_random_bytes);
}

#[test]
fn test_get_secure_random_words() {
    let word_length = 16;

    // Rust secure random words generation
    let rust_random_words = get_secure_random_words(word_length);

    // Check the length of the generated random words
    assert_eq!(
        rust_random_words.len(),
        word_length,
        "Length of random words does not match"
    );

    // Optional: Additional checks to verify format or range, etc.
    println!("Rust Random Words: {:?}", rust_random_words);
}
