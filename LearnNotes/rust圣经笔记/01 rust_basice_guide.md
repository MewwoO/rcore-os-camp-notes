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
// 对数字字面量可插入下划线以提高可读性
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

## 基本类型
- 基本类型（最小化原子类型，无法解构为其它类型）
  - 数值类型
    - 有符号整数：`i8`, `i16`, `i32`, `i64`, `isize`
    - 无符号整数：`u8`, `u16`, `u32`, `u64`, `usize`
    - 浮点数：`f32`, `f64`
    - 有理数
    - 复数
  - 字符串
    - 字符串字面量
    - 字符串切片 `&str`
  - 布尔类型
    - true
    - false
  - 字符类型
    - 单个 Unicode 字符，存储为 4 个字节
  - 单元类型
    - 唯一可能的值是 `()`
- 复合类型
  - 数组, eg. [1,2,3]
  - 元组, eg. (1, '2')

## 类型推导与标注
- Rust 是一门静态类型语言，也就是编译器必须在编译期知道我们所有变量的类型
- 类型推导：Rust 编译器很聪明，它可以根据变量的值和上下文中的使用方式来自动推导出变量的类型
- 类型标注：Rust 编译器在某些情况下，它无法推导出变量类型，需要手动去给予一个类型标注
  - 标注时用 `:` `let x: i32 = 7; `

## 数值类型
#### 整数类型
- 整数是没有小数部分的数字

|长度|有符号类型|无符号类型|
|---|---|---|
|8位|i8|u8|
|16位|i16|u16|
|32位|i32|u32|
|64位|i64|u64|
|128位|i28|u128|
|视架构而定|isize|usize|
- Rust 整型默认使用`i32`
- isize 和 usize 的主要应用场景是用作集合的索引
#### 整型溢出
- 在 debug 模式编译时，rust 会检查整型溢出，如果存在溢出问题，则使程序在编译时panic
- 在使用 `--release` 参数进行 release 模式构建时，rust 不检测溢出问题，如果存在整型溢出问题，rust 会按照补码循环溢出的规则处理，即大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值，此时程序不会panic，但是会导致变量的值不是预期的值
- 显式处理溢出，可以使用标准库针对原始数字类型提供的方法：
  - 使用 `wrapping_*` 方法在所有模式下都按照补码循环溢出规则处理
  - 使用 `checked_*` 方法时发生溢出，则返回 `None` 值
  - 使用 `overflowing_*` 方法返回该值和一个指示是否存在溢出的布尔值
  - 使用 `staturating_*` 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
#### 浮点类型
- rust 默认浮点类型是 `f64`
- 浮点数根据 `IEEE-754` 标准实现
  - `f32` 是单精度
  - `f64` 是双精度
- 浮点数陷阱
  - 避免在浮点数上测试相等性
  - 当结果在数学上可能存在未定义时，需要格外的小心
- NaN
  - 对于数学上未定义的结果，eg.`-42.1.sqrt()`, rust的浮点数类型使用 `NaN` 来处理
  - 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较
  - 可以使用 is_nan() 等方法，可以用来判断一个数值是否是 NaN

#### 数字运算
- `+`, `-`, `*`, `/`, `%`
- [Rust 支持的所有运算符](https://course.rs/appendix/operators.html#%E8%BF%90%E7%AE%97%E7%AC%A6)
```rust
fn main() {
  // 编译器会进行自动推导，给予twenty i32的类型
  let twenty = 20;
  // 类型标注
  let twenty_one: i32 = 21;
  // 通过类型后缀的方式进行类型标注：22是i32类型
  let twenty_two = 22i32;

  // 只有同样类型，才能运算
  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  // 对于较长的数字，可以用_进行分割，提升可读性
  let one_million: i64 = 1_000_000;
  println!("{}", one_million.pow(2));

  // 定义一个f32数组，其中42.0会自动被推导为f32类型
  let forty_twos = [
    42.0,
    42f32,
    42.0_f32,
  ];

  // 打印数组中第一个值，并控制小数位为2位
  println!("{:.2}", forty_twos[0]);
}
```
#### 位运算
|运算符|说明|
|---|---|
|`&` 位与|相同位置均为1时则为1，否则为0|
|`｜` 位或|相同位置只要有1时则为1，否则为0|
|`^` 异或|相同位置不相同则为1，相同则为0|
|`!` 位非|把位中的0和1相互取反，即0置为1，1置为0|
|`<<` 左移|所有位向左移动指定位数，右位补0|
|`>>` 右移|所有位向右移动指定位数，带符号移动（正数补0，负数补1）|
```rust
fn main() {
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}
```
#### 序列（Range）
- 序列只允许用于数字或字符类型
  - 原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型
- 生成连续的数值，常用于循环中
  - `1..5`, 生成1到4的连续数字, 不包含5
  - `1..=5`, 生成1到5的连续数字，包含5
```rust
for i in 1..=5 {
    println!("{}",i);
}
```
#### 使用 As 完成类型转换
- 类型转换必须是显式的
- 类型转换多是将范围较小的类型转换成较大的类型
```rust
fn main() {
   let a = 3.1 as i8;
   let b = 100_i8 as i32;
   let c = 'a' as u8; // 将字符'a'转换为整数，97

   println!("{},{},{}",a,b,c)
}
```
- 内存地址转换为指针
```rust
let mut values: [i32; 2] = [1, 2];
let p1: *mut i32 = values.as_mut_ptr();
let first_address = p1 as usize; // 将p1内存地址转换为一个整数
let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
unsafe {
    *p2 += 1;
}
assert_eq!(values[1], 3);
```
- 转换不具有传递性，e as u1 as u2 是合法的，但不能代表 e as u2 是合法的
#### 有理数和复数
- 需要引入 `num` 库，参考以下步骤
  - 创建新工程 `cargo new complex-num && cd complex-num`
  - 在 `Cargo.toml` 中的 `[dependencies]` 下添加一行 `num = "0.4.0"`
  - 将 `src/main.rs` 文件中的 `main` 函数替换为下面的代码
  - 运行 `cargo run`
```rust
use num::complex::Complex;

 fn main() {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   println!("{} + {}i", result.re, result.im)
 }
```

## 字符类型(char)
- 字符类型占用4个字节，只能用 `''` 表示
- Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型

```rust
fn main() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}

// 输出
// 字符'中'占用了4字节的内存大小
```

## 布尔类型(bool)
- 布尔值分为：`true`, `false `, 占用内存的大小为1个字节
- 布尔类型主要用于流程控制的场景
```rust
fn main() {
    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }
}
```

## 单元类型
- 单元类型就是 `()`, 唯一的值也是 `()`
- rust 的 main() 函数的返回值是单元类型 `()`
- println!() 的返回值是单元类型
- 用单元类型 `()` 作为 map 的值，表示不关注具体的值，只关注 key，类似 Go 的 struct{}，可以作为一个值用于占位，但是不占用任何内存
- Rust 中没有返回值的函数叫作 `发散函数`

## 字符、布尔、单元类型练习
[练习题](https://practice-zh.course.rs/basic-types/char-bool-unit.html)

## 语句与表达式
- Rust 函数体由一系列语句组成，最后由一个表达式来返回值
```rust
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}
```
- 语句会执行一些操作, 但是不会返回一个值
- 表达式会在求值后返回一个值，表达式后没有分号
  - 表达式如果不返回任何值，会隐式地返回一个`()`
- 在 Rust 中需要明确区分语句和表达式的概念
- 基于表达式是函数式语言的重要特征，表达式总要返回值

## 语句与表达式练习
[练习题](https://practice-zh.course.rs/basic-types/statements-expressions.html)

## 函数
- add 函数举例
  - `fn` 是声明函数的关键字
  - `add()` 是函数名
  - `i` 和 `j` 是参数
    - 参数类型和返回值类型都是 `i32`
```rust
fn add(i: i32, j: i32) -> i32 {
   i + j
 }
```
![alt text](image.png)

- 函数要点
  - 函数名和变量名使用[蛇形命名法(snake case)](https://course.rs/practice/naming.html)，eg. `fn add_two() -> {}`
  - 函数位置随便放，rust不作要求，只要有定义即可
  - Rust 是强类型语言，每个函数参数都需要标注类型
```rust
fn main() {
    another_function(5, 6.1);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```
- 在 Rust 中函数就是表达式，可以把函数的返回值直接赋给调用者
  - 函数的返回值就是函数体最后一条表达式的返回值
  - 也可以使用 return 提前返回
```rust
fn plus_five(x:i32) -> i32 {
    x + 5
}

fn main() {
    let x = plus_five(5);

    println!("The value of x is: {}", x);
}
```
```rust
fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}

fn main() {
    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);
}
```
### Rust 中的特殊返回类型
- 无返回值()
  - 单元类型 `()`，是一个零长度的元组，没什么用，但是可以用来表达一个函数没有返回值
  - 函数没有返回值，那么返回一个 `()`
  - 通过 `;` 结尾的语句返回一个 `()`
```rust
use std::fmt::Debug;
// report 函数会隐式返回一个 ()
fn report<T: Debug>(item: T) {
  println!("{:?}", item);

}
```
```rust
// clear 函数显式返回一个 ()
fn clear(text: &mut String) -> () {
  *text = String::from("");
}
```
### 永不返回的发散函数 ！
- 当用 `!` 作函数返回类型的时候，表示该函数永不返回( diverge function )
  - 特别的，这种语法往往用做会导致程序崩溃的函数
```rust
fn dead_end() -> ! {
  panic!("你已经到了穷途末路，崩溃吧！");
}

// 无限循环，该循环永不跳出，因此函数也永不返回
fn forever() -> ! {
  loop {
    //...
  };
}
```

## 函数练习题
[练习题](https://practice-zh.course.rs/basic-types/functions.html)