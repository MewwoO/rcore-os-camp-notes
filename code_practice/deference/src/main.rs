// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/ownership/borrowing.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/ownership/borrowing.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 引用 ---

// test 1
// 题目-填写空白处
// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = __;
//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }
// 我的答案
// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = &x;
//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }
// 官方答案
// fn main() {
//     let x = 5;
//     // fill the blank
//     let p = &x;
//     println!("the memory address of x is {:p}", p); // one possible output: 0x16fa3ac84
// }

// test 2
// 题目-按要求运行
// fn main() {
//     let x = 5;
//     let y = &x;
//     // 只能修改以下行
//     assert_eq!(5, y);
// }
// 我的答案
// fn main() {
//     let x = 5;
//     let y = &x;
//     // 只能修改以下行
//     assert_eq!(5, *y);
// }
// 官方答案
// fn main() {
//     let x = 5;
//     let y = &x;
//     // modify this line only
//     assert_eq!(5, *y);
// }

// test 3
// 题目-修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//     borrow_object(s)
// }
// fn borrow_object(s: &String) {}
// 我的答案
// fn main() {
//     let mut s = String::from("hello, ");
//     borrow_object(&s)
// }
// fn borrow_object(s: &String) {}
// 官方答案
// fn main() {
//     let mut s = String::from("hello, ");
//     borrow_object(&s)
// }
// fn borrow_object(s: &String) {}

// test 4
// 题目-修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//     push_str(s)
// }
// fn push_str(s: &mut String) {
//     s.push_str("world")
// }
// 我的答案
// fn main() {
//     let mut s = String::from("hello, ");
//     push_str(&mut s)
// }
// fn push_str(s: &mut String) {
//     s.push_str("world")
// }
// 官方答案
// fn main() {
//     let mut s = String::from("hello, ");
//     push_str(&mut s)
// }
// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// test 5
// 题目-填写空白处，让代码工作
// fn main() {
//     let mut s = String::from("hello, ");
//     let p = __;
//     p.push_str("world");
// }
// 我的答案
// fn main() {
//     let mut s = String::from("hello, ");
//     let p = &mut s;
//     p.push_str("world");
// }
// 官方答案
// fn main() {
//     let mut s = String::from("hello, ");
//     // fill the blank to make it work
//     let p = &mut s;
//     p.push_str("world");
// }

// --- ref ---
// ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。

// test 6
// 题目-按要求运行
// fn main() {
//     let c = '中';
//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let __ r2 = c;
//     assert_eq!(*r1, *r2);
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1),get_addr(r2));
// }
// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }
// 我的答案
// fn main() {
//     let c = '中';
//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;
//     assert_eq!(*r1, *r2);
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1),get_addr(r2));
// }
// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }
// 官方答案
// fn main() {
//     let c = '中';
//     let r1 = &c;
//     // fill the blank，dont change other code
//     let ref r2 = c;
//     assert_eq!(*r1, *r2);
//     // check the equality of the two address strings
//     assert_eq!(get_addr(r1),get_addr(r2));
// }
// // get memory address string
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// --- 借用规则 ---

// test 7
// 题目-移除代码某个部分，让它工作，你不能移除整行的代码！
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{}, {}", r1, r2);
// }
// 我的答案
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("{}, {}", r1, r2);
// }
// 官方答案
// fn main() {
//     let s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("{}, {}", r1, r2);
// }

// --- 可变性 ---

// test 8
// 题目-按要求运行
// fn main() {
//     // 通过修改下面一行代码来修复错误
//     let s = String::from("hello, ");
//     borrow_object(&mut s)
// }
// fn borrow_object(s: &mut String) {}
// 我的答案
// fn main() {
//     // 通过修改下面一行代码来修复错误
//     let mut s = String::from("hello, ");
//     borrow_object(&mut s)
// }
// fn borrow_object(s: &mut String) {}
// 官方答案
// fn main() {
//     //fix error by modifying this line
//     let mut s = String::from("hello, ");
//     borrow_object(&mut s)
// }
// fn borrow_object(s: &mut String) {}

// test 9
// 题目-从可变对象借用不可变
// 下面的代码没有任何错误，仅示例
// fn main() {
//     let mut s = String::from("hello, ");
//     borrow_object(&s);
//     s.push_str("world");
// }
// fn borrow_object(s: &String) {}

// --- NLL---

// test 10
// 题目-注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
//     println!("{}",r1); 
// }
// 我的答案
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
//     // println!("{}",r1); 需要注释的代码
// }
// 官方答案
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
//     // println!("{}",r1); 官方注释掉的
// }

// test 11
// 题目-按要求运行
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//     // 你不能同时使用 r1 和 r2
// }
// 我的答案
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//     // 你不能同时使用 r1 和 r2
//     println!("r1 is {}", r1);
// }
// 官方答案
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     // add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
//     // you can't use r1 and r2 at the same time
//     r1.push_str("world");
// }
