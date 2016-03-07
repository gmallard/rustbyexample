/*
Closures (also called lamdas or anonymous functions) in Rust are functions 
with a slightly specialized syntax which can capture the enclosing environment. 
Their syntax and capabilities make them very convenient for on the fly usage. 
Some characteristics include:

1) uses || instead of () around input variables.
2) both input and return types can be inferred.
3) input variable names must be specified.
4) body delimination ({}) is optional for a single expression. Mandatory otherwise.
5) the outer environment variables may be captured.
6) calling a closure is exactly like a function: call(var).
*/

fn main() {
    // Increment via closures and functions.
    fn  function            (i: i32) -> i32 { i + 1 }

    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    // It is possible to capture variables from the enclosing
    // environment; something which is impossible with functions.
    let professor_x = "Charles Xavier";

    // A closure which takes no argument, returning nothing, prints
    // a variable from the enclosing scope.
    let print = || println!("Professor X's name is: {}", professor_x);

    // Call the closure.
    print();
}

