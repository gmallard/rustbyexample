/*
Generics is the topic of generalizing types and functionality to broader cases. 
This is extremely useful in reducing code duplication in many ways, but 
requires a rather involving syntax. However, we will find that being generic 
involves taking great care to specify over what types a generic type is 
actually considered valid.

A type parameter is specified as generic by the use of angle brackets and 
camel case: <A, B, ...>. "Generic type parameters" are typically represented 
as <T>. In Rust, "generic" also describes anything that accepts one or more 
generic type parameters <T>. Any type specified as a generic type parameter 
is generic, and everything else is concrete (non-generic).

For example, defining a generic function named foo that takes an argument 
T of any type:

fn foo<T>(T) { ... }

Because T has been specified as a generic type parameter, it is considered 
generic when used here as (T). This is the case even if T has previously been 
defined as a struct.

This example shows some of the syntax in action:
*/

// A concrete type `A`.
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);
//            ^ Here is `Single`s first use of the type `A`.

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);

    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}
