// 《Rust圣经》const 泛型 练习实践
// 题目：https://practice-zh.course.rs/generics-traits/const-generics.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/generics-traits/const-generics.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- Const 泛型 ---
// 在之前的泛型中，可以抽象为一句话：针对类型实现的泛型，所有的泛型都是为了抽象不同的类型
// 那有没有针对值的泛型？答案就是 Const 泛型
// 示例,下面的例子同时使用泛型和 const 泛型来实现一个结构体，该结构体的字段中的数组长度是可变的
// struct ArrayPair<T, const N: usize> {
//     left: [T; N],
//     right: [T; N],
// }
// impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
//     // ...
// }
// 目前，const 泛型参数只能使用以下形式的实参:
// 一个单独的 const 泛型参数
// 一个字面量 (i.e. 整数, 布尔值或字符).
// 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
// fn foo<const N: usize>() {}
// fn bar<T, const M: usize>() {
//     foo::<M>(); // ok: 符合第一种
//     foo::<2021>(); // ok: 符合第二种
//     foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: 符合第三种
//     foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
//     foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T 
//     let _: [u8; M]; // ok: 符合第一种
//     let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
// }
// fn main() {}
// const 泛型还能避免一些运行时检查，提升性能
// pub struct MinSlice<T, const N: usize> {
//     pub head: [T; N],
//     pub tail: [T],
// }
// fn main() {
//     let slice: &[u8] = b"Hello, world";
//     let reference: Option<&u8> = slice.get(6);
//     // 我们知道 `.get` 返回的是 `Some(b' ')`
//     // 但编译器不知道
//     assert!(reference.is_some());
//     let slice: &[u8] = b"Hello, world";
//     // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
//     // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查    
//     let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
//     let value: u8 = minslice.head[6];
//     assert_eq!(value, b' ')
// }

// test 1
// <T, const N: usize> 是结构体类型的一部分，
// 和数组类型一样，这意味着长度不同会导致类型不同： 
// Array<i32, 3> 和 Array<i32, 4> 是不同的类型
// 题目-修复错误
// struct Array<T, const N: usize> {
//     data : [T; N]
// }
// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1.0, 2.0, 3.0],
//         },
//         Array {
//             data: [1, 2]
//         }
//     ];
// }
// 我的答案
// struct Array<T, const N: usize> {
//     data : [T; N]
// }
// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3]
//         }
//     ];
// }
// 官方答案
// struct Array<T, const N: usize> {
//     data : [T; N]
// }
// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 4]
//         }
//     ];
// }

// test 2
// 题目-填空
// fn print_array<__>(__) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);
//     let arr = ["hello", "world"];
//     print_array(arr);
// }
// 我的答案
// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);
//     let arr = ["hello", "world"];
//     print_array(arr);
// }
// 官方答案
// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);
//     let arr = ["hello", "world"];
//     print_array(arr);
// }

// test 3
// 有时我们希望能限制一个变量占用内存的大小
// 例如在嵌入式环境中，此时 const 泛型参数的第三种形式 const 表达式 就非常适合.
// 题目-#![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }
// // 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]); 
//     check_size([0i32; 191]);
//     check_size(["hello你好"; __]); // size of &str ?
//     check_size([(); __].map(|_| "hello你好".to_string()));  // size of String?
//     check_size(['中'; __]); // size of char ?
// }
// pub enum Assert<const CHECK: bool> {}
// pub trait IsTrue {}
// impl IsTrue for Assert<true> {}
// 我的答案 不会写
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }
// // 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]); 
//     check_size([0i32; 191]);
//     check_size(["hello你好"; 14]); // size of &str ?
//     check_size([(); 0].map(|_| "hello你好".to_string()));  // size of String?
//     check_size(['中'; 4]); // size of char ?
// }
// pub enum Assert<const CHECK: bool> {}
// pub trait IsTrue {}
// impl IsTrue for Assert<true> {}
// 官方答案
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }
// // fix the errors in main
// fn main() {
//     check_size([0u8; 767]); 
//     check_size([0i32; 191]);
//     check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
//     check_size([(); 31].map(|_| "hello你好".to_string()));  // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
//     check_size(['中'; 191]); // A char takes 4 bytes in Rust
// }
// pub enum Assert<const CHECK: bool> {}
// pub trait IsTrue {}
// impl IsTrue for Assert<true> {}