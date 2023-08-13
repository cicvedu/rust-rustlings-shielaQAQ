fn main() {
    // Get the current timestamp
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    // Set the environment variable TEST_FOO to the current timestamp
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    fn main() {
        // Get the current timestamp
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    
        // Set the environment variable TEST_FOO to the current timestamp
        println!("cargo:rustc-env=TEST_FOO={}", timestamp);
        if cfg!(feature = "pass") {
            println!("cargo:rustc-env=TEST_PASS=1");
        }
    
    }
    
}
