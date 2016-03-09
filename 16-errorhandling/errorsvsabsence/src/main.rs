/*
Previously, we have used the type Option to annotate that absence is a 
possibility. This absence sometimes appears as an error, for example when None 
is unwrapped. In the more general case where there may be multiple failure 
points for a multitude of different reasons, an Option can be replaced by the 
more general Result type. A Result<T, E> has these variants:

1) Ok<T>: An element T was found
2) Err<E>: An error was found with element E

Similar to Option, Result also contains the unwrap() method which yields the 
element T or calls panic!(). So far, this should seem similar to Option:
*/

fn double_number(number_str: &str) -> i32 {
    // It might not always be possible to parse a string into the other type
    // so `parse()` returns a `Result` indicating possible failure. Let's
    // just try `unwrap()` to get the number out. Will it bite us?
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}

/*
Clearly, panicking on an Err leaves an unhelpful error message. Do we even know 
anything about libcore that the error is telling us all about? There must be a 
better way.
*/


