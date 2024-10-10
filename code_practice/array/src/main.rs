// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/compound-types/array.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/array.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 数组 ---
// 数组的类型是 [T; Length]
// 数组的长度是类型签名的一部分，因此数组的长度必须在编译期就已知

// test 1
// 题目-填空,修改代码
// fn main() {
//     // 使用合适的类型填空
//     let arr: __ = [1, 2, 3, 4, 5];
//     // 修改以下代码，让它顺利运行
//     assert!(arr.len() == 4);
// }
// 我的答案
// fn main() {
//     // 使用合适的类型填空
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // 修改以下代码，让它顺利运行
//     assert!(arr.len() == 5);
// }
// 官方答案
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     assert!(arr.len() == 5);
// }

// test 2
// 题目-填空
// fn main() {
//     // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];
//     // 填空
//     // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//     // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//     assert!(std::mem::size_of_val(&arr) == __);
// }
// 我的答案
// fn main() {
//     // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//     let _arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];
//     // 填空
//     // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//     // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//     assert!(std::mem::size_of_val(&arr) == 12);
// }
// 官方答案
// fn main() {
//     // we can ignore parts of the array type or even the whole type, let the compiler infer it for us
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];
//     // Arrays are stack allocated, `std::mem::size_of_val` return the bytes which array occupies
//     // A char takes 4 byte in Rust: Unicode char
//     assert!(std::mem::size_of_val(&arr) == 12);
// }

// test 3
// 数组中的所有元素可以一起初始化为同一个值
// 题目-填空
// fn main() {
//     // 填空
//     let list: [i32; 100] = __ ;
//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
// }
// 我的答案
// fn main() {
//     // 填空
//     let list: [i32; 100] = [1; 100] ;
//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
// }
// 官方答案
// fn main() {
//     let list: [i32; 100] = [1; 100];
//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
// }

// test 4
// 数组中的所有元素必须是同一类型
// 题目-修复错误
// fn main() {
//     let _arr = [1, 2, '3'];
// }
// 我的答案
// fn main() {
//     let _arr = [1, 2, 3];
// }
// 官方答案
// fn main() {
//     // fix the error
//     let _arr = [1, 2, 3];
// }

// test 5
// 数组的下标索引从 0 开始.
// 题目-修改代码
// fn main() {
//     let arr = ['a', 'b', 'c'];
//     let ele = arr[1]; // 只修改此行来让代码工作
//     assert!(ele == 'a');
// }
// 我的答案
// fn main() {
//     let arr = ['a', 'b', 'c'];
//     let ele = arr[0]; // 只修改此行来让代码工作
//     assert!(ele == 'a');
// }
// 官方答案
// fn main() {
//     let arr = ['a', 'b', 'c'];
//     let ele = arr[0];
//     assert!(ele == 'a');
// }

// test 6
// 越界索引会导致代码的 panic.
// 题目-修复代码中的错误
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let name0 = names.get(0).unwrap();
//     // 但是下标索引就存在越界的风险了
//     let _name1 = &names[2];
// }
// 我的答案
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let _name0 = names.get(0).unwrap();
//     // 但是下标索引就存在越界的风险了
//     let _name1 = &names[1];
// }
// 官方答案
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
//     // `get` returns an Option<T>, it's safe to use
//     let name0 = names.get(0).unwrap();
//     // but indexing is not safe
//     let _name1 = &names[1];
// }