/*
The previous problem was awkward because avoiding unwrap forced us to nest 
deeper and deeper when what we really wanted was to get the variable out. So, 
is there any way to accomodate this approach without panic? Well, what is a 
valid action to take when an Err is found? It turns out there are two:

1) panic! which we already decided to try to avoid if possible
2) return because an Err means it cannot be handled

This is exactly the purpose of try!; it is almost exactly equivalent to an 
unwrap which returns instead of panics on Errs.
*/

use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// Setup to make this work. Create two files with some info.
fn setup() -> std::io::Result<()> {
    let mut a = try!(File::create("a"));
    try!(a.write_all(b"grape"));

    let mut b = try!(File::create("b"));
    b.write_all(b"fruit")
}

// Get the data from each file with the data stored in a `Result`.
fn get_data(path: &str) -> Result<String> {
    // `try` unwraps the value or returns the error.
    let mut file = try!(File::open(path)
        // Errors still must be converted to strings.
        .map_err(|err| err.to_string())
    );
    let mut contents = String::new();

    // Read the data into `contents`.
    try!(file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())
    );

    Ok(contents)
}

// Concat the contents of the two files together into a new `Result`.
fn concat(a: &str, b: &str) -> Result<String> {
    let (data_a, data_b) = (try!(get_data(a)), try!(get_data(b)));

    Ok(data_a + &data_b)
}

fn main() {
    // Ignore this result.
    setup().unwrap();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/*
This really is a huge improvement but there is still the nagging issue of 
map_err. There is actually a way to avoid it (we are using it everywhere it 
seems) but we are still missing some details. First, we have to learn how to 
make better errors.
*/

