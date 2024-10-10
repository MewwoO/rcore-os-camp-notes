// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/compound-types/enum.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/enum.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 枚举 ---

// test 1
// 在创建枚举时，你可以使用显式的整数设定枚举成员的值
// 题目-修复错误
// enum Number {
//     Zero,
//     One,
//     Two,
// }
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
// // C语言风格的枚举定义
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }
// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     assert_eq!(Number::One, Number1::One);
//     assert_eq!(Number1::One, Number2::One);
// } 
// 我的答案 写偏了
// #[derive(Debug, PartialEq)]
// enum Number {
//     Zero,
//     One,
//     Two,
// }
// #[derive(Debug, PartialEq)]
// enum Number1 {
//     Zero = 0,
//     // One, Two 隐式的分配为 1, 2
//     One,
//     Two,
// }
// // 在 Rust 中是不允许枚举的变体直接关联浮点数类型作为它们的“值”
// // 如需要存储浮点数，应该考虑使用结构体或元组枚举
// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     // Rust中的枚举值不能直接进行跨枚举的比较
//     assert_eq!(Number::One, Number::One);  
//     assert_eq!(Number1::One, Number1::One);  
// } 
// 官方答案
// enum Number {
//     Zero,
//     One,
//     Two,
// }
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
// // C-like enum
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }
// fn main() {
//     // a enum variant can be converted to a integer by `as`
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as u8, Number2::One as u8);
// } 

// test 2
// 枚举成员可以持有各种类型的值
// 题目-填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg1 = Message::Move{__}; // 使用x = 1, y = 2 来初始化
//     let msg2 = Message::Write(__); // 使用 "hello, world!" 来初始化
// } 
// 我的答案
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg1 = Message::Move{x: 1, y: 2}; // 使用x = 1, y = 2 来初始化
//     let msg2 = Message::Write(String::from("hello, world!")); // 使用 "hello, world!" 来初始化
// } 
// 官方答案
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg1 = Message::Move{x: 1, y: 2}; // instantiating with x = 1, y = 2 
//     let msg2 = Message::Write(String::from("hello, world")); // instantiating with "hello, world!"
// } 

// test 3
// 枚举成员中的值可以使用模式匹配来获取
// 题目-仅填空并修复错误
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::Move{x: 1, y: 1};
//     if let Message::Move{__} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("不要让这行代码运行！");
//     }
// } 
// 我的答案
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::Move{x: 1, y: 1};
//     if let Message::Move{x:a, y:b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("不要让这行代码运行！");
//     }
// } 
// 官方答案
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::Move{x: 1, y: 1};
//     if let Message::Move{x: a, y: b} = msg { // 模式匹配
//         assert_eq!(a, b);
//     } else {
//         panic!("NEVER LET THIS RUN！");
//     }
// } 

// test 4
// 使用枚举对类型进行同一化
// 题目-填空，并修复错误
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msgs: __ = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// } 
// fn show_message(msg: Message) {
//     println!("{}", msg);
// }
// 我的答案
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msgs: Vec<Message> = vec![
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// } 
// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }
// 官方答案
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     // 创建了一个固定大小（3个元素）的数组msgs，
//     // 其元素类型为Message枚举。数组被初始化为包含三个Message枚举的变体实例
//     let msgs: [Message; 3] = [ 
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// }
// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

// test 5
// Rust 中没有 null，我们通过 Option<T> 枚举来处理值为空的情况
// 题目-填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     if let __ = six {
//         println!("{}", n)
//     }  
//     panic!("不要让这行代码运行！");
// } 
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         __ => None,
//         __ => Some(i + 1),
//     }
// }
// 我的答案
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     if let Some(n) = six {
//         println!("{}", n)
//     }  
//     panic!("不要让这行代码运行！");
// } 
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
// 官方答案
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     if let Some(n) = six {
//         println!("{}", n);
//         return
//     } 
//     panic!("NEVER LET THIS RUN！");
// } 
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// test 6
// 使用枚举来实现链表.
// 题目-填空，让代码运行
// use crate::List::*;
// enum List {
//     // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
//     Cons(u32, Box<List>),
//     // Nil: 链表中的最后一个节点，用于说明链表的结束
//     Nil,
// }
// // 为枚举实现一些方法
// impl List {
//     // 创建空的链表
//     fn new() -> List {
//         // 因为没有节点，所以直接返回 Nil 节点
//         // 枚举成员 Nil 的类型是 List
//         Nil
//     }
//     // 在老的链表前面新增一个节点，并返回新的链表
//     fn prepend(self, elem: u32) -> __ {
//         Cons(elem, Box::new(self))
//     }
//     // 返回链表的长度
//     fn len(&self) -> u32 {
//         match *self {
//             // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
//             Cons(_, __ tail) => 1 + tail.len(),
//             // 空链表的长度为 0
//             Nil => 0
//         }
//     }
//     // 返回链表的字符串表现形式，用于打印输出
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 // 递归生成字符串
//                 format!("{}, {}", head, tail.__())
//             },
//             Nil => {
//                 format!("Nil")
//             },
//         }
//     }
// }
// fn main() {
//     // 创建一个新的链表(也是空的)
//     let mut list = List::new();
//     // 添加一些元素
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);
//      // 打印列表的当前状态
//     println!("链表的长度是: {}", list.len());
//     println!("{}", list.stringify());
// }
// 我的答案
// use crate::List::*;
// enum List {
//     // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
//     Cons(u32, Box<List>),
//     // Nil: 链表中的最后一个节点，用于说明链表的结束
//     Nil,
// }
// // 为枚举实现一些方法
// impl List {
//     // 创建空的链表
//     fn new() -> List {
//         // 因为没有节点，所以直接返回 Nil 节点
//         // 枚举成员 Nil 的类型是 List
//         Nil
//     }
//     // 在老的链表前面新增一个节点，并返回新的链表
//     fn prepend(self, elem: u32) -> List {
//         Cons(elem, Box::new(self))
//     }
//     // 返回链表的长度
//     fn len(&self) -> u32 {
//         match *self {
//             // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
//             Cons(_, ref tail) => 1 + tail.len(),
//             // 空链表的长度为 0
//             Nil => 0
//         }
//     }
//     // 返回链表的字符串表现形式，用于打印输出
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 // 递归生成字符串
//                 format!("{}, {}", head, tail.len())
//             },
//             Nil => {
//                 format!("Nil")
//             },
//         }
//     }
// }
// fn main() {
//     // 创建一个新的链表(也是空的)
//     let mut list = List::new();
//     // 添加一些元素
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);
//      // 打印列表的当前状态
//     println!("链表的长度是: {}", list.len());
//     println!("{}", list.stringify());
// }
// 官方答案
// use crate::List::*;
// enum List {
//     // Cons: Tuple struct that wraps an element and a pointer to the next node
//     Cons(u32, Box<List>),
//     // Nil: A node that signifies the end of the linked list
//     Nil,
// }
// // Methods can be attached to an enum
// impl List {
//     // Create an empty list
//     fn new() -> List {
//         // `Nil` has type `List`
//         Nil
//     }
//     // Consume a list, and return the same list with a new element at its front
//     fn prepend(self, elem: u32) -> List {
//         // `Cons` also has type List
//         Cons(elem, Box::new(self))
//     }
//     // Return the length of the list
//     fn len(&self) -> u32 {
//         // `self` has to be matched, because the behavior of this method
//         // depends on the variant of `self`
//         // `self` has type `&List`, and `*self` has type `List`, matching on a
//         // concrete type `T` is preferred over a match on a reference `&T`
//         // after Rust 2018 you can use self here and tail (with no ref) below as well,
//         // rust will infer &s and ref tail. 
//         // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
//         match *self {
//             // Can't take ownership of the tail, because `self` is borrowed;
//             // instead take a reference to the tail
//             Cons(_, ref tail) => 1 + tail.len(),
//             // Base Case: An empty list has zero length
//             Nil => 0
//         }
//     }
//     // Return representation of the list as a (heap allocated) string
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 // `format!` is similar to `print!`, but returns a heap
//                 // allocated string instead of printing to the console
//                 format!("{}, {}", head, tail.stringify())
//             }
//             Nil => {
//                 format!("Nil")
//             }
//         }
//     }
// }
// fn main() {
//     // Create an empty linked list
//     let mut list = List::new();
//     // Prepend some elements
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);
//     // Show the final state of the list
//     println!("linked list has length: {}", list.len());
//     println!("{}", list.stringify());
// }
