# 结构体(struct)
- 结构体由不同数据类型组合形成，因此是复合类型
- 结构体可以为内部的每个字段起一个富有含义的名称，无需依赖字段的顺序去访问和解析字段

## 结构体语法
### 定义结构体
- 结构体由以下几部分组成：
    - 使用关键字 `struct` 定义结构体
    - 结构体 `名称`
        - 见名知意的那种
    - 结构体字段
        - 结构体字段由 `字段名` 和 `字段类型` 组成，多个字段之间用 `,` 分隔 
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### 创建结构体实例
- 初始化实例时，每个字段都需要进行初始化
- 初始化时的字段顺序不需要和结构体定义时的顺序一致
```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### 访问结构体字段
- 使用 `.` 操作符访问结构体实例内部的字段值，或者修改字段值
    - 修改字段值的前提是必须将结构体实例声明为可变的，Rust 不支持将某个结构体某个字段标记为可变
```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

### 简化结构体创建
```rust
// 类似构建函数，返回 User 结构体的实例
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 再简化一点
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 结构体更新语法
