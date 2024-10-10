# 枚举(Enum)
- 枚举(enum 或 enumeration)允许通过列举可能的成员来定义一个枚举类型, eg.
```rust
// 扑克牌花色
enum PokerSuit {
  Clubs, // 梅花
  Spades, // 黑桃
  Diamonds, // 方块
  Hearts, // 红心
}
```
- 枚举类型是一个类型，它会包含所有可能的枚举成员, eg. PokerSuit 就是一个枚举类型
    - 枚举值是该类型中的具体某个成员的实例, eg. Clubs 就是 PokerSuit 类型的一个枚举值

## 枚举值
```rust
// 扑克牌花色

#[derive(Debug)]

enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}

fn main() {
    // 使用 :: 操作符来访问 PokerSuit 下的具体成员
    // heart 和 diamond 都是 PokerSuit 枚举类型
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);
}

fn print_suit(card: PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}",card);
}
```
```rust
// 扑克牌花色带值

#[derive(Debug)]

enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

// 使用结构体实现
struct PokerCard {
    suit: PokerSuit, // 牌的花色
    value: u8 // 牌的数值
}

fn main() {
   let c1 = PokerCard {
       suit: PokerSuit::Clubs,
       value: 1,
   };
   let c2 = PokerCard {
       suit: PokerSuit::Diamonds,
       value: 12,
   };
}

// 简化
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds(13);
}

// 同一个枚举类型下的不同成员还能持有不同的数据类型
// 任何类型的数据都可以放入枚举成员
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds('A');
}
```

## 同一化类型
- 使用枚举来将多种不同类型或形式的数据统一到一个共同的枚举类型下
- 好处: 提高代码的可读性、可维护性和类型安全性
```rust
fn new (stream: TcpStream) {
  let mut s = stream;
  if tls {
    s = negotiate_tls(stream)
  }

  // websocket是一个WebSocket<TcpStream>或者
  //   WebSocket<native_tls::TlsStream<TcpStream>>类型
  websocket = WebSocket::from_raw_socket(
    s, ......)
}

enum Websocket {
  Tcp(Websocket<TcpStream>),
  Tls(Websocket<native_tls::TlsStream<TcpStream>>),
}
```

## Option 枚举用于处理空值
- Option 枚举包含两个成员:
    - Some(T) 表示 Option 中存在一个值 T, T 是泛型参数
    - None 表示 Option 中不存在值
```rust
// Option枚举定义
enum Option<T> {
    Some(T),
    None,
}
```

- Option 主要用于处理可能为空的值，从而避免空指针异常的问题
- Option 常见使用场景如下:
    - 函数返回值：当函数可能返回空值时，可以使用Option作为返回值类型。这样，调用者就可以根据Option的类型来判断函数是否成功返回了值。
    - 结构体字段：结构体的字段也可以是Option类型，以表示该字段可能为空。这有助于避免在访问空字段时发生空指针异常。
    - 空值处理：在处理可能为空的值时，Option提供了一种安全且优雅的方式。例如，可以使用match表达式、if let语句或Option提供的方法（如map、and_then、or等）来处理Option中的值。

```rust
// 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中
// 当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值
let some_number = Some(5);
let some_string = Some("a string");

// 使用 None 时需要告诉 Rust Option<T> 是什么类型
// 因为编译器只通过 None 值无法得知 Some 成员保存的值的类型
let absent_number: Option<i32> = None;
```

- option 常用方法如下:
    - map：对Option中的值执行某个操作，并返回一个新的Option。如果Option为None，则不执行任何操作。
    - and_then：对Option中的值执行某个操作，该操作返回一个Option。然后，将返回的这个Option作为最终结果。如果原始Option为None，则不执行任何操作。
    - or：如果Option为None，则返回一个指定的默认值。如果Option为Some，则返回原始值。
    - or_else：与or类似，但允许指定一个返回Option的闭包作为默认值。
    - unwrap：如果Option为Some，则返回其中的值。如果为None，则触发panic。这种方法在需要确保Option一定有值时使用，但存在风险。
    - unwrap_or：如果Option为Some，则返回其中的值。如果为None，则返回一个指定的默认值。
    - unwrap_or_else：与unwrap_or类似，但允许指定一个返回默认值的闭包。
    - take：获取Option中的值，并将Option设置为None。这在需要获取值并清空Option时非常有用。
    - filter：对Option中的值进行过滤，如果值符合条件，则返回Some(value)，否则返回None。
```rust
fn main() {  
    let some_value: Option<i32> = Some(5);  
      
    // 使用map方法  
    let new_value = some_value.map(|value| value * 2);  
    println!("The new value is {:?}", new_value);  
      
    // 使用and_then方法  
    let new_value_and_then = some_value.and_then(|value| Some(value * 2));  
    println!("The new value and then is {:?}", new_value_and_then);  
      
    // 使用if let语句判断Option中的值  
    if let Some(value) = some_value {  
        println!("The value is {}", value);  
    } else {  
        println!("There is no value");  
    }  
      
    // 使用or方法获取默认值  
    let none_value: Option<i32> = None;  
    let new_value_or = none_value.or(Some(10));  
    println!("The new value or is {:?}", new_value_or);  
      
    // 使用take方法获取值并清空Option  
    let mut some_value_mut: Option<i32> = Some(5);  
    let value = some_value_mut.take();  
    println!("The value is {:?}", value);  
    println!("The new Option is {:?}", some_value_mut);  
}
```

- 使用 Option<T> 值，需要编写处理每个成员的代码, 可以考虑使用 match 表达式处理枚举的控制流, 类比 switch
    - 你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T
    - 也希望一些代码在值为 None 时运行
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```