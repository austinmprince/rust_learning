use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");
    println!("Please input a number!");

    let mut guess = String::new();

    let guess:i32 = guess.trim().parse().expect("Please type a number")
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");
    println!("Please input your guess");
    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line!");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Equal => println!("You got it!"),
      Ordering::Greater => println!("Too big!"),
    }
    
    println!("You got it! The secret number is {guess}");
  
    
}


