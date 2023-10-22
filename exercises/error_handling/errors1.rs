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
// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// pub fn generate_nametag_text(name: String) -> Option<String> {
//     if name.is_empty() {
//         // Empty names aren't allowed.
//         None
//     } else {
//         Some(format!("Hi! My name is {}", name))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn generates_nametag_text_for_a_nonempty_name() {
//         assert_eq!(
//             generate_nametag_text("Beyoncé".into()),
//             Ok("Hi! My name is Beyoncé".into())
//         );
//     }

//     #[test]
//     fn explains_why_generating_nametag_text_fails() {
//         assert_eq!(
//             generate_nametag_text("".into()),
//             // Don't change this line
//             Err("`name` was empty; it must be nonempty.".into())
//         );
//     }
// }
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        pub fn generate_nametag_text(name: String) -> Result<String, String> {
            if name.is_empty() {
                Err("`name` was empty; it must be nonempty.".to_string())
            } else {
                Ok(format!("Hi! My name is {}", name))
            }
        }
        
        #[cfg(test)]
        mod tests {
            use super::*;
        
            #[test]
            fn generates_nametag_text_for_a_nonempty_name() {
                assert_eq!(
                    generate_nametag_text("Beyoncé".into()),
                    Ok("Hi! My name is Beyoncé".to_string())
                );
            }
        
            #[test]
            fn explains_why_generating_nametag_text_fails() {
                assert_eq!(
                    generate_nametag_text("".into()),
                    Err("`name` was empty; it must be nonempty.".to_string())
                );
            }
        }
                // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".to_string())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            Err("`name` was empty; it must be nonempty.".to_string())
        );
    }
}
