// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/pattern-match/match-iflet.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/pattern-match/match.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- match ---

// test 1
// 题目-填空
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         __  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!(__),
//     };
// }
// 我的答案
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }
// 官方答案
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North  => { // matching South or North here
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }

// test 2
// match 是一个表达式，因此可以用在赋值语句中
// 题目-使用 match 表达式填空
// fn main() {
//     let boolean = true;
//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = __;
//     assert_eq!(binary, 1);
// }
// 我的答案
// fn main() {
//     let boolean = true;
//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 0,
//     };
//     assert_eq!(binary, 1);
// }
// 官方答案
// fn main() {
//     let boolean = true;
//     // fill the blank with an match expression:
//     //
//     // boolean = true => binary = 1
//     // boolean = false =>  binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 0
//     };
//     assert_eq!(binary, 1);
// }

// test 3
// 使用 match 匹配出枚举成员持有的值
// 题目-填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// } 
// fn show_message(msg: Message) {
//     match msg {
//         __ => { // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, __);
//             assert_eq!(b, __);
//         }
//         __ => println!("no data in these variants")
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
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// } 
// fn show_message(msg: Message) {
//     match msg {
//         Message::Move{x:a, y:b} => { // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants")
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
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// } 
// fn show_message(msg: Message) {
//     match msg {
//         Message::Move{x: a, y: b} => { // match  Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(_, g, b) => { // 这里把 r 省略掉了
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants")
//     }
// }

// --- matches! ---
// matches! 看起来像 match, 但是它可以做一些特别的事情

// test 4
// 题目-使用 `matches` 填空
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(__)
//     }
// } 
// 我的答案
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
//     }
// } 
// 官方答案
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
//     // fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
//     }
// } 

// test 5
// 题目-修复错误
// enum MyEnum {
//     Foo,
//     Bar
// }
// fn main() {
//     let mut count = 0;
//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if e == MyEnum::Foo { // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }
//     assert_eq!(count, 2);
// }
// 我的答案
// enum MyEnum {
//     Foo,
//     Bar
// }
// fn main() {
//     let mut count = 0;
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo) { // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }
//     assert_eq!(count, 2);
// }
// 官方答案
// enum MyEnum {
//     Foo,
//     Bar
// }
// fn main() {
//     let mut count = 0;
//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if matches!(e , MyEnum::Foo) { // fix the error with changing only this line
//             count += 1;
//         }
//     }
//     assert_eq!(count, 2);
// }

// ---if let---
// 在有些时候, 使用 match 匹配枚举有些太重了，此时 if let 就非常适合.

// test 6
// 题目-替代
// fn main() {
//     let o = Some(7);
//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     match o {
//         Some(i) => {
//             println!("This is a really long string and `{:?}`", i);
//         }
//         _ => {}
//     };
// }
// 我的答案
// fn main() {
//     let o = Some(7);
//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     // match o {
//     //     Some(i) => {
//     //         println!("This is a really long string and `{:?}`", i);
//     //     }
//     //     _ => {}
//     // };
//     if let Some(i) = o { // 尝试从o中取值，如果o是some, 则将其内部的值赋给i
//         println!("This is a really long string and `{:?}`", i);
//     };
// }
// 官方答案
// fn main() {
//     let o = Some(7);
//     if let Some(i) = o {
//         println!("This is a really long string and `{:?}`", i);
//     }
// }

// test 7
// 题目-填空
// enum Foo {
//     Bar(u8)
// }
// fn main() {
//     let a = Foo::Bar(1);
//     __ {
//         println!("foobar 持有的值是: {}", i);
//     }
// }
// 我的答案
// enum Foo {
//     Bar(u8)
// }
// fn main() {
//     let a = Foo::Bar(1);
//     if let Foo::Bar(i) = a {
//         println!("foobar 持有的值是: {}", i);
//     }
// }
// 官方答案
// enum Foo {
//     Bar(u8)
// }
// fn main() {
//     let a = Foo::Bar(1);
//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value: {}", i);
//     }
// }

// test 8
// 题目-移除代码
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }
// fn main() {
//     let a = Foo::Qux(10);
//     // 移除以下代码，使用 `match` 代替
//     if let Foo::Bar = a {
//         println!("match foo::bar")
//     } else if let Foo::Baz = a {
//         println!("match foo::baz")
//     } else {
//         println!("match others")
//     }
// }
// 我的答案
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }
// fn main() {
//     let a = Foo::Qux(10);
//     // 移除以下代码，使用 `match` 代替
//     // if let Foo::Bar = a {
//     //     println!("match foo::bar")
//     // } else if let Foo::Baz = a {
//     //     println!("match foo::baz")
//     // } else {
//     //     println!("match others")
//     // }
//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match others"),
//     };
// }
// 官方答案
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }
// fn main() {
//     let a = Foo::Qux(10);
//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ =>  println!("match others")
//     }
// }

// --- 变量遮蔽(Shadowing) ---

// test 9
// 题目-修复错误
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
//        assert_eq!(age, Some(30));
//     } // 新的 `age` 变量在这里超出作用域
//     match age {
//         // `match` 也能实现变量遮蔽
//         Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
//         _ => ()
//     }
// }
// 我的答案
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
//        assert_eq!(age, 30);
//     } // 新的 `age` 变量在这里超出作用域
//     match age {
//         // `match` 也能实现变量遮蔽
//         Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
//         _ => ()
//     }
// }
// 官方答案
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // create a new variable with the same name as previous `age`
//        assert_eq!(age, 30);
//     } // the new variable `age` goes out of scope here
//     match age {
//         // match can also introduce a new shadowed variable
//         Some(age) =>  println!("age is a new variable, it's value is {}",age),
//         _ => ()
//     }
// }
