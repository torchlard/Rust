
fn main() {

  // let s = String::from("hello");
  // println!("here {}", s);
  // takes_ownership(s);


  // let x = 5;
  // make_copy(x);
  // println!("x after {}", x);

  // let s1 = gives_ownership();
  // let s2 = String::from("hello");
  // let s3 = takes_and_gives_back(s2);

  // println!("{}, {}", s1, s3);

  // let s1 = String::from("hello");
  // let len = calculate_length(&s1);
  // println!("length of '{}' is {}.", s1, len);

  // let ref_to_nothing = dangle();

  // let my_str = String::from("hello world");
  // let word = first_word2(&my_str[..]);
  // let my_str_literal = "hello world";
  // let word = first_word2(&my_str_literal[..]);
  // let word = first_word2(my_str_literal);

  let a = [1,2,3,4,5];
  

}

fn first_word2(s: &str) -> &str {
  let bytes = s.as_bytes();
  for(i, &item) in bytes.iter().enumerate(){
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

// fn first_word(s: &String) -> usize {
//   let bytes = s.as_bytes();
//   // index, ref to element
//   for (i, &item) in bytes.iter().enumerate(){
//     if item == b' ' {return i;}
//   }
//   s.len()
// }

// fn dangle() -> String {
//   let s = String::from("hello");
//   s
// }

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// fn takes_ownership(some_str: String) {
//   println!("{}", some_str);
// }

// fn make_copy(some_int: i32){
//   println!("{}", some_int);
// }

// fn gives_ownership() -> String {
//   let some_str = String::from("kell");
//   some_str
// }

// fn takes_and_gives_back(a_str: String) -> String {
//   a_str + "x"
// }









