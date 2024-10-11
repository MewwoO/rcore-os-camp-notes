// fn main() {
//     let word = String::from("green"); // Try not changing this line :)
//     if is_a_color_word(word) {
//         println!("That is a color word I know!");
//     } else {
//         println!("That is not a color word I know.");
//     }
// }

// fn is_a_color_word(attempt: String) -> bool {
//     attempt == "green" || attempt == "blue" || attempt == "red"
// }

// fn main() {
//     let s = String::from("hello");  // s 进入作用域
//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效
//     println!("在move进函数后继续使用s: {}",s);
//     let x = 5;                      // x 进入作用域
//     makes_copy(x);                  // x 应该移动函数里，
//                                     // 但 i32 是 Copy 的，所以在后面可继续使用 x
// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn main() {
//     match 1 {
//         // num @ 1 | 2 => { 编译不通过
//         // 因为 num 没有绑定到所有的模式上，只绑定了模式 1
//         // 使用下面这种方式在 Rust 1.53 之前的版本会报错，因为编译器不支持
//         // 1.53 之后就可以
// rustc -V
// rustc 1.81.0 (eeb90cda1 2024-09-04)
//         num @ (1 | 2) => {
//             println!("{}", num);
//         }
//         _ => {}
//     }
// }