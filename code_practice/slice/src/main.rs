// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/compound-types/slice.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/slice.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- slice ---
// 切片跟数组相似，但是切片的长度无法在编译期得知，因此无法直接使用切片类型。
// 一个切片引用占用了2个字大小的内存空间
// 该切片引用的第一个字是指向数据的指针，第二个字是切片引用的长度。字的大小取决于处理器架构
// 例如在 x86-64 上，字的大小是 64 位也就是 8 个字节，那么一个切片引用就是 16 个字节大小。
// 切片( 引用 )可以用来借用数组的某个连续的部分，对应的签名是 &[T]，大家可以与数组的签名对比下 [T; Length]。

// test 1
// 题目-修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: [i32] = arr[0..2];
//     let s2: str = "hello, world" as str;
// }
// 我的答案
// fn main() {
//     let arr = [1, 2, 3];
//     let _s1: &[i32] = &arr[0..2];
//     let _s2: &str = "hello, world";
// }
// 官方答案
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];
//     let s2: &str = "hello, world";
// }

// test 2
// 题目-修改数字 `8` 让代码工作
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//     let slice = &arr[..2];
//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。
//     // 如果是数组的话，那下面的 `assert!` 将会通过： 
//     // '中'和'国'是char类型，char类型是Unicode编码，
//     // 大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 8);
// }
// 我的答案
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//     let slice = &arr[..2];
//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。
//     // 如果是数组的话，那下面的 `assert!` 将会通过： 
//     // '中'和'国'是char类型，char类型是Unicode编码，
//     // 大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 16);
//     // 在 Rust 中，切片是一个结构体，它包含了一个指向数据的指针和一个表示长度的字段
//     // （在 32 位系统上通常是两个 usize，即 8 字节；在 64 位系统上也是两个 usize，
//     // 但每个 usize 是 8 字节，总共 16 字节
// }
// 官方答案
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//     let slice = &arr[..2];
//     // TIPS: slice( reference ) IS NOT an array, 
//     // because if it is, then `assert!` will passed: 
//     // each of the two UTF-8 chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
//     assert!(std::mem::size_of_val(&slice) == 16);
// }

// test 3
// 题目-填空让代码工作起来
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     let slice: __ = __;
//     assert_eq!(slice, &[2, 3, 4]);
// }
// 我的答案
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
// }
// 官方答案
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
// }

// --- 字符串切片 ---

// test 4
// 题目-填空，不要再使用 0..2
// fn main() {
//     let s = String::from("hello");
//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[__];
//     assert_eq!(slice1, slice2);
// }
// 我的答案
// fn main() {
//     let s = String::from("hello");
//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[..2];
//     assert_eq!(slice1, slice2);
// }
// 官方答案
// fn main() {
//     let s = String::from("hello");
//     let slice1 = &s[0..2];
//     let slice2 = &s[..2];
//     assert_eq!(slice1, slice2);
// }

// test 5
// 题目-修改代码，让代码工作起来
// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..2];
//     assert!(slice == "你");
// }
// 我的答案
// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..3];
//     assert!(slice == "你");
// }
// 官方答案
// fn main() {
//     let s = "你好，世界";
//     let slice = &s[0..3];
//     assert!(slice == "你");
// }

// test 6
// &String 可以被隐式地转换成 &str 类型.
// 题目-修复所有错误
// fn main() {
//     let mut s = String::from("hello world");
//     // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
//     // 尽管两个类型不一样，但是代码仍然可以工作，
//     // 原因是 `&String` 会被隐式地转换成 `&str` 类型，
//     // 如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
//     let ch = first_character(&s);
//     s.clear(); // error!
//     println!("the first character is: {}", ch);
// }
// fn first_character(s: &str) -> &str {
//     &s[..1]
// }
// 我的答案
// fn main() {
//     let s = String::from("hello world");
//     // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
//     // 尽管两个类型不一样，但是代码仍然可以工作，
//     // 原因是 `&String` 会被隐式地转换成 `&str` 类型，
//     // 如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
//     let ch = first_character(&s);
//     // s.clear(); // error!
//     println!("the first character is: {}", ch);
// }
// fn first_character(s: &str) -> &str {
//     &s[..1]
// }
// 官方答案
// fn main() {
//     let mut s = String::from("hello world");
//     // here, &s is `&String` type, but `first_letter` needs a `&str` type.
//     // it works because `&String` can be implicitly converted to `&str, 
//     // If you want know more ,this is called `Deref` 
//     let letter = first_letter(&s);
//     println!("the first letter is: {}", letter);
//     // 我写的不一样
//     s.clear();
// }
// fn first_letter(s: &str) -> &str {
//     &s[..1]
// }
