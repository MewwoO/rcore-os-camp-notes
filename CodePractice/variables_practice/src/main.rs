// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/variables.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/variables.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// ---绑定和可变性---

// test 1
// 题目-修复下面代码的错误并尽可能少的修改
// fn main() {
//     let x: i32; // 未初始化，但被使用
//     let y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x);
// }
// 我的答案
// fn main() {
//     let x: i32 = 7; // 未初始化，但被使用
//     let _y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x); 
// }
// 官方答案
// fn main() {
//     let x: i32 = 5; // uninitialized but using, ERROR !
// 这里 y 没有被初始化也没有被使用，只会warning
//     let y: i32; // uninitialized but also unusing, only warning
//     println!("{} is equal to 5", x);
// }

// test 2
// 题目-完形填空，让代码编译
// fn main() {
//     let __ = 1;
//     __ += 2; 
//  
//     println!("x = {}", x); 
// }
// 我的答案
// fn main() {
//     let mut x = 1;
//     x += 2; 
//
//     println!("x = {}", x); 
// }
// 官方答案
// fn main() {
//     let mut x = 1;
//     x += 2;
// 差评！答案的输出语句跟题目都没有对上
//     println!("{} is equal to 3", x);
// }

// ---变量作用域---

// test 3
// 题目-修复下面代码的错误并使用尽可能少的改变
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}, y 的值是 {}", x, y);
// }
// 我的答案
// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 5;
//     println!("x 的值是 {}, y 的值是 {}", x, y);
//     println!("x 的值是 {}, y 的值是 {}", x, y);  
// }
// 官方答案
// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 20;
//     {
// 我是直接删除这个作用域了，官方是在外层声明变量y，并在此处做了变量遮蔽
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is {}", x, y); 
// }

// test 4
// 题目-修复错误
// fn main() {
//     println!("{}, world", x); 
// }
// 
// fn define_x() {
//     let x = "hello";
// }
// 我的答案（错的）
// fn main() {
//     let x = define_x()
// 这里我写的时候没想明白怎么弄，一直报错
//     println!("{}, world", define_x()); 
// }
// 
// fn define_x() {
//     let _x = "hello";
// }
// 官方答案 1
// fn main() {
//     let x = define_x();
//     println!("{}, world", x);
// }
// 差评！Rust圣经变量的解构与绑定里根本没讲到返回值怎么设定呀！！！
// 它这里做了返回值的设定，在main里面初始化了x
// fn define_x() -> String {
//     let x = "hello".to_string();
//     x
// }
// 官方答案 2
// fn main() {
//     let x = define_x();
//     println!("{:?}, world", x);
// }
// 暂时看个半懂，还没学到返回值设定
// fn define_x() -> &'static str {
//     let x = "hello";
//     x
// }

// ---变量遮蔽---

// test 5
// 题目-只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }
// 
//     assert_eq!(x, 12);
// 
//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }
// 我的答案
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }
// 
//     assert_eq!(x, 5);
// 
//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }
// 官方答案
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }
// 嘻嘻！我的答案和官方答案一样！
//     assert_eq!(x, 5);
// 
//     let x = 42;
//     println!("{}", x); // Prints "42".
// }

// test 6
// 题目-修改一行代码以通过编译
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let x = x; 
//     x += 3;
// 
//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!"; 
// }
// 我的答案
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let x = x; 
//     x += 3;
// 
//     let y = 4;
//     // 遮蔽
//     let y: &str = "I can also be bound to text!"; 
// }
// 官方答案
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and re-binding
//     let x = x;
// 对于这个答案我只能是说啊？啊？啊？这样的吗？
// 运行官方答案后可以看到warning信息
// 意思是warning不算错？？？确实也不是错
// 只是警告变量没有用到而已，但我还是不理解答案为啥是这样
//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!"; 
// 
//     println!("Success!");
// }

// ---未使用的变量---

// test 7
// 题目-使用以下方法来修复编译器输出的 warning
// 注意: 你可以使用两种方法解决，但是它们没有一种是移除 let x = 1 所在的代码行
// fn main() {
//     let x = 1; 
// }
// compiler warning: unused variable: `x`
// 我的答案 1
// fn main() {
//     let _x = 1; 
// }
// 我的答案 2
// fn main() {
//     let x = 1;
//     println!("x is {}", x)
// }
// 官方答案 1
// fn main() {
// 嘻嘻！又写的一样了！
//     let _x = 1;
// }
// 官方答案 2
// 我只能说啊？啊？啊？你来这套的吗？
// 下面这句话应该是什么写法吧，能够允许存在未使用的变量
// 真好，我都不知道还能这样写，我还寻思让这个变量使用一下就没错了
// #[allow(unused_variables)]
// fn main() {
//     let x = 1;
// }

// ---变量解构---

// test 8
// 题目-修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;
// 
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }
// 我的答案 1 - 变量可变性
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;
// 
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }
// 我的答案 2 - 变量遮蔽
// fn main() {
//     let (x, y) = (1, 2);
//     let x = 3;
// 
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// 
//     println!("x is {}", x)
// }
// 官方答案 1
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;
// 嘻嘻！又写的一样喽！
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }
// 官方答案 2
// fn main() {
//     let (x, y) = (1, 2);
//     let x = 3;
// 我只能说好吧，又是一个仅warning，这里是利用变量遮蔽
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// ---解构式赋值---

// test 9
// 题目-填空，让代码工作
// 注意：该功能于 Rust 1.59 版本引入：你可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值。
// 即解构式赋值只能在 Rust 1.59 或者更高版本中使用
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     assert_eq!([x,y], __);
// } 
// 我的答案
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     assert_eq!([x,y], [3,2]);
// } 
// 官方答案
// fn main() {
// 哦吼！又写对了！！！但对于那个..属于是一知半解吧，后续再补充一下
//     let (x, y);
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     // fill the blank to make the code work
//     assert_eq!([x, y], [3, 2]);
// }