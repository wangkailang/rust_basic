pub fn main() {
  let tuple_name = ("Kilian", "Macintosh", "Kitten");
  println!("names: {:?}, first name: {}", tuple_name, tuple_name.0);
  print_tuple(tuple_name);

  // 解构
  let (name1, name2, _name3) = tuple_name;
  println!("name1: {:?}, name2: {}", name1, name2);
}

fn print_tuple(t: (&str, &str, &str)) {
  println!("{:?}", t);
}