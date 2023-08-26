pub fn main() {
  // 常量用 const
  // 常量用大写字母
  const PI:f64 = 3.1415926;

  // TODO
  static BOOK:&'static str = "<<Rust Guide>>";

  println!("PI: {}", PI);
  println!("BOOK: {}", BOOK);
}