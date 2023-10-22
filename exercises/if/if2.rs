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
// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// pub fn foo_if_fizz(fizzish: &str) -> &str {
//     if fizzish == "fizz" {
//         "foo"
//     } else {
//         1
//     }
// }

// // No test changes needed!
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn foo_for_fizz() {
//         assert_eq!(foo_if_fizz("fizz"), "foo")
//     }

//     #[test]
//     fn bar_for_fuzz() {
//         assert_eq!(foo_if_fizz("fuzz"), "bar")
//     }

//     #[test]
//     fn default_to_baz() {
//         assert_eq!(foo_if_fizz("literally anything"), "baz")
//     }
// }
pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
