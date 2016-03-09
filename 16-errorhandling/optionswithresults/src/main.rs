/*
The previous examples have always been very convenient; a Result interacts with 
the same Results and an Option with the same Option. Sometimes it is not this 
easy though; Options and Results may have to interact or even Result<T, Error1> 
with Result<T, Error2>.

Here is an example where one returns an Option and the other returns an Result. 
Aside from messy errors provided by unwrap, this looks reasonable:
*/

// The first attempt conveniently uses `unwrap` with the aforementioned
// bad errors it results in.
fn double_first(vec: Vec<&str>) -> i32 {
    // What if the vector is empty?
    let first = vec.first().unwrap();

    // What if the element doesn't parse to a number?
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("{}", "start");
    println!("The first doubled is {}", double_first(numbers));
    println!("{}", "at A");
    println!("The first doubled is {}", double_first(empty));
    println!("{}", "at B");
    // ^ Comment out this line to see the second error. -> Compile error,
    // *not* runtime (??)
    println!("{}", "at C");
    println!("The first doubled is {}", double_first(strings));
    println!("{}", "done");
}

/*

*/

