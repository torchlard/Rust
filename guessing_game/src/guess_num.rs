use std::io;
use rand::Rng;
use std::cmp::Ordering;

// use rand crate as external dependency
// then we can call anythin in rand by rand::
extern crate rand;

fn main() {
  println!("Guess the number!");

  // local to current thread of execution, seeded by OS
  let secret_number = rand::thread_rng().gen_range(1,101);
  println!("the secret num is {}", secret_number);

  println!("pls input your guess");

  // static method: ::new is an associated function of sring type
  let mut guess = String::new();
  // &: argument is reference
  // let multiple parts of code access one piece of data without needing copy data into memory multiple times
  io::stdin().read_line(&mut guess)
    .expect("failed to read line");
  // .expect: handle failure with Result type

  // shadow previous value of guess with new one
  // let guess: u32 = guess.trim().parse().expect("pls type a num");
  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    // _ : catch all Err value
    Err(_) => 50,
  };

  // let x = 5;
  // let y = 6;
  // println!("you guessed: {}, x={}, y={}", guess, x, y);

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("too small!"),
    Ordering::Greater => println!("too big"),
    Ordering::Equal => {
      println!("you win");
    }
  }

}
