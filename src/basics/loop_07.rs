pub fn main () {
  // 1 ~ 4
  for num in 1..5 {
    println!("num is {}", num);
  }

  // 1 ~ 5
  for num in 1..=5 {
    println!("num is {}", num);
  }

  let name_list = vec![
    "Kalian",
    "Helen",
    "Kitty"
  ];
  // iter() 每次迭代获取集合的一个元素
  for name in name_list.iter() {
    match name {
      &"Kalian" => println!("first name is {:?}", name),
      _ => println!("other name is {:?}", name)
    }
  }
  // name_list 依旧可以使用
  println!("name_list is {:?}", name_list);
  

  let name_move_list = vec![
    "Kalian",
    "Helen",
    "Kitty"
  ];
 // into_iter 会消耗集合，循环的同时会逐步移出集合
  for name in name_move_list.into_iter() {
    match name {
      "Kalian" => println!("first name is {:?}", name),
      _ => println!("other name is {:?}", name)
    }
  }

  let mut name_mut_list = vec![
    "Kalian",
    "Helen",
    "Kitty"
  ];
  // iter_mut 可以修改集合内元素的值
  for name in name_mut_list.iter_mut() {
    *name = match name {
      &mut "Kalian" => {"first name is Kalian"},
      _ => *name
    }
  }
  println!("name_mut_list: {:?}", name_mut_list);

  let mut num = 1;
  while num < 10 {
    println!("num: {:?}", num);
    num = num * 2;
  }

  let mut num = 1;
  // loop 没有 break 会一直循环
  loop {
    if num > 10 { break; };
    println!("num: {:?}", num);
    num = num * 3;
  }

}