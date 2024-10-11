# 解构Option
## 匹配 Option<T>
- 使用 Option<T>，是为了从 Some 中取出其内部的 T 值以及处理没有值的情况
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