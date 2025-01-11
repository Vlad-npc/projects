use std::io;

fn main() {
    println!("What do you want to convert? Type F for Fahrenheit to Celsius or C for Celsius to Fahrenheit.");

    let mut convert = String::new();

    io::stdin()
        .read_line(&mut convert)
        .expect("Failed to read line");

    let convert: char = convert.trim().parse().expect("REASON");

    if convert == 'C' {

        println!("Type the temperature in Celsius.");

        let mut c = String::new();

        io::stdin()
            .read_line(&mut c)
            .expect("Failed to read line");

        let c: f32 = c.trim().parse().expect("REASON");

        let convert_to_f: f32 = (c *9.0/5.0) + 32.0;
        println!("The Celsius temperature convets to F = {convert_to_f} ");
    } 
    else {
        println!("Type the temperature in Fahrenheit.");

        let mut f = String::new();

        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");

        let f: f32 = f.trim().parse().expect("REASON");

        let convert_to_c: f32 = (f - 32.0) * (5.0/9.0);
        println!("The Fahrenheit temperature convets to C = {convert_to_c} ");
    }
}
