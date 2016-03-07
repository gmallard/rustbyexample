/*
Tuples can be destructured in a match as follows:
*/

fn try_pair(pair: (i32, i32)) {
  println!("\nTell me about {:?}", pair);
  // Match can be used to destructure a tuple
  match pair {
      // Destructure the second
      (0, y) => println!("First is `0` and `y` is `{:?}`", y),
      (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
      _      => println!("It doesn't matter what they are"),
      // `_` means don't bind the value to a variable
  }
}

fn main() {

    try_pair((0, -2));
    try_pair((-2, 0));
    try_pair((-10, 42));

}

