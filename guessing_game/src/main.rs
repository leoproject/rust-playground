// The line below  is the io library comes from the standard library, known as std:
use std::io; 

fn main() {
    println!("Guessing the number!");

    println!("Please input your guess.");
    //  let name (immutable) or let mut age (mutable)
    let mut guess = String::new();

    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
    println!("You guessed: {guess}");
}
