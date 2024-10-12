// 《Rust圣经》特征 练习实践
// 题目：https://practice-zh.course.rs/generics-traits/traits.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/generics-traits/traits.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- 特征 ---
// 特征 Trait 可以告诉编译器一个特定的类型所具有的、且能跟其它类型共享的特性
// 我们可以使用特征通过抽象的方式来定义这种共享行为，
// 还可以使用特征约束来限定一个泛型类型必须要具有某个特定的行为
// 示例
// struct Sheep { // 定义 Sheep 结构体
//     // 结构体有两个字段: naked, name
//     naked: bool, // naked 字段的类型是 bool
//     name: String, // name 字段的类型是 String
// }
// // sheep 结构体的方法的实现
// impl Sheep {
//     // 判断 is_naked 方法
//     fn is_naked(&self) -> bool {
//         self.naked
//     }
//     // shear 方法
//     fn shear(&mut self) {
//         if self.is_naked() {
//             // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);

//             self.naked = true;
//         }
//     }
// }
// // Animal 特征的定义
// trait Animal {
//     // 关联函数签名；`Self` 指代实现者的类型
//     // 例如我们在为 Sheep 类型实现特征时，
//     // 那 `new` 函数就会返回一个 `Sheep` 类型的实例，
//     // 这里的 `Self` 指代的就是 `Sheep` 类型
//     fn new(name: String) -> Self;
//     // 方法签名
//     fn name(&self) -> String;
//     fn noise(&self) -> String;
//     // 方法还能提供默认的定义实现
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

// // 为 Sheep 类型实现 Animal 特征
// impl Animal for Sheep {
//     // `Self` 被替换成具体的实现者类型： `Sheep`
//     fn new(name: String) -> Sheep {
//         Sheep { name: name, naked: false }
//     }
//     // 实现方法签名
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//     fn noise(&self) -> String {
//         if self.is_naked() {
//             "baaaaah?".to_string()
//         } else {
//             "baaaaah!".to_string()
//         }
//     }
//     // 默认的特征方法可以被重写
//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }
// fn main() {
//     // 这里的类型注释时必须的
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());
//     // TODO ^ 尝试去除类型注释，看看会发生什么
//     dolly.talk();
//     dolly.shear();
//     dolly.talk();
// }

// test 1
// 题目-完成两个 `impl` 语句块,不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String;
// }
// struct Student {}
// impl Hello for Student {
// }
// struct Teacher {}
// impl Hello for Teacher {
// }
// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");
//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");
//     println!("Success!")
// }
// 我的答案
// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
// Hello特征定义
// trait Hello {
//     // 默认实现方法
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     // 方法签名
//     fn say_something(&self) -> String;
// }
// // student 结构体
// struct Student {}
// // 为 Student 类型实现 Hello 特征
// impl Hello for Student {
//     // 实现方法签名
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// // teacher 结构体
// struct Teacher {}
// // 为 Teacher 类型实现 Hello 特征
// impl Hello for Teacher {
//     // 重载实现默认方法
//     // &self表示对Teacher的不可变借用
//     // 只读取数据,并不修改,也不获取所有权
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//     // 实现方法签名
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }
// fn main() {
//     // 创建并初始化Student实例并将其绑定到变量s上
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");
//     // 创建并初始化Teacher实例并将其绑定到变量t上
//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");
//     println!("Success!")
// }
// 官方答案
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }
// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");
//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");
// }

// --- Derive 派生 ---
// 我们可以使用 #[derive] 属性来派生一些特征，
// 对于这些特征编译器会自动进行默认实现，
// 对于日常代码开发而言，这是非常方便的，
// 例如大家经常用到的 Debug 特征，
// 就是直接通过派生来获取默认实现，而无需我们手动去完成这个工作。

// test 2
// 题目-添加代码让代码工作
// `Centimeters`, 一个元组结构体，可以被比较大小
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);
// // `Inches`, 一个元组结构体可以被打印
// #[derive(Debug)]
// struct Inches(i32);
// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;
//         Centimeters(inches as f64 * 2.54)
//     }
// }
// // 添加一些属性让代码工作
// // 不要修改其它代码！
// struct Seconds(i32);
// fn main() {
//     let _one_second = Seconds(1);
//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_false = _one_second > _one_second;
//     let foot = Inches(12);
//     println!("One foot equals {:?}", foot);
//     let meter = Centimeters(100.0);
//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };
//     println!("One foot is {} than one meter.", cmp);
// }
// 我的答案
// // 派生 PartialEq, PartialOrd 特征
// #[derive(PartialEq, PartialOrd)]
// // 定义Centimeters元组结构体
// // 元组中有一个字段类型为f64
// struct Centimeters(f64);
// // `Inches`, 一个元组结构体可以被打印
// // 派生 Debug 特征
// #[derive(Debug)]
// // 定义Inches元组结构体
// // 元组中有一个字段类型为i32
// struct Inches(i32);
// // 为Inches结构体实现方法
// impl Inches {
//     // to_centimeters方法
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;
//         Centimeters(inches as f64 * 2.54)
//     }
// }
// // 添加一些属性让代码工作
// // 不要修改其它代码！
// // 定义Seconds元组结构体
// // 元组中有一个字段类型为i32
// #[derive(PartialEq, PartialOrd, Debug)]
// struct Seconds(i32);
// // 主函数
// fn main() {
//     let _one_second = Seconds(1);
//     // 打印，使用Debug特征
//     println!("One second looks like: {:?}", _one_second);
//     // 等值比较，使用 PartialEq 特征
//     let _this_is_true = _one_second == _one_second;
//     // 次序比较，使用 PartialOrd 特征
//     let _this_is_false = _one_second > _one_second;
//     let foot = Inches(12);
//     //  // 打印，使用Debug特征
//     println!("One foot equals {:?}", foot);
//     let meter = Centimeters(100.0);
//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };
//     println!("One foot is {} than one meter.", cmp);
// }
// 官方答案
// // `Centimeters`, a tuple struct that can be compared
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);
// // `Inches`, a tuple struct that can be printed
// #[derive(Debug)]
// struct Inches(i32);
// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }
// // Add some attributes to make the code work
// // DON'T modify other codes
// #[derive(Debug,PartialEq,PartialOrd)]
// struct Seconds(i32);
// fn main() {
//     let _one_second = Seconds(1);
//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = (_one_second == _one_second);
//     let _this_is_false = (_one_second > _one_second);
//     let foot = Inches(12);
//     println!("One foot equals {:?}", foot);
//     let meter = Centimeters(100.0);
//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };
//     println!("One foot is {} than one meter.", cmp);
// }

// --- 运算符 ---
// 在 Rust 中，许多运算符都可以被重载
// 事实上，运算符仅仅是特征方法调用的语法糖
// 例如 a + b 中的 + 是 std::ops::Add 特征的 add 方法调用
// 因此我们可以为自定义类型实现该特征来支持该类型的加法运算

// test 3
// 题目-// 实现 fn multiply 方法
// use std::ops;
// // 实现 fn multiply 方法
// // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
// fn multiply
// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
//     println!("Success!")
// }
// 我的答案 写错了 需要的是*
// use std::ops;
// // 实现 fn multiply 方法
// // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
// // T: std::ops::Add<Output = T> 特征约束
// fn multiply<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
//     println!("Success!")
// }
// 官方答案
// use std::ops;
// // implement fn multiply to make the code work
// // As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait
// // so, what about `*` ?  You can find the answer here: https://doc.rust-lang.org/core/ops/
// fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
//     x * y
// }
// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
// }

// test 4
// 题目-修复错误，不要修改 `main` 中的代码!
// use std::ops;
// struct Foo;
// struct Bar;
// struct FooBar;
// struct BarFoo;
// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }
// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;
//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }
// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
//     println!("Success!")
// }
// 我的答案 不会写
// use std::ops;
// struct Foo; // Foo 结构体
// struct Bar; // Bar 结构体
// struct FooBar; // FooBar 结构体
// struct BarFoo; // BarFoo 结构体
// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }
// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;
//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }
// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
//     println!("Success!")
// }
// 官方答案
// use std::ops;
// struct Foo;
// struct Bar;
// // 引入派生特征
// #[derive(PartialEq, Debug)]
// struct FooBar;
// #[derive(PartialEq, Debug)]
// struct BarFoo;
// // The `std::ops::Add` trait is used to specify the functionality of `+`.
// // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// // The following block implements the operation: Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }
// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;
//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }
// fn main() {
//     // DON'T modify the below code
//     // you need to derive some trait for FooBar to make it comparable
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
// }

// --- 使用特征作为函数参数 ---
// 除了使用具体类型来作为函数参数
// 我们还能通过 impl Trait 的方式来指定实现了该特征的参数：
// 该参数能接受的类型必须要实现指定的特征。

// test 5
// 题目-实现 `fn summary`,修复错误且不要移除任何代码行
// trait Summary {
//     fn summarize(&self) -> String;
// }
// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }
// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }
// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };
//     summary(post);
//     summary(weibo);
//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }
// // 在下面实现 `fn summary` 函数
// 我的答案
// // 实现 `fn summary` 
// // 修复错误且不要移除任何代码行
// use std::fmt::Debug;
// // Summary 特征定义
// trait Summary {
//     // 方法签名
//     fn summarize(&self) -> String;
// }
// // 派生特征
// #[derive(Debug)]
// struct Post { // Post 结构体类型定义
//     title: String,
//     author: String,
//     content: String,
// }
// // 为Post类型引入Summary特征
// impl Summary for Post {
//     // 实现特征方法
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }
// // 派生特征
// #[derive(Debug)]
// struct Weibo { // Weibo 结构体类型定义
//     username: String,
//     content: String,
// }
// // 为Weibo类型引入Summary特征
// impl Summary for Weibo {
//      // 实现特征方法
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }
// fn main() {
//     let post = Post { // 创建并初始化绑定 post
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo { // 创建并初始化绑定 weibo
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };
//     summary(&post);
//     summary(&weibo);
//     // 打印，引入 Debug 特征
//     println!("{:?}", post);
//     // 打印，引入 Debug 特征
//     println!("{:?}", weibo);
// }
// // 在下面实现 `fn summary` 函数
// // T: Summary 声明泛型并对其做特征约束
// fn summary<T: Summary + Debug>(item: &T) {
//     match item {
//         Post => println!("{:?}", item),
//         Weibo => println!("{:?}", item),
//         _ => println!("other"),
//     }
// }
// 官方答案
// // implement `fn summary` to make the code work
// // fix the errors without removing any code line
// trait Summary {
//     fn summarize(&self) -> String;
// }
// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }
// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }
// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };
//     summary(&post);
//     summary(&weibo);
//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }
// fn summary(t: &impl Summary) {
//     let _ = t.summarize(); // 可以直接调用特征里实现的方法
// }

// --- 使用特征作为函数返回值 ---
// 还可以在函数的返回值中使用 impl Trait 语法
// 然后只有在返回值是同一个类型时，才能这么使用
// 如果返回值是不同的类型，你可能更需要特征对象。

// test 6
// 题目-按要求完成代码
// struct Sheep {}
// struct Cow {}
// trait Animal {
//     fn noise(&self) -> String;
// }
// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }
// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }
// // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Cow {}
//     }
// }
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }
// 我的答案
// // Sheep 结构体
// struct Sheep {} 
// // Cow 结构体
// struct Cow {} 
// // Animal 特征定义
// trait Animal { 
//     // 方法签名
//     fn noise(&self) -> String;
// }
// // 为 Sheep 类型实现 Animal 特征
// impl Animal for Sheep {
//     // 实现 noise 方法
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }
// // 为 Cow 类型实现 Animal 特征
// impl Animal for Cow {
//     // 实现 noise 方法
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }
// // 返回一个类型，该类型实现了 Animal 特征
// // 但是我们并不能在编译期获知具体返回了哪个类型
// // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> impl Animal {
//     // 使用虚假的随机
//     // impl Animal 只能返回实现该特征的一个类型
//     Sheep {}
// }
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }
// 官方答案 1 虚假随机
// struct Sheep {}
// struct Cow {}
// trait Animal {
//     fn noise(&self) -> String;
// }
// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }
// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }
// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the errors here, you can make a fake random, or you can use trait object
// fn random_animal(random_number: f64) -> impl Animal {
//     // 虚假随机
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Sheep {}
//     }
// }
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }
// 官方答案 2 特征对象
// struct Sheep {}
// struct Cow {}
// trait Animal {
//     fn noise(&self) -> String;
// }
// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }
// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }
// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the errors here, you can make a fake random, or you can use trait object
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow{})
//     }
// }
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }

// --- 特征约束 ---
// impl Trait 语法非常直观简洁，但它实际上是特征约束的语法糖。
// 当使用泛型参数时，我们往往需要为该参数指定特定的行为，
// 这种指定方式就是通过特征约束来实现的。

// test 7
// 题目-通过两种方法使用特征约束来实现 `fn sum`
// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T>(x: T, y: T) -> T {
//     x + y
// }
// 我的答案 1
// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }
// 我的答案 2
// use std::ops::Add;
// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }
// 官方答案 1
// fn main() {
//     assert_eq!(sum(1, 2), 3);
//     assert_eq!(sum(1.0, 2.0), 3.0);
// }
// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }
// 官方答案 2
// fn main() {
//     assert_eq!(sum(1, 2), 3);
//     assert_eq!(sum(1.0, 2.0), 3.0);
// }
// fn sum<T>(x: T, y: T) -> T
// where
//     T: std::ops::Add<Output = T>,
// { // 用了 where 做特征约束
//     x + y
// }

// test 8
// 题目-修复代码中的错误
// struct Pair<T> {
//     x: T,
//     y: T,
// }
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// struct Unit(i32);
// fn main() {
//     let pair = Pair{
//         x: Unit(1),
//         y: Unit(3)
//     };
//     pair.cmp_display();
// }
// 我的答案
// struct Pair<T> { // 泛型 Pair 结构体
//     x: T,
//     y: T,
// }
// impl<T> Pair<T> { // 定义方法
//     // 关联函数
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
// // 定义方法, 对T进行约束, 打印, 次序比较
// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// #[derive(Debug, PartialOrd, PartialEq)]
// // Unit 元组结构体
// struct Unit(i32);
// fn main() {
//     let pair = Pair{
//         x: Unit(1),
//         y: Unit(3)
//     };
//     pair.cmp_display();
// }
// 官方答案
// struct Pair<T> {
//     x: T,
//     y: T,
// }
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Unit(i32);
// fn main() {
//     let pair = Pair{
//         x: Unit(1),
//         y: Unit(3)
//     };
//     pair.cmp_display();
// }

// test 9
// 题目-填空
// fn example1() {
//     // `T: Trait` 是最常使用的方式
//     // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
//     struct Cacher<T: Fn(u32) -> u32> {
//         calculation: T,
//         value: Option<u32>,
//     }
//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(10), __);
//     assert_eq!(cacher.value(15), __);
// }
// fn example2() {
//     // 还可以使用 `where` 来约束 T
//     struct Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }
//     impl<T> Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(20), __);
//     assert_eq!(cacher.value(25), __);
// }
// fn main() {
//     example1();
//     example2();
//     println!("Success!")
// }
// 我的答案
// fn example1() { // example1 函数
//     // `T: Trait` 是最常使用的方式
//     // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
//     struct Cacher<T: Fn(u32) -> u32> { 
//         calculation: T,
//         value: Option<u32>,
//     }
//     // 方法定义
//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         // 关联函数
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg); // 通过 (self.calculation)(arg) 语法调用这个闭包
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//     let mut cacher = Cacher::new(|x| x+1); // |x| x+1 闭包
//     assert_eq!(cacher.value(10), 11);
//     assert_eq!(cacher.value(15), 11);
// }
// fn example2() {
//     // 还可以使用 `where` 来约束 T
//     struct Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }
//     impl<T> Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(20), 21);
//     assert_eq!(cacher.value(25), 21);
// }
// fn main() {
//     example1();
//     example2();
//     println!("Success!")
// }
// 官方答案
// fn example1() {
//     // `T: Trait` is the commonly used way
//     // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`
//     struct Cacher<T: Fn(u32) -> u32> {
//         calculation: T,
//         value: Option<u32>,
//     }
//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(10), 11);
//     assert_eq!(cacher.value(15), 11);
// }
// fn example2() {
//     // We can also use `where` to constrain `T`
//     struct Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T> Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(20), 21);
//     assert_eq!(cacher.value(25), 21);
// }
// fn main() {
//     example1();
//     example2();
// }