/*
Indirectly accessing a variable makes it impossible to branch and use that 
variable without re-binding. match provides the @ sigil for binding values 
to names:
*/
// A function `age` which returns a `u32`.
fn age(r: u32) -> u32 {
    r
}

fn try_match(d: u32) {
    println!("\nTell me type of person you are, at {}", d);

    match age(d) {
        0             => println!("I'm not born yet I guess"),
        // Could `match` 1 ... 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 .. 12. Now the age can be reported.
        n @ 1  ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}

fn main() {

  try_match(15);
  try_match(0);
  try_match(7);
  try_match(101);
}

