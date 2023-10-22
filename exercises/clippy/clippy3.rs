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
// // clippy3.rs
// // 
// // Here's a couple more easy Clippy fixes, so you can see its utility.
// //
// // Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// // I AM NOT DONE

// #[allow(unused_variables, unused_assignments)]
// fn main() {
//     let my_option: Option<()> = None;
//     if my_option.is_none() {
//         my_option.unwrap();
//     }

//     let my_arr = &[
//         -1, -2, -3
//         -4, -5, -6
//     ];
//     println!("My array! Here it is: {:?}", my_arr);

//     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
//     println!("This Vec is empty, see? {:?}", my_empty_vec);

//     let mut value_a = 45;
//     let mut value_b = 66;
//     // Let's swap these two!
//     value_a = value_b;
//     value_b = value_a;
//     println!("value a: {}; value b: {}", value_a, value_b);
// }
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_some() {
        // 处理 Some 的情况
    } else {
        println!("my_option is None"); // 添加一个打印语句以指示 my_option 是 None
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
