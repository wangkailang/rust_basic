pub fn main() {
  // Rust 所有权是内存管理机制，可以避免内存安全问题，并且不需垃圾回收器
  // 每个值都有所有者，当所有者超出范围，值将被释放（安全性、有效性）
  // 1. 每个值只能有一个所有者
  // 2.所有权可以移动或借用转移
  // 3. 移动所有权意味着权限转移，原来的所有者将不再有效
  // 4.借用只是暂时使用，原所有者依旧拥有所有权
  // 5. 引用属于特殊的借用，允许在不获取所有权时访问值

  // 通过这些所有权规则，Rust 可以在编译时检测到许多常见的内存错误，如空指针引用、数据竞争和内存泄漏。

  let str1 = String::from("Hello, Rust");
  let str2 = str1; // str1 的使用权移动到 str2
  println!("{}", str2); // 正常打印
  // println!("{}", str1); // Error: value borrowed here after move

  let str11 = String::from("Hello, Rust");
  let len = calculate_length(&str11); // 借用 str11 的所有权
  println!("{}", len); // 正常打印
  println!("{}", str11); // 正常打印

  let str111 = String::from("Hello, Rust");
  let reference = &str111; // 引用值，不获取所有权
  println!("{}", reference); // 正常打印
  println!("{}", str111); // 正常打印
}

fn calculate_length(s: &String) -> usize {
  return s.len();
}


