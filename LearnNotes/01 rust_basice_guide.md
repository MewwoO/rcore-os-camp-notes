# Rust 基础入门
## Rust 基本概念
```rust
// Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
fn main() {
    // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    let a = 10;
    // 主动指定b的类型为i32
    let b: i32 = 20;
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let mut c = 30i32;
    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;
    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把e的值代入进来
    println!("( a + b ) + ( c + d ) = {}", e);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回相加值，这里可以省略return
    // 不要为 i+j 添加 ;，这会改变语法导致函数返回 () 而不是 i32
    i + j
}

// 字符串使用双引号 "" 
// 单个字符类型（char）使用单引号
```

## 变量绑定与解构
(笑死，Rust圣经作者认为我这个读者是已经熟练掌握其它任意一门编程语言，所以掠过解释何为变量，只能说还好我确实已经熟练掌握其它任意一门编程语言，just like Go)

### 为什么要手动设置变量的可变性
- 既保证灵活性又保证安全性
- 提升运行性能，因为将本身无需改变的变量声明为不可变在运行期会避免一些多余的 runtime 检查
Salute to The Rust Team!

### 变量命名
[Rust 命名规范](https://course.rs/practice/naming.html)

### 变量绑定
- let a = "hello world" 这个过程叫`变量绑定`
> 不用`赋值`的概念而是采用`绑定`概念的原因:
> Rust 最核心的原则——所有权，简单来讲，任何内存对象都是有主人的，而且一般情况下完全属于它的主人，绑定就是把这个对象绑定给一个变量，让这个变量成为它的主人

### 变量可变性
- Rust 的变量在默认情况下是不可变的
- 可以通过`mut`关键字让变量变为可变的
    - 好处
        - 可以避免变量发生无法预期的错误，同时提升代码易读性
        - 使用上的灵活性
        - 性能上的提升
```shell
PS D:\rcore-os-camp-notes\CodePractice\variables> cargo run
   Compiling variables v0.1.0 (D:\rcore-os-camp-notes\CodePractice\variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 6;
  |         - first assignment to `x`
3 |     println!("The value of x is {}", x);
4 |     x = 7;
  |     ^^^^^ cannot assign twice to immutable variable    
  |
help: consider making this binding mutable
  |
2 |     let mut x = 6;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error

PS D:\rcore-os-camp-notes\CodePractice\variables> cargo run
   Compiling variables v0.1.0 (D:\rcore-os-camp-notes\CodePractice\variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target\debug\variables.exe`
The value of x is 6
The value of x is 7
```

### 使用下划线开头忽略未使用的变量
- 使用下划线作为变量名的开头可以使得 Rust 不去警告未使用的变量（Go有类似的下划线定义）
```shell
PS D:\rcore-os-camp-notes\CodePractice\variables_underline> cargo run
   Compiling variables_underline v0.1.0 (D:\rcore-os-camp-notes\CodePractice\variables_underline)
warning: unused variable: `y`
 --> src/main.rs:3:9
  |
3 |     let y = 10;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `variables_underline` (bin "variables_underline") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s       
     Running `target\debug\variables_underline.exe`
```

### 变量解构
- let 表达式用于
    - 变量的绑定
    - 复杂变量的解构
        - 解构：从一个相对复杂的变量中，匹配出该变量的一部分内容
```shell
PS D:\rcore-os-camp-notes\CodePractice\variables_deconstruction> cargo run
   Compiling variables_deconstruction v0.1.0 (D:\rcore-os-camp-notes\CodePractice\variables_deconstruction)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target\debug\variables_deconstruction.exe`
a = true, b = false
```
- 解构式赋值：在 Rust 1.59 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了
- 使用 += 的赋值语句还不支持解构式赋值

### 变量和常量之间的差异
- 常量不允许使用 `mut`
- 常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
- 常量使用 `const` 关键字声明，并且值的类型必须标注。
```rust
const MAX_POINTS: u32 = 100_000;
```
- 常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效。
    - 对于需要在多处代码共享一个不可变的值时非常有用，例如游戏中允许玩家赚取的最大点数或光速。
> 建议：在实际使用中，最好将程序中用到的硬编码值都声明为常量，对于代码后续的维护有莫大的帮助。如果将来需要更改硬编码的值，也只需要在代码中更改一处即可。

### 变量遮蔽
- Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
```shell
PS D:\rcore-os-camp-notes\CodePractice\variables_shadowing> cargo run
   Compiling variables_shadowing v0.1.0 (D:\rcore-os-camp-notes\CodePractice\variables_shadowing)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target\debug\variables_shadowing.exe`
The value of x in the inner scope is: 12
The value of x is: 6
```
- 变量遮蔽和 `mut` 声明变量的区别
    - 变量遮蔽中 `let` 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配
    - `mut` 声明变量可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好
- 变量遮蔽的用处
    - 如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字
```rust
// 变量遮蔽的写法
// 字符串类型
let spaces = "   ";
// usize数值类型
let spaces = spaces.len();

// 绝！直接理解变量遮蔽的用处！不用去写space_str和space_num

// 不用 let 的写法, 会报错
let mut spaces = "   ";
spaces = spaces.len();
```
```shell
# Rust 对类型的要求很严格，不允许将整数类型 usize 赋值给字符串类型。usize 是一种 CPU 相关的整数类型
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

error: aborting due to previous error
```

### 课后练习
[rust by practice](https://practice-zh.course.rs/variables.html)