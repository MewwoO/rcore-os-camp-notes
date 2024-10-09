# 元组（tuple）
- 元组有多种类型组合形成，因此是复合类型
- 元组的长度固定，元组中元素的顺序固定
- 元组的索引从0开始
- 创建元组:
```rust
fn main() {
    // 变量 tup 被绑定了一个元组值 (500, 6.4, 1)
    // 该元组的类型是 (i32, f64, u8)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
- 使用模式匹配或者 . 操作符可以获取元组中的值

## 用模式匹配解构元组
- 解构：用同样的形式把一个复杂对象中的值匹配出来
```rust
fn main() {
    // 变量 tup 被绑定了一个元组值 (500, 6.4, 1)
    // 该元组的类型是 (i32, f64, u8)
    let tup = (500, 6.4, 1);

    // 因为元组是 (n1, n2, n3) 形式的，
    // 因此我们用一模一样的 (x, y, z) 形式来进行匹配
    // 元组中对应的值会绑定到变量 x， y， z上
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

## 用 `.` 操作符访问元组
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

## 元组的使用示例
- 常用于函数返回值，使用元组可以返回多个值
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
```
> 注意：
> 在其他语言中，可以用结构体来声明一个三维空间中的点，例如 Point(10, 20, 30)
> 虽然使用 Rust 元组也可以做到：(10, 20, 30)，但是这样写有个非常重大的缺陷：不具备任何清晰的含义