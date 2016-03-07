/*
Since closures are possible as arguments, you might wonder if functions are 
possible as well. Indeed they are! The previously mentioned Fn, FnMut, and 
FnOnce traits all dictate in what fashion a closure captures variables 
from the enclosing scope. Because a function can never capture variables, 
closures are strictly more flexible. Therefore, any function which can take 
a closure as an argument can also take a function.
*/
// A function which takes a closure as an argument and calls it.
fn call_function<F: Fn()>(f: F) {
    f()
}

fn print() {
    println!("I'm a function! I can be used like a closure.")
}

fn main() {
    let closure = || println!("I'm a closure!");
    call_function(closure);

    call_function(print);
}

