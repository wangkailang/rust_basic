pub fn main() {

  // 有符号整型 signed 存储正数或者负数；无符号整型 unsigned 只能存正数
  // 按照存储空间，划分 1、2、4、8、16... 字节，1 字节 = 8 位(0~256 or -128~127)
  let int = 100; // 默认 i32
  let int_u32:u32 = 200;
  let int_i32:i32 = -300;
  let int_isize:isize = 400;
  let int_usize:usize = 500;

  println!("int: {}", int);
  println!("int_u32: {}", int_u32);
  println!("int_i32: {}", int_i32);
  println!("int_isize: {}", int_isize);
  println!("int_usize: {}", int_usize);

  // Error example
  // 浮点被定义为整数
  // let int_float:i32 = 66.6;

  // i8 表示 -128~127 区间，溢出错误
  // let overflow_int:i8 = 130;
  
}