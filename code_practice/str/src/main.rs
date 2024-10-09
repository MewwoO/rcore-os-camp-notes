// 《Rust圣经》变量绑定与结构 练习实践
// 题目：https://practice-zh.course.rs/compound-types/string.html
// 答案：https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/string.md
// 以下为我练习时的代码，题目及答案均有，根据注释分割
// 我的答案是我写的，官方答案是官方写的
// 注释里会补充知识点，也会吐槽，仅作自娱
// 需要运行哪个题的代码就取消哪个题的代码注释就好了（也不一定有人看，应该都是我自己看）

// --- str 和 &str ---

// test 1
// 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
// 题目-修复错误，不要新增代码行
// fn main() {
//     let s: str = "hello, world";
// }
// 我的答案
// fn main() {
//     let _s: &str = "hello, world";
// }
// 官方答案
// fn main() {
//     let s: &str = "hello, world";
// }

// test 2
// 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
// 题目-使用至少两种方法来修复错误
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }
// fn greetings(s: &str) {
//     println!("{}",s)
// }
// 我的答案 1
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }
// fn greetings(s: &str) {
//     println!("{}",s)
// }
// 我的答案 2
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }
// fn greetings(s: Box<str>) {
//     println!("{}",s)
// }
// 官方答案 1
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
//  }
//  fn greetings(s: &str) {
//      println!("{}",s)
//  }
// 官方答案 2
// fn main() {
//     let s: Box<&str> = "hello, world".into();
//     greetings(*s)
// }
// 这个我写的不一样
// fn greetings(s: &str) {
//     println!("{}", s);
// }

// --- String ---
// String 是定义在标准库中的类型，分配在堆上，可以动态的增长。
// 它的底层存储是动态字节数组的方式( Vec<u8> )，但是与字节数组不同，String 是 UTF-8 编码。

// test 3
// 题目-填空
// fn main() {
//     let mut s = __;
//     s.push_str("hello, world");
//     s.push('!');
//     assert_eq!(s, "hello, world!");
// }
// 我的答案
// fn main() {
//     let mut s = String::from("");
//     s.push_str("hello, world");
//     s.push('!');
//     assert_eq!(s, "hello, world!");
// }
// 官方答案
// fn main() {
// 这里用的是new
//     let mut s = String::new();
//     s.push_str("hello, world");
//     s.push('!');
//     assert_eq!(s, "hello, world!");
//  }

// test 4
// 题目-修复所有错误，并且不要新增代码行
// fn main() {
//     let  s = String::from("hello");
//     s.push(',');
//     s.push(" world");
//     s += "!".to_string();
//     println!("{}", s)
// }
// 我的答案
// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";
//     println!("{}", s)
// }
// 官方答案
// fn main() {
//     let mut s = String::from("hello");
//      s.push(',');
//      s.push_str(" world");
//      s += "!";
//      println!("{}", s)
//  }

// test 5
// 可以用 replace 方法来替换指定的子字符串，返回一个新的 String
// 题目-填空
// fn main() {
//     let s = String::from("I like dogs");
//     // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//     let s1 = s.__("dogs", "cats");
//     assert_eq!(s1, "I like cats")
// }
// 我的答案
// fn main() {
//     let s = String::from("I like dogs");
//     // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//     let s1 = s.replace("dogs", "cats");
//     assert_eq!(s1, "I like cats")
// }
// 官方答案
// fn main() {
//     let s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.replace("dogs", "cats");
//     assert_eq!(s1, "I like cats")
// }

// test 6
// + 只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
// 题目-修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + s2; 
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1);
// }
// 我的答案
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; 
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s2);
// }
// 官方答案
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2; // 它用了clone
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1);
// }


// --- &str 和 String ---
// 与 str 的很少使用相比，&str 和 String 类型却非常常用，因此也非常重要。

// test 7
// 题目-使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }
// fn greetings(s: String) {
//     println!("{}",s)
// }
// 我的答案 1
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }
// fn greetings(s: &str) {
//     println!("{}",s)
// }
// 我的答案 2
// fn main() {
//     let s = String::from("hello, world");
//     greetings(s)
// }
// fn greetings(s: String) {
//     println!("{}",s)
// }
// 官方答案 1
// fn main() {
//     let s = "hello, world".to_string();
//     greetings(s)
// }
// fn greetings(s: String) {
//     println!("{}",s)
// }
// 官方答案 2
// fn main() {
//     let s = String::from("hello, world");
//     greetings(s)
// }
// fn greetings(s: String) {
//     println!("{}",s)
// }

// test 8
// 题目-使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = s;
// }
// 我的答案 1
// fn main() {
//     let s = "hello, world".to_string();
//     let _s1: String = s;
// }
// 我的答案 2
// fn main() {
//     let s = "hello, world";
//     let _s1: &str = s;
// }
// 官方答案 1 这个我没想到
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = &s;
// }
// 官方答案 2
// fn main() {
//     let s = "hello, world";
//     let s1: &str = s;
// }
// 官方答案 3
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: String = s;
// }

// --- 字符串转义 ---

// test 9
// 题目-转义
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73__!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );
//     // 还能使用 \ 来连接多行字符串
//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }
// 我的答案
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73t!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );
//     // 还能使用 \ 来连接多行字符串
//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }
// 官方答案
// fn main() {
//     // You can use escapes to write bytes by their hexadecimal values
//     // fill the blank below to show "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
//     // ...or Unicode code points.
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );
//    let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }

// test 10
// 有时候需要转义的字符很多，我们会希望使用更方便的方式来书写字符串: raw string.
// 题目-填空并修复所有错误
// fn main() {
//     let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//     // 修改上面的行让代码工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes)
//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);
//     // 填空
//     let long_delimiter = __;
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }
// 我的答案
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // 修改上面的行让代码工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);
//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);
//     // 填空
//     let long_delimiter = "Hello, \"##\"";
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }
// 官方答案
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // modify above line to make it work
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
//     // If you need quotes in a raw string, add a pair of #s
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);
//     // If you need "# in your string, just use more #s in the delimiter.
//     // You can use up to 65535 #s.
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);
//     // Fill the blank
//     let long_delimiter = r###"Hello, "##""###; // 写的不一样
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }

// --- 字节字符串 ---
// 想要一个非 UTF-8 形式的字符串吗
// (我们之前的 str, &str, String 都是 UTF-8 字符串) ? 
// 可以试试字节字符串或者说字节数组:
// 示例
// use std::str;
// fn main() {
//     // 注意，这并不是 `&str` 类型了！
//     let bytestring: &[u8; 21] = b"this is a byte string";
//     // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
//     println!("A byte string: {:?}", bytestring);
//     // 字节数组也可以使用转义
//     let escaped = b"\x52\x75\x73\x74 as bytes";
//     // ...但是不支持 unicode 转义
//     // let escaped = b"\u{211D} is not allowed";
//     println!("Some escaped bytes: {:?}", escaped);
//     // raw string
//     let raw_bytestring = br"\u{211D} is not escaped here";
//     println!("{:?}", raw_bytestring);
//     // 将字节数组转成 `str` 类型可能会失败
//     if let Ok(my_str) = str::from_utf8(raw_bytestring) {
//         println!("And the same as text: '{}'", my_str);
//     }
//     let _quotes = br#"You can also use "fancier" formatting, \
//                     like with normal raw strings"#;
//     // 字节数组可以不是 UTF-8 格式
//     let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS
//     // 但是它们未必能转换成 `str` 类型
//     match str::from_utf8(shift_jis) {
//         Ok(my_str) => println!("Conversion successful: '{}'", my_str),
//         Err(e) => println!("Conversion failed: {:?}", e),
//     };
// }

// --- 字符串索引 string index ---
// 无法通过索引的方式去访问字符串中的某个字符，
// 但是可以使用切片的方式 &s1[start..end] ，
// 但是start 和 end 必须准确落在字符的边界处

// test 11
// 题目- 修复错误
// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = s1[0]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//     assert_eq!(h, "h");
//     let h1 = &s1[3..5];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//     assert_eq!(h1, "中");
// }
// 我的答案
// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//     assert_eq!(h, "h");
//     let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//     assert_eq!(h1, "中");
// }
// 官方答案
// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; 
//     assert_eq!(h, "h");
//     let h1 = &s1[3..6];
//     assert_eq!(h1, "中");
// }

// --- 操作 UTF-8 字符串 ---

// test 12
// 题目-填空，打印出 "你好，世界" 中的每一个字符
// fn main() {
//     for c in "你好，世界".__ {
//         println!("{}", c)
//     }
// }
// 我的答案
// fn main() {
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }
// 官方答案
// fn main() {
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }

// ---  utf8_slice ---
// 可以使用三方库 utf8_slice 来访问 UTF-8 字符串的某个子串，
// 但是与之前不同的是，该库索引的是字符，而不是字节.
// 示例
// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";
//     let rocket = utf8_slice::slice(s, 4, 5);
//     // 结果是 "🚀"
// }