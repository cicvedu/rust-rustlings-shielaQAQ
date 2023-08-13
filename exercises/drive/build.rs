fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let env_var_value = format!("{}", timestamp);
    println!("cargo:rustc-env=TEST_FOO={}", env_var_value);

    // Set the pass feature conditionally
    #[cfg(feature = "pass")]
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
