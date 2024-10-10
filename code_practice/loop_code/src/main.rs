// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/flow-control.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/flow-control.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 流程控制 ---

// --- if/else ---

// test 1
// 题目-填空
// fn main() {
//     let n = 5;
//     if n < 0 {
//         println!("{} is negative", n);
//     } __ n > 0 {
//         println!("{} is positive", n);
//     } __ {
//         println!("{} is zero", n);
//     }
// } 
// 我的答案
// fn main() {
//     let n = 5;
//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// } 
// 官方答案
// fn main() {
//     let n = 5;
//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// } 

// test 2
// if/else 可以用作表达式来进行赋值
// 题目-修复错误
// fn main() {
//     let n = 5;
//     let big_n =
//         if n < 10 && n > -10 {
//             println!(" 数字太小，先增加 10 倍再说");
//             10 * n
//         } else {
//             println!("数字太大，我们得让它减半");
//             n / 2.0 ;
//         }
//     println!("{} -> {}", n, big_n);
// } 
// 我的答案
// fn main() {
//     let n = 5;
//     let big_n =
//         if n < 10 && n > -10 {
//             println!(" 数字太小，先增加 10 倍再说");
//             10 * n
//         } else {
//             println!("数字太大，我们得让它减半");
//             n / 2
//         };
//     println!("{} -> {}", n, big_n);
// } 
// 官方答案
// fn main() {
//     let n = 5;
//     let big_n =
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase ten-fold");
//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");
//             n / 2
//         };
//     println!("{} -> {}", n, big_n);
// }

// --- for ---

// test 3
// for in 可以用于迭代一个迭代器，例如序列 a..b.
// 题目-修改代码
// fn main() {
//     for n in 1..=100 { // 修改此行，让代码工作
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }
// } 
// 我的答案
// fn main() {
//     for n in 1..100 { // 修改此行，让代码工作
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }
// } 
// 官方答案
// fn main() {
//     for n in 1..100 {
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }
// } 

// test 4
// 题目-修复错误，不要新增或删除代码行
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in names {
//         // do something with name...
//     }
//     println!("{:?}", names);
//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//     } 
//     println!("{:?}", numbers);
// } 
// 我的答案
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in &names {
//         // do something with name...
//         println!("{}", name);
//     }
//     println!("{:?}", names);
//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//         println!("{}", n);
//     } 
//     println!("{:?}", numbers);
// } 
// 官方答案
// fn main() {
//     let names = [String::from("liming"), String::from("hanmeimei")];
//     for name in &names {
//         // do something with name...
//     }
//     println!("{:?}", names);
//     let numbers = [1, 2, 3];
//     // the elements in numbers are Copy，so there is no move here
//     for n in numbers {
//         // do something with name...
//     }
//     println!("{:?}", numbers);
// } 

// test 5
// 题目-填空
// fn main() {
//     let a = [4,3,2,1];
//     // 通过索引和值的方式迭代数组 `a` 
//     for (i,v) in a.__ {
//         println!("第{}个元素是{}",i+1,v);
//     }
// }
// 我的答案
// fn main() {
//     let a = [4,3,2,1];
//     // 通过索引和值的方式迭代数组 `a` 
//     for (i,v) in a.iter().enumerate() {
//         println!("第{}个元素是{}",i+1,v);
//     }
// }
// 官方答案
// fn main() {
//     let a = [4, 3, 2, 1];
//     // iterate the indexing and value in 'a'
//     for (i, v) in a.iter().enumerate() {
//         println!("The {}th element is {}", i + 1, v);
//     }
// }

// --- while ---

// test 6
// 当条件为 true 时，while 将一直循环
// 题目-填空，让最后一行的  println! 工作 !
// fn main() {
//     // 一个计数值
//     let mut n = 1;
//     // 当条件为真时，不停的循环
//     while n __ 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//         __;
//     }
//     println!("n 的值是 {}, 循环结束",n);
// }
// 我的答案
// fn main() {
//     // 一个计数值
//     let mut n = 1;
//     // 当条件为真时，不停的循环
//     while n > 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//         n = n + 1;
//     }
//     println!("n 的值是 {}, 循环结束",n);
// }
// 官方答案
// fn main() {
//     // A counter variable
//     let mut n = 1;
//     // Loop while the condition is true
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//         n += 1;
//     }
//     println!("n reached {}, so loop is over", n);
// }

// --- continue and break ---

// test 7
// 使用 break 可以跳出循环
// 题目-填空，不要修改其它代码
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n == 66 {
//            __
//        }
//        n += 1;
//     }
//     assert_eq!(n, 66);
// }
// 我的答案
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n == 66 {
//            break;
//        }
//        n += 1;
//     }
//     assert_eq!(n, 66);
// }
// 官方答案
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n == 66 {
//             break;
//         }
//         n += 1;
//     }
//     assert_eq!(n, 66);
// }

// test 8
// continue 会结束当次循环并立即开始下一次循环
// 题目-填空，不要修改其它代码
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n != 66 {
//            n+=1;
//            __;
//        }
//        __
//     }
//     assert_eq!(n, 66);
// }
// 我的答案
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n != 66 {
//            n += 1;
//            continue;
//        }
//        break;
//     }
//     assert_eq!(n, 66);
// }
// 官方答案
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n != 66 {
//             n += 1;
//             continue;
//         }
//         break;
//     }
//     assert_eq!(n, 66);
// }

// --- loop ---

// test 9
// loop 一般都需要配合 break 或 continue 一起使用
// 题目-填空，不要修改其它代码
// fn main() {
//     let mut count = 0u32;
//     println!("Let's count until infinity!");
//     // 无限循环
//     loop {
//         count += 1;
//         if count == 3 {
//             println!("three");
//             // 跳过当此循环的剩余代码
//             __;
//         }
//         println!("{}", count);
//         if count == 5 {
//             println!("OK, that's enough");
//             __;
//         }
//     }
//     assert_eq!(count, 5);
// }
// 我的答案
// fn main() {
//     let mut count = 0u32;
//     println!("Let's count until infinity!");
//     // 无限循环
//     loop {
//         count += 1;
//         if count == 3 {
//             println!("three");
//             // 跳过当此循环的剩余代码
//             continue;
//         }
//         println!("{}", count);
//         if count == 5 {
//             println!("OK, that's enough");
//             break;
//         }
//     }
//     assert_eq!(count, 5);
// }
// 官方答案
// fn main() {
//     let mut count = 0u32;
//     println!("Let's count until infinity!");
//     // Infinite loop
//     loop {
//         count += 1;
//         if count == 3 {
//             println!("three");
//             // Skip the rest of this iteration
//             continue;
//         }
//         println!("{}", count);
//         if count == 5 {
//             println!("OK, that's enough");

//             break;
//         }
//     }
//     assert_eq!(count, 5);
// }

// test 10
// loop 是一个表达式，因此我们可以配合 break 来返回一个值
// 题目-填空
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             __;
//         }
//     };
//     assert_eq!(result, 20);
// }
// 我的答案
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter*2;
//         }
//     };
//     assert_eq!(result, 20);
// }
// 官方答案
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     assert_eq!(result, 20);
// }

// test 11
// 当有多层循环时，你可以使用 continue 或 break 来控制外层的循环。
// 要实现这一点，外部的循环必须拥有一个标签 'label, 然后在 break 或 continue 时指定该标签
// 题目-填空
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // 这只会跳出 inner1 循环
//                 break 'inner1; // 这里使用 `break` 也是一样的
//             }
//             count += 2;
//         }
//         count += 5;
//         'inner2: loop {
//             if count >= 30 {
//                 break 'outer;
//             }
//             continue 'outer;
//         }
//     }
//     assert!(count == __)
// }
// 我的答案
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // 这只会跳出 inner1 循环
//                 break 'inner1; // 这里使用 `break` 也是一样的
//             }
//             count += 2;
//         }
//         count += 5;
//         'inner2: loop {
//             if count >= 30 {
//                 break 'outer;
//             }
//             continue 'outer;
//         }
//     }
//     assert!(count == 30)
// }
// 官方答案
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // This would break only the inner1 loop
//                 break 'inner1; // `break` is also ok 
//             }
//             count += 2;
//         }
//         count += 5;
//         'inner2: loop {
//             if count >= 30 {
//                 // This breaks the outer loop
//                 break 'outer;
//             }
//             // This will continue the outer loop
//             continue 'outer;
//         }
//     }
//     assert!(count == 30)
// }