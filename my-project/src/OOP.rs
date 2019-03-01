// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   if x.len() > y.len() {x} else {y}
//   // x
// }

// struct ImportantExcerpt<'a> {
//   part: &'a str
// }

// use std::thread;
// use std::time::Duration;

// fn simulated_epensive(intensity: u32) -> u32 {
//   println!("calc slowly...");
//   thread::sleep(Duration::from_secs(2));
//   intensity
// }

// let expensive_closure = |num| {
//   println!("calc slowly...");
//   thread::sleep(Duration::from_secs(2));
//   num
// };

// fn generate_workout(intensity: u32, random_number: u32){}

// impl<T> Cacher<T> where T: Fn(u32) -> u32 {
//   fn new(calc: T) -> Cacher<T> {
//     Cacher {
//       calc, value: None
//     }
//   }
//   fn value(&mut self, arg: u32) -> u32 {
//     match self.value {
//       Some(v) => v,
//       None => {
//         let v = (self.calc)(arg);
//         self.value = Some(v);
//         v
//       }
//     }
//   }
// }

// #[derive(PartialEq, Debug)]
// struct Shoe {
//   size: u32,
//   style: String
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//   shoes.into_iter().filter(|s| s.size == shoe_size)
//        .collect()
// }

// struct Counter {
//   count: u32
// }

// impl Counter {
//   fn new() -> Counter {
//     Counter { count: 0 }
//   }
// }

// impl Iterator for Counter {
//   type Item = u32;
//   fn next(&mut self) -> Option<Self::Item> {
//     self.count += 1;
//     if self.count < 6 {Some(self.count)} else {None}
//   }
// }

pub trait Draw {
  fn draw(&self);
}

pub struct Screen<T: Draw> {
  // pub components: Vec<Box<dyn Draw>>
  pub components: Vec<T>
}

// impl Screen {
//   pub fn run(&self) {
//     for component in self.components.iter(){
//       component.draw();
//     }
//   }
// }

impl<T> Screen<T> where T: Draw {
  pub fn run(&self){
    for components in self.components.iter(){
      components.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String
}

impl Draw for Button {
  fn draw(&self){

  }
}

use gui::Draw;
struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>
}

impl Draw for SelectBox {
  fn draw(&self){

  }
}

use gui::{Screen, Button};

fn main() {

  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No")
        ]
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK")
      })
    ]
  };

  screen.run();

  // let counter = Counter::new();

  // let sum: u32 = counter.sum();
  // println!("{}", sum);

  // let shoes = vec![
  //   Shoe {size: 10, style: String::from("sneaker")},
  //   Shoe {size: 13, style: String::from("sandal")},
  //   Shoe {size: 10, style: String::from("boot")}
  // ];

  // let in_my_size = shoes_in_size(shoes, 10);
  // println!("{:?}", in_my_size);

  // let v1 = vec![1,2,3];
  // let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
  // println!("{:?}", v2);

  // let mut v1_iter = v1.iter();

  // println!("{:?}", v1_iter.next());
  // println!("{:?}", v1_iter.next());
  // println!("{:?}", v1_iter.next());
  // println!("{:?}", v1_iter.next());
  // println!("{:?}", v1_iter.next());

  // fn add_v1(x: u32) -> u32 {x+1}
  // let add_v2 = |x: u32| -> u32 {x+1};
  // let add_v3 = |x: u32| { x+1 };
  // let add_v4 = |x: u32| x+1;

  // println!("{}", add_v4(3));


  // let simulated_val = 10;
  // let simulated_random_num = 7;

  // generate_workout(simulated_val, simulated_random_num);

  // let novel = String::from("call me Ishamel. some");
  // let first_sentence = novel.split('.')
  //       .next().expect("could not find '.'");
  // let i = ImportantExcerpt {part: first_sentence};

  // let string1 = String::from("long string is long");
  // let result;
  // {
  //   let string2 = String::from("xyz");
  //   result = longest(string1.as_str(), string2.as_str());
  //   println!("longest str is {}", result);
  // }
  
}





















