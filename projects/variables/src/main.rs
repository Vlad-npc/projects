fn main() {
    let x = 5; //unmutable variable

    let x = x + 1; // declaration of a variable with the same name, shadowing is used here
    
    {
        let x = x * 2;

        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
