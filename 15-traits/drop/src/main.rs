/*
The Drop trait only has one method: drop, and this method is called when an 
object goes out of scope. The main use of the Drop trait is to free the 
resources that the implementor instance owns.

Box, Vec, String, File and Process are some examples of types that implement 
the Drop trait to free resources. The Drop trait can be implemented for any 
custom data type.
*/

struct Droppable {
    name: &'static str,
}

// This `drop` implementation doesn't free resources; but instead reports its
// usage via a print to the console
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line : DONE - works, but the drop of a
    // occurs after main function exit.

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}

