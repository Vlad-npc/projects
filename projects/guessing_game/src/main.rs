use std::io; // io library that comes from the standard library. input/output library
use std::cmp::Ordering; // from standard lib
use rand::Rng; // from the crate rand with the Rng trait
fn main() {
    println!("Guess the number!"); // println! macro to print a string to the screen.

    let secret_number = rand::thread_rng().gen_range(1..=100); // funktion with a particular random number generator
    // gen_range gives the range of numbers and is inclusive on the lower and upper bounds
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // mut = mutable, let apples = 5; is by default immutable 
    // has created a mutable variable that is currently bound to a new, empty instance of a String.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) { //cant compare string with integer
        Ordering:: Less => println!("Too small!"),
        Ordering:: Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
