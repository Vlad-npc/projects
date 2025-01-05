fn main() {
    let mut x = 5; // mutable number, otherwise an error occurs in the compiler
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //declaration of constants
}
