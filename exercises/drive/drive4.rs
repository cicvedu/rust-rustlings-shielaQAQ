// drive4.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.



// This execrise shares build.rs with the previous exercise.
// You need to add some code to build.rs to make both this exercise and
// the previous one work.

fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let env_var_value = format!("{}", timestamp);
    println!("cargo:rustc-env=TEST_FOO={}", env_var_value);

    // Set the pass feature conditionally
    #[cfg(feature = "pass")]
    println!("cargo:rustc-cfg=feature=\"pass\"");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
