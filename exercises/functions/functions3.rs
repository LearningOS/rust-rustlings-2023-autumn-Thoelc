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
// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    call_me(20);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
