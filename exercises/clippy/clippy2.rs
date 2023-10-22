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
// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     for x in option {
//         res += x;
//     }
//     println!("{}", res);
// }
fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
