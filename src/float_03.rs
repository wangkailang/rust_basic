pub fn main() {
  let float_default = 18.00; // 默认是 f64
  let float_f32:f32 = 6.6;
  let float_f64:f64 = 8.8;
  let float_underline = 1_000_000.666_111_222;

  println!("float_default {}", float_default);
  println!("float_f32 {}", float_f32);
  println!("float_f64 {}", float_f64);
  println!("float_underline {}", float_underline);

  // Error examples
  // 整数不能覆盖浮点类型
  // let float_int:f32 = 66;
}