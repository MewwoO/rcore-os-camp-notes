// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/basic-types/statements-expressions.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/basic-types/statements.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// ---示例---
// fn main() {
//     let x = 5u32;
// 
//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;
// 
//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };
// 
//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };
// 
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// test 1
// 题目-使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
//     assert_eq!(v, 3);
// }
// 我的答案 1
// fn main() {
//     let v = {
//         let x = 1;
//         x + 2
//     };
//  
//     assert_eq!(v, 3);
// }
// 我的答案 2
// fn main() {
//     let v = {
// 不加下划线不会err，但会warning
// 强迫症，所以加下划线，这样就看不到warning了
//         let mut _x = 1;
//         _x += 2;
//     };
//  
//     assert_eq!(v, ());
// }
// 官方答案 1
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };
// 好吧，跟我那个写的也差不多
// 重点是区别于语句和表达式
//     assert_eq!(v, 3);
// }
// 官方答案 2
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
// 写的一样！只不过它保留了warning
//     assert_eq!(v, ());
// }

// test 2
// 题目-让代码工作起来
// fn main() {
//     let v = (let x = 3);
//  
//     assert!(v == 3);
// }
// 我的答案
// fn main() {
//     let v = {
//         let x = 3;
//         x
//     };
//  
//     assert!(v == 3);
// }
// 官方答案
// fn main() {
//     let v = {
//         let x = 3;
//         x
//     };
// 嘻嘻！写的一样！！！
//     assert!(v == 3);
// }

// test 3
// 题目-让代码工作起来
// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }
// 
// fn sum(x: i32, y: i32) -> i32 {
//     x + y;
// }
// 我的答案
// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }
// 
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }
// 官方答案
// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }
// 嘻嘻！又写对了！！！
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }