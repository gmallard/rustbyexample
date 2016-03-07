/*
A match guard can be added to filter the arm.
*/

fn try_pair(pair: (i32, i32)) {
    println!("\nTell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn main() {
    try_pair((2, -2));
    try_pair((9, 9));
    try_pair((9, 42));
    try_pair((22, 26));
}

