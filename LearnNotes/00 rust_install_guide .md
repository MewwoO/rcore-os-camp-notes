# Rust 导学  
```shell
PS D:\rcore-os-camp-notes\CodePractice> cargo new world_hello
    Creating binary (application) `world_hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
PS D:\rcore-os-camp-notes\CodePractice> cargo run
error: could not find `Cargo.toml` in `D:\rcore-os-camp-notes\CodePractice` or any parent directory
PS D:\rcore-os-camp-notes\CodePractice> cd .\world_hello\
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo run
   Compiling world_hello v0.1.0 (D:\rcore-os-camp-notes\CodePractice\world_hello)
error: linker `link.exe` not found
  |
  = note: program not found

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.

note: VS Code is a different product, and is not sufficient.

error: could not compile `world_hello` (bin "world_hello") due to 1 previous error
```
解决办法：
visual studio install 安装 Windows SDK, 安装 Desktop development with C++

## 运行项目
```shell
PS PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo run
   Compiling world_hello v0.1.0 (D:\rcore-os-camp-notes\CodePractice\world_hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target\debug\world_hello.exe`
Hello, world!
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
PS D:\rcore-os-camp-notes\CodePractice\world_hello> .\target\debug\world_hello  
Hello, world!
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo run --release
   Compiling world_hello v0.1.0 (D:\rcore-os-camp-notes\CodePractice\world_hello)
    Finished `release` profile [optimized] target(s) in 0.28s
     Running `target\release\world_hello.exe`
Hello, world!
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo build --release
    Finished `release` profile [optimized] target(s) in 0.00s
PS D:\rcore-os-camp-notes\CodePractice\world_hello> .\target\release\world_hello  
Hello, world!
```

## cargo check
```shell
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo check 
    Checking world_hello v0.1.0 (D:\rcore-os-camp-notes\CodePractice\world_hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

## 多国语言的"世界，你好"
```shell 
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo run
   Compiling world_hello v0.1.0 (D:\rcore-os-camp-notes\CodePractice\world_hello)
error[E0423]: expected function, found macro `println`                                                             
 --> src/main.rs:7:9
  |
7 |         println("{}", &region);
  |         ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
7 |         println!("{}", &region);
  |                +

For more information about this error, try `rustc --explain E0423`.
error: could not compile `world_hello` (bin "world_hello") due to 1 previous error
PS D:\rcore-os-camp-notes\CodePractice\world_hello> cargo run
   Compiling world_hello v0.1.0 (D:\rcore-os-camp-notes\CodePractice\world_hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target\debug\world_hello.exe`
Grüß Gott!
世界，你好
World, hello
```

我的初印象（仅运行代码，未看作者解释前）：
fn 类比 go 中的 func, 整体结构风格也很像
let 类比 go 中的 var, 应该是用于定义、声明变量的
每行代码以 ; 结尾, 有点像 java
字符串定义也是用双引号
for 遍历的 iter() 也有点像 go 的 for k, v := range s, iter() 字面上感觉像 item 的含义，每一项
println 也像 go 的 fmt.Println()，但是后面要加 !, ! 是啥意思，此外，输出时用的 {} 像是占位符，但为什么是 {}，有什么含义

- rust 原生支持 UTF-8 编码的字符串，
- `println` 后面的 `!` 是 `宏` 操作符，暂且认为其是一种特殊类型函数
- Rust 能够自动识别输出数据的类型，因此占位符直接使用 `{}` (好nb 不管啥类型 直接{})
- Rust 的集合类型不能直接进行循环，需要变成迭代器（这里是通过 .iter() 方法），才能用于迭代循环
- 现在也可简写为 `for region in regions`，因为 for 隐式地将 regions 转换成迭代器

## 初印象
在终端中运行代码时，会看到很多 debug: ... 的输出，上面有讲，这些都是 条件编译 的输出，那么该怎么消除掉这些输出呢？
在 认识 Cargo中，曾经介绍过 --release 参数，因为 cargo run 默认是运行 debug 模式。因此想要消灭那些 debug: 输出，需要更改为其它模式，其中最常用的模式就是 --release 也就是生产发布的模式。

```shell
PS D:\rcore-os-camp-notes\CodePractice\first_hard_code> cargo run                                                  al C++ option.
   Compiling first_hard_code v0.1.0 (D:\rcore-os-camp-notes\CodePractice\first_hard_code)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target\debug\first_hard_code.exe`
debug: "    Little penguin,33" -> ["Little penguin", "33"]
Little penguin, 33cm
debug: "    Yellow-eyed penguin,65" -> ["Yellow-eyed penguin", "65"]   
Yellow-eyed penguin, 65cm
debug: "    Fiordland penguin,60" -> ["Fiordland penguin", "60"]       
Fiordland penguin, 60cm
debug: "    Invalid,data" -> ["Invalid", "data"]
PS D:\rcore-os-camp-notes\CodePractice\first_hard_code> cargo run --release
   Compiling first_hard_code v0.1.0 (D:\rcore-os-camp-notes\CodePractice\first_hard_code)
    Finished `release` profile [optimized] target(s) in 0.32s
     Running `target\release\first_hard_code.exe`
Little penguin, 33cm
Yellow-eyed penguin, 65cm
Fiordland penguin, 60cm
```

## 下载依赖
- 下载依赖库的地址是 [crates.io](https://crates.io/)