fn main() {

    for number in 0..12 {

        let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
        let element = days[number];
        println!("On the {element} day of Christmas");
        println!("My true love brought to me");
        if number > 10 {
            println!("Twelve drummers drumming");
        }
        if number > 9 {
            println!("Eleven pipers piping");
        }
        if number > 8 {
            println!("Ten lords a-leaping");
        }
        if number > 7 {
            println!("Nine ladies dancing");
        }
        if number > 6 {
            println!("Eight maids a-milking");
        }
        if number > 5 {
            println!("Seven swans a-swimming");
        }
        if number > 4 {
            println!("Six geese a-laying");
        }
        if number > 3 {
            println!("Five goldenen rings");
        }
        if number > 2 {
            println!("Four calling birds");
        }
        if number > 1 {
            println!("Three French hens");
        }
        if number > 0 {
            println!("Two turtle doves");
        }
        println!("A partridge in a pear");
        println!("");
}
}
