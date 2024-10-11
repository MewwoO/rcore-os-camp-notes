# match 和 if let

## match 匹配
- 模式匹配：将模式与 target 进行匹配, 模式匹配不止有 match
- match 将一个值与一系列的模式相比较，并根据相匹配的模式执行相应的代码
    - 不同分支之间使用逗号分隔
    - 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值
    - 如果分支有多行代码，那么需要用 {} 包裹，同时最后一行代码需要是一个表达式
```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
- match 的匹配必须要穷举出所有可能，如果未完全列举，可以用 _ 来代表未列出的所有可能性
- match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
- X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { // 满足一个即可
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

### 使用 match 表达式赋值
```rust
enum IpAddr {
   Ipv4,
   Ipv6
}

fn main() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}
```

### 模式绑定
- 模式匹配的另外一个重要功能是从模式中取出绑定的值
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Coin::Quarter 成员还存放了一个值: 美国的某个州
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 在匹配 Coin::Quarter(state) 模式时，
        // 我们把它内部存储的值绑定到了 state 变量上，
        // 因此 state 变量就是对应的 UsState 枚举类型
        // eg. Coin::Quarter(UsState::Alaska), 
        // 它在匹配时，state 变量将被绑定 UsState::Alaska 的枚举值
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

// 运行结果
// Hello Rust
// point from (0, 0) move to (1, 2)
// change color into '(r:255, g:255, b:0)', 'b' has been ignored
```

### 穷尽匹配
- match 的匹配必须穷尽所有情况, 如果不想在匹配时列出所有值，可以使用以下两种方式：
    - `_` 通配符
        - 通过将 `_` 其放置于其他分支后，`_` 将会匹配所有遗漏的值。`()` 表示返回单元类型与所有分支返回值的类型相同，所以当匹配到 `_` 后，什么也不会发生
    - 用一个变量来承载其他情况
```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    // _ 通配符
    _ => (),
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        // 用一个变量承载
        other => println!("other direction: {:?}", other),
    };
}
```

## if let 匹配
- 只要匹配一个条件且忽略其他条件时就用 if let, 否则都用 match
```rust
let v = Some(3u8);
match v {
    Some(3) => println!("three"),
    _ => (),
}

// 简化
if let Some(3) = v {
    println!("three");
}
```

## matches!宏
- matches!宏由Rust 标准库提供，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
```rust
let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));
```

## 变量遮蔽
- 无论是 match 还是 if let，这里都是一个新的代码块，而且这里的绑定相当于新变量
    - 如果你使用同名变量，会发生变量遮蔽
- match 中的变量遮蔽并不容易看出，因此要小心！建议最好不要使用同名，避免难以理解
```rust
// if let
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   // 尝试从age中提取值。如果age是Some，
   // 则将其内部的值（这里是30）赋给新的变量age_value
   // if let Some(age_value) = age
   if let Some(age) = age {
       // 变量名age在if let块内被age_value（在修正代码中）所遮蔽
       // 但这仅影响if let块内部的作用域。外部的age变量保持不变
       println!("匹配出来的age是{}",age);
   }

   // age变量仍然是Some(30)，因为它没有被修改 
   println!("在匹配后，age是{:?}",age);
}

// 运行结果
// 在匹配前，age是Some(30)
// 匹配出来的age是30
// 在匹配后，age是Some(30)

// match
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   match age {
       Some(age) =>  println!("匹配出来的age是{}",age),
       _ => ()
   }
   println!("在匹配后，age是{:?}",age);
}

// 不使用同名
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}", age);
   match age {
       Some(x) =>  println!("匹配出来的age是{}", x),
       _ => ()
   }
   println!("在匹配后，age是{:?}", age);
}
```