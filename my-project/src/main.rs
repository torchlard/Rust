// enum List {
//   Cons(i32, Box<List>),
//   Nil
// }

// use List::{Cons, Nil};
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.0
  }
}

fn hello(name: &str){
  println!("hello, {}", name);
}

fn main() {

  let m = MyBox::new(String::from("Rust"));
  hello(&(*m)[..]);
  hello(&m);

  // let list = Cons(1,
  //   Box::new(Cons(2,
  //     Box::new(Cons(3,
  //       Box::new(Nil))))));

  // let x = 5;
  // let y = &x;
  // let z = &y;

  // println!("{},{:?},{}", x, &x,*y);

  // let x = 5;
  // let y = Box::new(x);
  // assert_eq!(5, x);
  // assert_eq!(5, *y);
  // println!("b = {}", b);

}
















