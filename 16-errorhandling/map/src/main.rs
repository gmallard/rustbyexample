/*
Like before, match is a valid method for handling an Option however you may find 
that this gets tedious with heavy usage of Options. Luckily, there is an easier 
way.

Consider the example below and in particular peel() and chop(); since each of 
these operations is only valid when the input exists, it seems sensible that bad 
input simply means the result is bad. This results in the simplistic mapping 
Some -> Some and None -> None. This is actually so common that there is a built 
in method for it called map() which is already implemented on Option.

The result of this is that chop() is simpler to write than any of the previous 
methods. Furthermore, the ability to chain these together makes it even more 
flexible; process() easily can replace all the previous functions and still be 
compact.
*/

#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Peeling food. If there isn't any, then just return `None`.
// Otherwise, return the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// Similarly, we still need to keep track of whether there is a problem. If there
// is, we just pass it on.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// Same as before, when `Some`, pass `food` to `Cooked`, otherwise return `None`.
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// You could even simplify the process further
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Can we eat it now?
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

