// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/basic-types/char-bool-unit.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/basic-types/char-bool.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// ---字符---

// test 1
// 题目-修改2处 `assert_eq!` 让代码工作
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),1); 
// 
//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),3); 
// 
//     println!("Success!")
// } 
// 我的答案
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1), 4); 
// 
//     let c2 = '中';
//     assert_eq!(size_of_val(&c2), 4); 
// 
//     println!("Success!")
// } 
// 官方答案
// use std::mem::size_of_val;
// 
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1), 4);
// 嘻嘻！又写对了！！！
//     let c2 = '中';
//     assert_eq!(size_of_val(&c2), 4);
// } 

// test 2
// 题目-修改一行让代码正常打印
// fn main() {
//     let c1 = "中";
//     print_char(c1);
// } 
// 
// fn print_char(c : char) {
//     println!("{}", c);
// }
// 我的答案
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// } 
// 
// fn print_char(c : char) {
//     println!("{}", c);
// }
// 官方答案
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// }
// 嘻嘻！又写对了！！！
// fn print_char(c: char) {
//     println!("{}", c);
// }

// ---布尔---

// test 3
// 题目-使成功打印
// fn main() {
//     let _f: bool = false;
// 
//     let t = true;
//     if !t {
//         println!("Success!")
//     }
// } 
// 我的答案
// fn main() {
//     let _f: bool = false;
// 
//     let t = false;
//     if !t {
//         println!("Success!")
//     }
// } 
// 官方答案
// fn main() {
//     let _f: bool = false;
// 嘻嘻！又写对了！！！
//     let t = false;
//     if !t {
//         println!("hello, world");
//     }
// } 

// test 4
// 题目-使成功打印
// fn main() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(t, f);
// 
//     println!("Success!")
// }
// 我的答案
// fn main() {
//     let f = true;
//     let t = true || false;
//     assert_eq!(t, f);
// 
//     println!("Success!")
// }
// 官方答案
// 嘻嘻！又写对了！！！
// fn main() {
//     let f = true;
//     let t = true || false;
//     assert_eq!(t, f);
// }

// ---单元类型---

// test 5
// 题目-让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();
// 
//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());
// 
//     println!("Success!")
// }
// 
// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }
// 
// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }
// 我的答案（错的 没写出来 不知道这个是啥类型）
// fn main() {
//     let _v: () = ();
// 
//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());
// 
//     println!("Success!")
// }
// 
// fn implicitly_ret_unit() -> (2,3) {
//     println!("I will return a ()")
// }
// 
// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }
// 官方答案
// fn main() {
//     let v0: () = ();
// 搜噶！原来如此！返回单元类型，取消v变量的不使用和遮蔽
//     let v = (2, 3);
//     assert_eq!(v0, implicitly_ret_unit())
// }
// 
// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }
// 
// // don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }

// test 6
// 题目-让代码工作：修改 `assert!` 中的 `4` 
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 4);
// 
//     println!("Success!")
// }
// 我的答案（错的 不知道咋写 笔记里没记内存大小 难道看漏了？）
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     // 提示：单元类型占用的内存大小是多少？
//     assert!(size_of_val(&unit) == 1);
// 
//     println!("Success!")
// }
// 官方答案
// use std::mem::size_of_val;
// 无了个大语！单元类型不占用任何内存就是占用内存大小为0！！
// 笔记里记了，也没有看漏！纯纯脑子傻！
// fn main() {
//     let unit: () = ();
//     // unit type doesn't occupy any memory space
//     assert!(size_of_val(&unit) == 0);
// }