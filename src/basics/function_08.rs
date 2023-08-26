
pub fn main() {
  hello();
  println!("get name {}", get_name());

  let price = 66;
  double_price(price);
  println!("primary price: {}", price);

  let mut price = 66;
  double_price_index(&mut price);
  println!("primary price: {}", price);

}

fn hello() {
  println!("Hello");
}

fn get_name() -> String {
  return String::from("Kitty");
}

// 内部 price 不会影响外部 price 的值
fn double_price(mut price: i32) {
  price = price * 2;
  return println!("double price {}", price);
}

// 引用传递，会将内存地址传递给函数，表示共用一个内存地址，传递参数需要加 & 标记
fn double_price_index(price:&mut i32) {
  // * 引用符，用户指向 price 内存位置并修改值
  *price = *price * 2;
  return println!("double price {}", price);
}



