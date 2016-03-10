/*
When a Process goes out of scope, its drop method will wait until the child 
process finishes before releasing the resource.
*/

use std::process::Command;

fn main() {
    let _process = Command::new("sleep").arg("5").spawn();

    println!("reached end of main");
}

