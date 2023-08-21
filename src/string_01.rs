pub fn main() {
  // Rust 使用 UTF-8 作为底层编码，而非

  // &str 在模块 std:str，表示字符串切片
  let name = "Kalian";
  let age = 32;
  let man = true;

  // 字符串对象、在堆上分配，提供字符串值以及方法
  // String::new()
  let mut str_new = String::new();
  str_new.push_str("Rust basic");
  // String::from()
  let str_from = String::from("Rust basic");

  // &str 转 String
  let str_method = "Rust basic".to_string();

  // String 转 &str
  let str_index = str_from.as_str();

  // 字符串拼接
  let prefix = "Rust".to_string();
  let suffix = " basic".to_string();
  let join_str = prefix.clone() + &suffix;

  println!("name is {}", name);
  println!("age is {}", age);
  println!("man is {}", man);
  println!("str_new is {}, length is {}", str_new, str_new.len());
  println!("str_from is {}, length is {}", str_from, str_from.len());
  println!("str_method is {}, length is {}", str_method, str_method.len());
  println!("str_index is {}, length is {}", str_index, str_index.len());
  println!("prefix '{}' and suffix '{}' join to '{}'", prefix, suffix, join_str);
}