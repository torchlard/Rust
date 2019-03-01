// struct User {
//   username: String,
//   email: String,
//   sign_in_count: u64,
//   active: bool
// }

// struct Rectangle {
//   width: u32,
//   height: u32
// }

// impl Rectangle {
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }
//   fn can_hold(&self, other: &Rectangle) -> bool{
//     self.width > other.width && self.height > other.height
//   }
//   fn adds (x: i32, y: i32) -> i32 {
//     x+y
//   }
// }

// enum IpAddrKind {
//   V4, V6
// }

// enum IpAddr2 {
//   V4(u8, u8, u8, u8),
//   V6(String)
// }

// struct IpAddr {
//   kind: IpAddrKind,
//   address: String
// }

// enum Optional<T> {
//   Some(T), None
// }

// enum Coin {
//   Penny, Nickel, Dime, Quarter(i32)
// }

// fn value_in_cents(coin: Coin) -> u32 {
//   match coin {
//     Coin::Penny => 1,
//     Coin::Nickel => 5,
//     Coin::Dime => 10,
//     Coin::Quarter(state) => {
//       println!("state quarter from {:?}!", state);
//       25
//     }
//   }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1)
  }
}

fn main() {

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("{}, {:?}, {:?}", five.unwrap(), six, none);

  // println!("{}", value_in_cents(Coin::Quarter(3)));

  // let some_number = Some(5);
  // let some_string = Some("a string");
  // let absent_num: Option<i32> = None;
  
  // let home = IpAddr2::V4(127, 0, 0, 1);
  // let loopback = IpAddr2::V6(String::from("::1"));

  // let home = IpAddr {
  //   kind: IpAddrKind::V4,
  //   address: String::from("127.0.0.1")
  // };
  // let loopback = IpAddr {
  //   kind: IpAddrKind::V6,
  //   address: String::from("::1")
  // };
  
  // let user1 = build_user(String::from("abc"));
  // let user2 = User {
  //   email: String::from("email2"),
  //   username: String::from("ant456"),
  //   ..user1
  // };

  // println!("{}", user1.email);
  // println!("{}", user2.email);
  // println!("{}", user2.active);

  // let rect1 = (30, 50);
  // println!("area of rect is {:#?}", rect1);

  // let rect1 = Rectangle{ width: 30, height: 50};
  // let rect2 = Rectangle{ width: 10, height: 70};
  // let rect3 = Rectangle{ width: 20, height: 20};
  // println!("area is {}", rect1.area());
  // println!("rect 1 can hold 3? {}", rect1.can_hold(&rect3));
  // println!("{}", Rectangle::adds(1,2));
}

// fn area(dimensions: (u32, u32)) -> u32 {
//   dimensions.0 * dimensions.1
// }

// fn build_user(email: String) -> User {
//   User {
//     email: email,
//     username: String::from("some_usernames"),
//     active: true,
//     sign_in_count: 1
//   }
// }


















