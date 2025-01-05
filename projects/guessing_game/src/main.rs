use std::io; // io library that comes from the standard library. input/output library

fn main() {
    println!("Guess the number!"); // println! macro to print a string to the screen.

    println!("Please input your guess.");

    let mut guess = String::new(); // mut = mutable, let apples = 5; is by default immutable 
    // has created a mutable variable that is currently bound to a new, empty instance of a String.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
