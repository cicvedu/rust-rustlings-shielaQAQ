// drive5.rs
// Your task is to make the testcase be able to call the `my_demo_function` in module Foo.
// the `my_demo_function_alias` is an alias for `my_demo_function_alias`, so the two line of
// code in the testcase should call the same function.
// You should not modify any existing code. All you need to do is add two line of attributes.

extern {
    fn my_demo_function(a:u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a:u32) -> u32;
}




mod Foo{
    fn my_demo_function(a:u32) -> u32 {a}
}

pub use Foo::my_demo_function;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Foo;

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
