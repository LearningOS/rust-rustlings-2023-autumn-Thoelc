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
// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);

// #[derive(PartialEq, Debug)]
// enum CreationError {
//     Negative,
//     Zero,
// }

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         // Hmm...? Why is this only returning an Ok value?
//         Ok(PositiveNonzeroInteger(value as u64))
//     }
// }

// #[test]
// fn test_creation() {
//     assert!(PositiveNonzeroInteger::new(10).is_ok());
//     assert_eq!(
//         Err(CreationError::Negative),
//         PositiveNonzeroInteger::new(-10)
//     );
//     assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
// }
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Err(CreationError::Zero)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
