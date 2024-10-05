// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/ownership/ownership.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/ownership/ownership.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// ---所有权---

// test 1
// 题目-使用尽可能多的方法来通过编译
// fn main() {
//     let x = String::from("hello, world");
//     let y = x;
//     println!("{},{}",x,y);
// }
// 我的答案 1
// fn main() {
//     let x = "hello, world";
//     let y = x;
//     println!("{},{}",x,y);
// }
// 我的答案 2
// fn main() {
//     let x = String::from("hello, world");
//     let y = x;
//     println!("{}", y);
// }
// 我的答案 3
// fn main() {
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{}, {}", x, y);
// }
// 官方答案 1
// fn main() {
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{},{}",x,y);
// }
// 官方答案 2
// fn main() {
//     let x = "hello, world";
//     let y = x;
//     println!("{},{}",x,y);
// }
// 官方答案 3
// fn main() {
//     let x = &String::from("hello, world"); // 创建字符串获取引用
//     let y = x;
//     println!("{},{}",x,y);
// }
// 官方答案 4
// fn main() {
//     let x = String::from("hello, world");
//     let y = x.as_str(); // 获取不可变&str
//     println!("{},{}",x,y);
// }

// test 2
// 题目-使编译通过
// // 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//     println!("{}", s2);
// }
// // 只能修改下面的代码!
// fn take_ownership(s: String) {
//     println!("{}", s);
// }
// 我的答案
// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//     println!("{}", s2);
// }
// // 只能修改下面的代码!
// fn take_ownership(s: String) -> String{
//     s
// }
// 官方答案
// Don't modify code in main!
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//     println!("{}", s2);
// }
// // Only modify the code below!
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// test 3
// 题目-使编译通过
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let _s = s.into_bytes();
//     s
// }
// 我的答案
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
// 
// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // 将 String 转换成 Vec 类型
//     let _s = s.clone().into_bytes();
//     s
// }
// 官方答案 1
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     let _s = s.as_bytes();
//     s
// }
// 官方答案 2
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     s
// }

// test 4
// 题目-修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");
//     print_str(s);
//     println!("{}", s);
// }
// fn print_str(s: String)  {
//     println!("{}",s)
// }
// 我的答案
// fn main() {
//     let s = String::from("hello, world");
//     print_str(s.clone());
//     println!("{}", s);
// }
// fn print_str(s: String)  {
//     println!("{}",s)
// }
// 官方答案 1
// fn main() {
//     let s = String::from("hello, world");
//     print_str(s.clone());
//     println!("{}", s);
// }
// fn print_str(s: String)  {
//     println!("{}",s)
// }
// 官方答案 2
// fn main() {
//     let s = String::from("hello, world");
//     print_str(&s);
//     println!("{}", s);
// }
// fn print_str(s: &String)  {
//     println!("{}",s)
// }

// test 5
// 题目-不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = x.clone();
//     println!("{:?}, {:?}", x, y);
// }
// 我的答案
// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }
// 官方答案
// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }

// test 6
// 题目-按要求运行
// fn main() {
//     let s = String::from("hello, ");
//     // 只修改下面这行代码 !
//     let s1 = s;
//     s1.push_str("world")
// }
// 我的答案
// fn main() {
//     let s = String::from("hello, ");
//     // 只修改下面这行代码 !
//     let mut s1 = s.clone();
//     s1.push_str("world")
// }
// 官方答案
// fn main() {
//     let s = String::from("hello, ");
//     // modify this line only !
//     let mut s1 = s;
//     s1.push_str("world")
// }

// test 7
// 题目-按要求运行
// fn main() {
//     let x = Box::new(5);
//     let ...      // 完成该行代码，不要修改其它行！
//     *y = 4;   
//     assert_eq!(*x, 5);
// }
// 我的答案
// fn main() {
//     let x = Box::new(5);
//     let mut y = x.clone();      // 完成该行代码，不要修改其它行！
//     *y = 4;   
//     assert_eq!(*x, 5);
// }
// 官方答案
// fn main() {
//     let x = Box::new(5);    
//     let mut y = Box::new(3);       // implement this line, dont change other lines!
//     *y = 4;
//     assert_eq!(*x, 5);
// }

// ---move---
// 当解构一个变量时
// 可以同时使用 move 和引用模式绑定的方式
// 当这么做时，部分 move 就会发生：
// 变量中一部分的所有权被转移给其它变量，
// 而另一部分我们获取了它的引用。
// 在这种情况下，原变量将无法再被使用，
// 但是它没有转移所有权的那一部分依然可以使用，
// 也就是之前被引用的那部分。
// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//     // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
//     // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
//     let Person { name, ref age } = person;
//     println!("The person's age is {}", age);
//     println!("The person's name is {}", name);
//     // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
//     //println!("The person struct is {:?}", person);
//     // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
//     println!("The person's age from person struct is {}", person.age);
// }

// test 8
// 题目-按要求运行
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     let _s = t.0;
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t);
// }
// 我的答案
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     let _s = t.0;
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
// }
// 官方答案
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     let _s = t.0;
//     // modify this line only, don't use `_s`
//     println!("{:?}", t.1);
// }

// test 9
// 题目-按要求运行
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     // 填空，不要修改其它代码
//     let (__, __) = __;
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }
// 我的答案
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     // 填空，不要修改其它代码
//     let (ref s1, ref s2) = t; // 解构并创建对元组t中每个元素的引用
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }
//官方答案
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     // fill the blanks
//     let (s1, s2) = t.clone();
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }