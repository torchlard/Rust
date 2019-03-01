// fn largest<T>(list: &[T]) -> T {
//   let mut largest = list[0];

//   for &item in list.iter() {
//     if item > largest {
//       largest = item;
//     }
//   }
// }

// struct Point2<T,U> {
//   x: T,
//   y: U
// }

// struct Point<T> {
//   x: T,
//   y: T
// }

// impl<T> Point<T> {
//   fn x(&self) -> &T {
//     &self.x
//   }
// }

// impl<T,U> Point2<T,U> {
//   fn mixup<V,W>(self, other: Point2<V,W>) -> Point2<T,W> {
//     Point2 {
//       x: self.x, 
//       y: other.y
//     }
//   }
// }

pub trait Summary {
  fn summarize(&self) -> String {
    String::from("default read from ...")
  }
  fn add(&self) -> i32 {
    1
  }
  // fn sub(&self) -> i32;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{} by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

// must implement Copy trait so that value can be moved out of list[0] to largest
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

use std::fmt::Display;

struct Pair<T> {
  x: T, y: T
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {x, y}
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("largest x = {}", self.x);
    } else {
      println!("largest y = {}", self.y);
    }
  }
}

impl<T: Display> ToString for T {}

fn main(){



  // pub fn notify(item: impl Summary) {
  //   println!("breaking news! {}", item.summarize());
  // }
  // pub fn notify<T: summary> (item: T) {
  //   println!("Breaking news! {}", item.summarize());
  // }


  let tweet = Tweet {
    username: String::from("horse_ebook"),
    content: String::from("of coz, as you"),
    reply: false,
    retweet: false
  };
  println!("1 new tweet: {}", tweet.summarize());
  println!("{}", tweet.add());
  
  // let p = Point { x: 'a', y: 'b' };
  // let p2 = Point { x: 2, y: 5 };
  // let q1 = Point2{ x: 5, y: 10.0};
  // let q2 = Point2{ x: "hello", y: 'w'};

  // println!("p.x = {}", p.x());
  // let mix = q1.mixup(q2);
  // println!("after mixup: {}, {}", mix.x, mix.y);

  
  // let number_list = vec![34, 50, 25, 100, 65];

  // let result = largest(&number_list);
  // println!("largest num is {}", result);

  // let char_list = vec!['y', 'm', 'a', 'q'];
  // let result = largest(&char_list);
  // println!("largest char is {}", result);

}


