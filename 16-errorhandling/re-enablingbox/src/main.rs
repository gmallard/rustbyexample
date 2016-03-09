/*
We have seen that by implementing Display and From for our error type, we have 
enabled usage of almost all of the std library error handling tools. That is, we 
missed one capability: the ability to easily box our error type.

Namely, the std library will automatically convert from any type which 
implements the Error trait into the trait object Box<Error> via From. To a 
library user, this conveniently allows the following:

// Any error type automatically convertible to `Box<Error>` may be used here.
fn foo(...) -> Result<T, Box<Error>> { ... }

For example, a user may use a variety of libraries which each provide their own 
error types. In order to define a valid Result<T, E> type, the user has a few 
choices:

1) define a new wrapper error type around the external libraries error types
2) convert it to String or some other intermediate choice
3) box it up into Box<Error> via type erasure

Boxing it is a common choice. The only penalty is that the underlying error type 
is only known at runtime and not statically determined. All that needs to be 
done to enable this is implement the Error trait:

trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}

By implementing this, our previous example would be just as valid when the error 
type is Box<Error> as it was before with DoubleError.
*/

use std::error;
use std::fmt;
use std::num::ParseIntError;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // A very short description of the error. Doesn't need to be the
            // same as `Display`.
            DoubleError::EmptyVec => "empty vectors not allowed",
            // This already impls `Error`, so defer to its own implementation.
            DoubleError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // No underlying cause so return `None`.
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

/*

*/

