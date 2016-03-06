/*
There are three types of structures ("structs") that can be created using the 
struct keyword:

1) Tuple structs, which are, basically, named tuples.
2) The classic C structs
3) Unit structs, which are field-less, are useful for generics.

*/

// A unit struct
struct Nil;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, f64);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    println!("my_x: {:}", my_x);
    println!("my_y: {:}", my_y);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };
    println!("A rectangle: {:?}", _rectangle);

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("A Pair: {:?}", pair);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

