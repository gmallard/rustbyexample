/*
Rust provides pattern matching via the match keyword, which can be used like 
a C switch.
*/

fn try_number(number: i32) {
    println!("\nTell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13...19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
}

fn main() {

    try_number(13);
    try_number(1);
    try_number(5);
    try_number(42);

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("\n{} -> {}", boolean, binary);
}
