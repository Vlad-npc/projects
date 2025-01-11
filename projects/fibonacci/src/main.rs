use std::io;

fn main() {

    let mut prior1 = 0;
    let mut prior2 = 1;

    println!("Calculating the n-th Fibonacci number. Enter number.");
    loop {
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n : u128 = n.trim().parse().expect("REASON");

        
        if n < 1 {
            println!("Invalid input.");
            continue;
        }
        if n == 1 {
            println!("The {n}-th Fibonacci number is 1.");
            break;
        }
        else {
            let mut current = 0;
            for _number in 1..n {
                current = prior1 + prior2;
                prior1 = prior2;
                prior2 = current;
            }
            println!("The {n}-th Fibonacci number is {current}.");
            break;
        }
    }
}
