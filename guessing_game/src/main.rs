use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");
    println!("Please input a number!");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");
    println!("Please input your guess");
    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line!");
    println!("You guessed: {guess}");
    
    println!("You got it! The secret number is {guess}");
  
    
}


