// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/pattern-match/patterns.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/pattern-match/patterns.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 模式 ---

// test 1
// 使用 | 可以匹配多个值, 而使用 ..= 可以匹配一个闭区间的数值序列
// 题目-填空
// fn main() {}
// fn match_number(n: i32) {
//     match n {
//         // 匹配一个单独的值
//         1 => println!("One!"),
//         // 使用 `|` 填空，不要使用 `..` 或 `..=`
//         __ => println!("match 2 -> 5"),
//         // 匹配一个闭区间的数值序列
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match 11 -> +infinite")
//         }
//     }
// }
// 我的答案
// fn main() {}
// fn match_number(n: i32) {
//     match n {
//         // 匹配一个单独的值
//         1 => println!("One!"),
//         // 使用 `|` 填空，不要使用 `..` 或 `..=`
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         // 匹配一个闭区间的数值序列
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match 11 -> +infinite")
//         }
//     }
// }
// 官方答案
// fn main() {}
// fn match_number(n: i32) {
//     match n {
//         // match a single value
//         1 => println!("One!"),
//         // fill in the blank with `|`, DON'T use `..` ofr `..=`
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         // match an inclusive range
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match 11 -> +infinite")
//         }
//     }
// }

// test 2
// @ 操作符可以让我们将一个与模式相匹配的值绑定到新的变量上
// 题目-填空
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     // 填空，让 p 匹配第二个分支
//     let p = Point { x: __, y: __ };
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // 第二个分支
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }
// 我的答案
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     // 填空，让 p 匹配第二个分支
//     let p = Point { x: 0, y: 10 };
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // 第二个分支
//         // 匹配 y 字段是否为 10 或 20 或 30 并将其值绑定在变量 y 上
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }
// 官方答案
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     // fill in the blank to let p match the second arm
//     let p = Point { x: 2, y: 20 }; // x can be [0, 5], y can be 10 20 or 30
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // second arm
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }

// test 3
// 题目- 修复错误
// enum Message {
//     Hello { id: i32 },
// }
// fn main() {
//     let msg = Message::Hello { id: 5 };
//     match msg {
//         Message::Hello {
//             id:  3..=7,
//         } => println!("id 值的范围在 [3, 7] 之间: {}", id),
//         Message::Hello { id: newid@10 | 11 | 12 } => {
//             println!("id 值的范围在 [10, 12] 之间: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }
// 我的答案
// enum Message {
//     Hello { id: i32 },
// }
// fn main() {
//     let msg = Message::Hello { id: 5 };
//     match msg {
//         Message::Hello {
//             id:  3..=7,
//         } => println!("id 值的范围在 [3, 7] 之间"),
//         Message::Hello { id: newid@10..=12 } => {
//             println!("id 值的范围在 [10, 12] 之间: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }
// 官方答案
// enum Message {
//     Hello { id: i32 },
// }
// fn main() {
//     let msg = Message::Hello { id: 5 };
//     match msg {
//         Message::Hello {
//             id:  id@3..=7, // 用了绑定 我是直接删掉输出了
//         } => println!("Found an id in range [3, 7]: {}", id),
//         Message::Hello { id: newid@(10 | 11 | 12) } => { // 这里加了括号,我是直接连续加了..
//             println!("Found an id in another range [10, 12]: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }

// test 4
// 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件
// 题目-填空让代码工作，必须使用 `split`
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) __ => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
// }
// 我的答案
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
// }
// 官方答案
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
// }

// test 5
// 使用 .. 忽略一部分值
// 题目-填空，让代码工作
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//     match numbers {
//         __ => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }
// }
// 我的答案
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//     match numbers {
//         (first, .., last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }
// }
// 官方答案
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//     match numbers {
//         (first,..,last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }
// }

// test 6
// 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用
// 题目-修复错误，尽量少地修改代码,不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//     match r {
//        &mut value => value.push_str(" world!") 
//     }
// }
// 我的答案
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//     match r {
//        value => value.push_str(" world!") 
//     }
// }
// 官方答案
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//     match r {
//         // The type of value is &mut String
//         value => value.push_str(" world!") 
//     }
// }