/*
Rust has two different types of constants which can be declared in any 
scope including global. Both require explicit type annotation:

1) const: An unchangeable value (the common case).
2) static: A possibly mutable variable with 'static lifetime.

One special case is the "string" literal. It can be assigned directly to 
a static variable without modification because its type signature: 
&'static str has the required lifetime of 'static. All other reference 
types must be specifically annotated so that they fulfill 
the 'static lifetime. This may seem minor though because the required 
explicit annotation hides the distinction.
*/

// Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let mut n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    n = 9;
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}

