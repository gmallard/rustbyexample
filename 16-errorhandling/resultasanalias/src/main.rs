/*
What if the specific Result type is reused many many times? Then quickly it 
becomes tedious to write out the full type name. Instead, a generic alias for 
the specific Result may be defined.
*/

use std::num::ParseIntError;
use std::result;

// A generic alias for any `Result` with this specific `Err` type.
type Result<T> = result::Result<T, ParseIntError>;

// Use the alias defined above referring to our specific `Result` type.
fn double_number(number_str: &str) -> Result<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// Again, the alias saved us from defining it again.
fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}

/*
This is particularly helpful at a module level because all errors found in a 
specific module may have the same Err type; a single alias succinctly defines 
all module Results. This is so useful that the std library even supplies one: 
io::Result which refers to IO errors.
*/

