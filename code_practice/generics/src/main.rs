// 《Rust圣经》泛型 练习实践
// 题目：https://practice-zh.course.rs/generics-traits/generics.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/generics-traits/generics.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 泛型 ---

// --- 函数 ---

// test 1
// 题目-填空
// struct A;          // 具体的类型 `A`.
// struct S(A);       // 具体的类型 `S`.
// struct SGen<T>(T); // 泛型 `SGen`.
// fn reg_fn(_s: S) {}
// fn gen_spec_t(_s: SGen<A>) {}
// fn gen_spec_i32(_s: SGen<i32>) {}
// fn generic<T>(_s: SGen<T>) {}
// fn main() {
//     // 使用非泛型函数
//     reg_fn(__);          // 具体的类型
//     gen_spec_t(__);   // 隐式地指定类型参数  `A`.
//     gen_spec_i32(__); // 隐式地指定类型参数`i32`.
//     // 显式地指定类型参数 `char`
//     generic::<char>(__);
//     // 隐式地指定类型参数 `char`.
//     generic(__);
// }
// 我的答案 不会写 搞不懂
// struct A;          // 具体的类型 `A`.
// struct S(A);       // 具体的类型 `S`.
// struct SGen<T>(T); // 泛型 `SGen`.
// fn reg_fn(_s: S) {}
// fn gen_spec_t(_s: SGen<A>) {}
// fn gen_spec_i32(_s: SGen<i32>) {}
// fn generic<T>(_s: SGen<T>) {}
// fn main() {
//     // 使用非泛型函数
//     reg_fn(S(A));          // 具体的类型
//     gen_spec_t(SGen<A>);   // 隐式地指定类型参数  `A`.
//     gen_spec_i32(SGen<i32>); // 隐式地指定类型参数`i32`.
//     // 显式地指定类型参数 `char`
//     generic::<char>(SGen<char);
//     // 隐式地指定类型参数 `char`.
//     generic(SGen<char>);
// }
// 官方答案
// 定义了一个没有字段的具体类型 A
// struct A;          // Concrete type `A`.
// 定义了一个包含单个字段 A 的具体类型 S。
// 这个字段是匿名的，因为您没有在括号外给它命名。
// 注意，这种语法是 Rust 早期版本的遗留语法，
// 现代 Rust 通常使用大括号 {} 和命名字段来定义结构体。
// 不过，对于只有一个匿名字段的元组结构体，这种语法仍然是有效的。
// struct S(A);       // Concrete type `S`.
// 定义了一个泛型类型 SGen<T>，它包含一个类型为 T 的匿名字段
// struct SGen<T>(T); // Generic type `SGen`.
// fn reg_fn(_s: S) {}
// fn gen_spec_t(_s: SGen<A>) {}
// fn gen_spec_i32(_s: SGen<i32>) {}
// fn generic<T>(_s: SGen<T>) {}
// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.
//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('a'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen('c'));
// }

// test 2
// 题目-实现下面的泛型函数 sum
// fn sum
// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }
// 我的答案
// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }
// 官方答案
// fn sum<T:std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }
// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }

// --- 结构体和impl ---

// test 3
// 题目-实现一个结构体 Point 让代码工作
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }
// 我的答案
// struct Point<T> {
//     x: T,
//     y: T,   
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }
// 官方答案
// struct Point<T> {
//     x: T,
//     y: T,
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// test 4
// 题目-修改以下结构体让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }
// fn main() {
//     // 不要修改这行代码！
//     let p = Point{x: 5, y : "hello".to_string()};
// }
// 我的答案
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// fn main() {
//     // 不要修改这行代码！
//     let p = Point{x: 5, y : "hello".to_string()};
// }
// 官方答案
// modify this struct to make the code work
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// fn main() {
//     // DON'T modify here
//     let p = Point{x: 5, y : "hello".to_string()};
// }

// test 5
// 题目-为 Val 增加泛型参数，不要修改 `main` 中的代码
// struct Val {
//     val: f64,
// }
// impl Val {
//     fn value(&self) -> &f64 {
//         &self.val
//     }
// }
// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }
// 我的答案
// struct Val<T> {
//     val: T,
// }
// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }
// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }
// 官方答案
// struct Val<T> {
//     val: T,
// }
// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }
// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

// --- 方法 ---

// test 6
// 题目-实现 mixup，不要修改其它代码！
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     // 实现 mixup，不要修改其它代码！
//     fn mixup
// }
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};
//     let p3 = p1.mixup(p2);
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }
// 我的答案
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     // 实现 mixup，不要修改其它代码！
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};
//     let p3 = p1.mixup(p2);
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }
// 官方答案
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};
//     let p3 = p1.mixup(p2);
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }

// test 7
// 题目-修复错误，让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// fn main() {
//     let p = Point{x: 5, y: 10};
//     println!("{}",p.distance_from_origin())
// }
// 我的答案
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// fn main() {
//     let p = Point{x: 5.9, y: 10.1};
//     println!("{}",p.distance_from_origin())
// }
// 官方答案
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// fn main() {
//     let p = Point{x: 5.0_f32, y: 10.0_f32};
//     println!("{}",p.distance_from_origin())
// }
