// enum List {
//   Cons(i32, Box<List>),
//   Nil
// }

// enum List {
//   Cons(i32, Rc<List>), Nil
// }

// use List::{Cons, Nil};
// use std::rc::Rc;

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//   fn new(x: T) -> MyBox<T> {
//     MyBox(x)
//   }
// }

// impl<T> Deref for MyBox<T> {
//   type Target = T;
//   fn deref(&self) -> &T {
//     &self.0
//   }
// }

// fn hello(name: &str){
//   println!("hello, {}", name);
// }

// pub trait Messager {
//   fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: 'a+Messager> {
//   messager: &'a T,
//   value: usize,
//   max: usize
// }

// impl<'a, T> LimitTracker<'a, T> where T: Messager {
//   pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
//     LimitTracker {
//       messager, value: 0, max
//     }
//   }

//   pub fn set_value(&mut self, value: usize) {
//     self.value = value;
//     let percentage = self.value as f64 / self.max as f64;
    
//     if percentage >= 1.0 {
//       self.messager.send("error: over quita");
//     } else if percentage >= 0.9 {
//       self.messager.send("urgent warning");
//     } else if percentage >= 0.75 {
//       self.messager.send("warning");
//     }
//   }
// }

// mod tests {
//   use super::*;
//   use std::cell::RefCell;

//   struct MockMessenger {
//     sent_messages: RefCell<Vec<String>>
//   }

//   impl MockMessenger {
//     fn new() -> MockMessenger {
//       MockMessenger { sent_messages: RefCell::new(vec![]) }
//     }
//   }

//   impl Messager for MockMessenger {
//     fn send(&self, message: &str) {
//       self.sent_messages.borrow_mut().push(String::from(message));
//     }
//   }

//   #[test]
//   fn send_75() {
//     assert_eq!(mock_messager.send_messages.borrow().len(), 1);
//   }
// }

// use List::{Cons, Nil};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// #[derive(Debug)]
// enum List {
//   Cons(i32, RefCell<Rc<List>>), Nil
// }

// impl List {
//   fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//     match self {
//       Cons(_, item) => Some(item),
//       Nil => None
//     }
//   }
// }

#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>
}



fn main() {

  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );

  {
      let branch = Rc::new(Node {
          value: 5,
          parent: RefCell::new(Weak::new()),
          children: RefCell::new(vec![Rc::clone(&leaf)]),
      });

      *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

      println!(
          "branch strong = {}, weak = {}",
          Rc::strong_count(&branch),
          Rc::weak_count(&branch),
      );

      println!(
          "leaf strong = {}, weak = {}",
          Rc::strong_count(&leaf),
          Rc::weak_count(&leaf),
      );
  }

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );

  // let leaf = Rc::new(Node {
  //   value: 3,
  //   parent: RefCell::new(Weak::new()),
  //   children: RefCell::new(vec![])
  // });

  // println!("leaf parent {:?}", leaf.parent.borrow().upgrade());

  // let branch = Rc::new(Node {
  //   value: 5,
  //   parent: RefCell::new(Weak::new()),
  //   children: RefCell::new(vec![Rc::clone(&leaf)])
  // });

  // let branch2 = Rc::new(Node {
  //   value: 5,
  //   parent: RefCell::new(Weak::new()),
  //   children: RefCell::new(vec![Rc::clone(&branch)])
  // });

  // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
  // *branch.parent.borrow_mut() = Rc::downgrade(&branch2);
  // println!("leaf parent {:#?}", leaf.parent.borrow().upgrade());
  // println!("branch parent {:#?}", branch.parent.borrow().upgrade());

  // println!("strong count branch {}", Rc::strong_count(&leaf));
  // println!("weak count branch {}", Rc::weak_count(&leaf));

  // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  // println!("a initial rc count = {}", Rc::strong_count(&a));
  // println!("a next item = {:?}", a.tail());

  // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  // println!("a initial rc count = {}", Rc::strong_count(&a));
  // println!("b initial rc count = {}", Rc::strong_count(&b));
  // println!("b next item = {:?}", b.tail());

  // if let Some(link) = a.tail() {
  //   *link.borrow_mut() = Rc::clone(&b);
  // }
  // println!("b rc count = {}", Rc::strong_count(&b));
  // println!("a rc count = {}", Rc::strong_count(&a));

  // println!("a next item = {:?}", a.tail());



  // let x = 5;
  // let y = &mut x;

  // let a = Rc::new(Cons(5, Rc::new(
  //   Cons(10, Rc::new(Nil)))));
  // println!("1. {}", Rc::strong_count(&a));
  // let b = Cons(3, a.clone());
  // println!("2. {}", Rc::strong_count(&a));
  // {
  //   let c = Cons(4, Rc::clone(&a));
  //   println!("3. {}", Rc::strong_count(&a));
  // }
  // println!("4. {}", Rc::strong_count(&a));


  // let a = Cons(5, Box::new(Cons(
  //   10, Box::new(Nil)
  // )));
  // let b = Cons(3, Box::new(a));
  // let c = Cons(4, Box::new(a));

  // let m = MyBox::new(String::from("Rust"));
  // hello(&(*m)[..]);
  // hello(&m);

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
















