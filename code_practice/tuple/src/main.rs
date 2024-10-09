// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/compound-types/tuple.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/tuple.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 元组 ---
// 元组中的元素可以是不同的类型
// 元组的类型签名是 (T1, T2, ...), 这里 T1, T2 是相对应的元组成员的类型.

// test 1
// 题目-填空让代码工作
// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // 元组的成员还可以是一个元组
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }
// 我的答案
// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // 元组的成员还可以是一个元组
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }
// 官方答案
// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // Tuples can be tuple's members
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }

// test 2
// 可以使用索引来获取元组的成员
// 题目-修改合适的地方，让代码工作
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.1, "sunface");
// }
// 我的答案
// fn main() {
//     let t = ("i", "sunface");
//     assert_eq!(t.1, "sunface");
// }
// 官方答案
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface"); // 哎呀 只改索引就可以了
//  }

// test 3
// 过长的元组无法被打印输出
// 题目-修复代码错误
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     println!("too long tuple: {:?}", too_long_tuple);
// }
// 我的答案
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7);
//     println!("too long tuple: {:?}", too_long_tuple);
// }
// 官方答案
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
//     println!("too long tuple: {:?}", too_long_tuple); // 不太理解 为什么去掉一个13就可以
// 在 Rust 编程语言中，元组（tuple）的长度是固定的，并且有一个最大长度限制，这具体取决于编译器的实现和平台架构，
// 但通常元组的最大元素数量是相当可观的（在 Rust 1.41 及以后的版本中，元组可以包含最多 127 个元素）
// }


// test 4
// 使用模式匹配来解构元组
// 题目- 填空
// fn main() {
//     let tup = (1, 6.4, "hello");
//     let __ = tup;
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }
// 我的答案
// fn main() {
//     let tup = (1, 6.4, "hello");
//     let (x, z, y) = tup;
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }
// 官方答案
// fn main() {
//     let tup = (1, 6.4, "hello");
//     let (x, z, y) = tup;
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }

// test 5
// 解构式赋值
// 题目-填空
// fn main() {
//     let (x, y, z);
//     __ = (1, 2, 3);
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }
// 我的答案
// fn main() {
//     let (x, y, z);
//     (y, z, x) = (1, 2, 3);
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }
// 官方答案
// fn main() {
//     let (x, y, z);
//     // fill the blank
//     (y, z, x) = (1, 2, 3);
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }

// test 6
// 元组可以用于函数的参数和返回值
// 题目-填空，需要稍微计算下
// fn main() {
//     let (x, y) = sum_multiply(__);
//     assert_eq!(x, 5);
//     assert_eq!(y, 6);
// }
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }
// 我的答案
// fn main() {
//     let (x, y) = sum_multiply((2, 3));
//     assert_eq!(x, 5);
//     assert_eq!(y, 6);
// }
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }
// 官方答案
// fn main() {
//     let (x, y) = sum_multiply((2, 3));
//     assert_eq!(x, 5);
//     assert_eq!(y, 6);
// }
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }