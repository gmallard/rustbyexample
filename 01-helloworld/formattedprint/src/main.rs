
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => (
        // The macro will expand into the contents of this block.
        println!("Hello from a macro!");
    )
}

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // Does *not* work.  'h' format unknown.
    // println!("Maybe decimal {} maybe hex {:h}", 14, 14);

    // OK, 'x' for hex
    println!("Maybe decimal {} maybe hex {:x}", 14, 14);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure which contains an `i32`. Name it `Structure`.
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    //* println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    // local macro - for fun
    say_hello!();

    //
    let n = 22.0;
    let d = 7.0;
    let r = n / d;
    let rf = format!("{:.*}", 3, r);
    println!("Pi is roughly {}", rf);
}

