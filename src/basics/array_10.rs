pub fn main() {
  let array_name: [&str;3] = ["Kalian", "Macintosh", "Kitten"];
  let array_default = ["";3];
  let inner_array_name = ["Kalian", "Macintosh", "Kitten"];
  let mut mut_array_name = ["Kalian", "Macintosh", "Kitten"];

  println!("array length: {}", array_name.len());
  println!("array default: {:?}", array_default);

  // loop array
  for item in array_name {
    println!("name: {}", item);
  }

  // update array
  inner_update_array(inner_array_name);
  // 只会影响函数作用域内的值，原数值不变
  println!("primary mut array name: {:?}", inner_array_name);

  // 引用传递，影响外部数值
  update_array(&mut mut_array_name);
  // 返回改变的数组
  println!("primary mut array name: {:?}", mut_array_name);
}

fn inner_update_array(mut arr: [&str;3]) {
  let len = arr.len();
  for i in 0..len {
    if i == 0 {
      arr[0] = "New Name";
    }
    println!("array item is {}", arr[i]);
  }
}

fn update_array(arr:&mut[&str;3]) {
  let len = arr.len();
  for i in 0..len {
    if i == 0 {
      arr[0] = "New Name";
    } 
    println!("array item is {}", arr[i]);
  }
}
