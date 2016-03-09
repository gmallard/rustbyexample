/*
map() was previously described as a way to simplify a match which also allows 
chaining. However, map() does not work for all cases because the constituents 
often occur in many different combinations. Consider the following example:
*/

#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// We know how to make everything except Cordon Bleu.
fn can_cook(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// To make a meal, we require both the ingredients and the ability to make that
// meal, which is only possible when both are true; thus successes chain.
// Conveniently, this can be rewritten more compactly with `and_then()`.
fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None       => None,
        Some(food) => match can_cook(food) {
            None       => None,
            Some(food) => Some(food),
        },
    }
}

// Same as `v1` above but uses `and_then()` instead.
fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(can_cook)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}

/*
The reason this worked is because and_then() happened to require the exact 
function type as an input that was needed here. map() did not. Comparing the 
signatures of their input types, you will see that when the function returned 
an Option, and_then() became the one and only valid choice.

1) map():      FnOnce(T) -> U
2) and_then(): FnOnce(T) -> Option<U>

These are just two of many different combinators that are implemented on Option 
by the std library for many different use cases. It is advantageous to become 
familiar with them because they can simplify many error handling procedures and 
avoid the ugly and suicidal panic!() alternative. The other common error 
handling type, Result, also uses most of these same constructs so the skills 
are transferable.
*/

