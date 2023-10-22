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
// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// fn main() {
//     let mut shopping_list: Vec<?> = Vec::new();
//     shopping_list.push("milk");
// }
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
