/*
We determined a snake is an inappropriate gift for a princess. What if she 
expected a gift but did not receive one? Clearly that would be just as bad but 
how would it be handled? Well, Option<T> is the type which is used when absense 
is a possibility. This manifests itself as two choices:

1) Some<T>: An element T was found
2) None: No element was found

These can either be explicitly handled via match or implicitly with unwrap. 
unwrap, deferring to the std library, either returns the inner element or 
panics. Regardless of explicit or implicit handling, an enum such as Option 
will have all cases handled. The compiler ensures that none are forgotten giving 
us more confidence in its robustness.
*/

// The commoner cannot bring down the task which precludes the option of `panic`.
// These must all be handled manually. `match` would be the correct approach.
fn give_commoner(gift: Option<&str>) {
    // Specify a specific course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! Throws the snake in the fire."),
        Some(inner)   => println!("{}! How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// A princess is allowed to bring down the task at will so `panic` is an option.
fn give_princess(gift: Option<&str>) {
    // Using `unwrap` defers the case analysis to the std library which will
    // `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}

/*
As you can see, direct control yielded an even nicer result than the original 
panic along with the choice to panic if we desired. unwrap on the other hand, 
deferring to the std library left us with the most generic and unhelpful: 
"I unwrapped a None!" meanwhile yielding the good results the rest of the time. 
A more meaningful message will require a better approach.
*/

