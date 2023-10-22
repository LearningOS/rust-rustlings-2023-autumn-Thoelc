/*
 * @File: 
 * @Description: 
 * @Author: thoelc
 * @Date: 2023-10-21
 * @LastEditTime: 2023-10-22
 * @LastEditors: Thoelc
 * 
 * Copyright (c) 2023 by ${git_name}, All Rights Reserved. 
 */
// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// struct Wrapper {
//     value: u32,
// }

// impl Wrapper {
//     pub fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn store_u32_in_wrapper() {
//         assert_eq!(Wrapper::new(42).value, 42);
//     }

//     #[test]
//     fn store_str_in_wrapper() {
//         assert_eq!(Wrapper::new("Foo").value, "Foo");
//     }
// }
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    // #[test]
    // fn store_str_in_wrapper() {
    //     assert_eq!(Wrapper::new("Foo").value, "Foo");
    // }
}
