// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/basic-types/functions.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/basic-types/statements.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// test 1
// 题目-使代码运行起来
// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
// 
//     assert_eq!(s, 3);
// }
// 
// fn sum(x, y: i32) {
//     x + y;
// }
// 我的答案
// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
// sum 函数隐式返回（）
//     assert_eq!(s, ());
// }
// rust 函数的参数定义时每个参数的类型都要定义
// fn sum(x: i32, y: i32) {
//     x + y;
// }
// 官方答案
// fn main() {
//     // don't modify the following two lines!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
//
//     assert_eq!(s, 3);
// }
// 增加了返回值
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// test 2
// 题目-使代码运行起来
// fn main() {
//     print();
//  }
//  
//  // 使用另一个类型来替代 i32
//  fn print() -> i32 {
//     println!("hello,world");
//  }
// 我的答案
// fn main() {
//     print();
//  }
//  
//  // 使用另一个类型来替代 i32
//  fn print() -> () {
//     println!("hello,world");
//  }
// 官方答案
// fn main() {
//     print();
// }
// 写对了！
// // replace i32 with another type
// fn print() -> () {
//     println!("hello,world");
// }

// test 3
// 题目-用两种方法求解
// fn main() {
//     never_return();
// }

// fn never_return() -> ! {
//     // 实现这个函数，不要修改函数签名!
// }
// 我的答案 1
// 用两种方法求解
// fn main() {
//     never_return();
// }
// 
// fn never_return() -> ! {
//     // 实现这个函数，不要修改函数签名!
//     panic!("never return")
// }
// 我的答案 2（不会）
// 官方答案 1
// fn main() {
//     never_return();
// }
//  
// fn never_return() -> ! {
//     // implement this function, don't modify fn signatures
//     panic!("I return nothing!"
// }
// 官方答案 2
// fn main() {
//     never_return();
// }
// 举报！超纲！
// use std::thread;
// use std::time;
// 
// fn never_return() -> ! {
//     // implement this function, don't modify fn signatures
//     loop {
//         println!("I return nothing");
//         // sleeping for 1 second to avoid exhausting the cpu resource
//         thread::sleep(time::Duration::from_secs(1))
//     }
// }
