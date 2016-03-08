/*
It has been noted that Rust chooses how to capture variables on the fly without 
annotation. This is all very convenient in normal usage however when writing 
functions, this ambiguity is not allowed. The closure's complete type, 
including which capturing type, must be annotated. The manner of capture a 
closure uses is annotated as one of the following traits:

1) Fn: takes captures by reference (&T)
2) FnMut: takes captures by mutable reference (&mut T)
3) FnOnce: takes captures by value (T)

Even annotated, these are very flexible: a parameter of FnOnce specifies the 
closure may capture by T or &mut T or &T at will (if a move is possible, 
any type of borrow should also be possible). The reverse is not true: if the 
parameter is Fn, then nothing lower is allowed. Therefore, the rule is:

any annotated parameter restricts capture to itself and above

In addition, Rust will preferentially capture variables in the least 
restrictive manner possible on a variable-by-variable basis:
*/

// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`. : DONE, compile
    // errors.  Not intuitive wording!

    f()
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}


