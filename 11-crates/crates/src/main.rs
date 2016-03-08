/*
To link a crate to this new library, the extern crate declaration must be used. 
This will not only link the library, but also import all its items under a 
module named the same as the library. The visibility rules that apply to 
modules also apply to libraries.
*/

// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}

