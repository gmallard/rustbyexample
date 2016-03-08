/*
The crate_type attribute can be used to tell the compiler whether a crate is 
a binary or a library (and even which type of library), and the crate_name 
attribute can be used to set the name of the crate.

To build:

$ rustc lib.rs
$ ls lib*
library.rlib
*/

// lib.rs
// This crate is a library
#![crate_type = "lib"]

// Changed for 'cargo' build:
// The library is named "rary"
// #![crate_name = "rary"]
#![crate_name = "crates"]

pub fn public_function() {
    println!("called rary's(crate's) `public_function()`");
}

fn private_function() {
    println!("called rary's(crate's) `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's(crate's) `indirect_access()`, that\n> ");

    private_function();
}

