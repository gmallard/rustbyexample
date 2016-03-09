/*
When working with generics, the type parameters (e.g. Ty) may use traits 
(e.g. Tr) as bounds (e.g. Ty: Tr, which reads as: Ty must implement the 
Tr trait). Bounding has two effects:

1) Generics instances (let ty: Ty = (...)) can now access the methods 
(ty.tr()) of the traits specified in the bounds.
2) The generic can only be specialized for type parameters that conform to the 
bounds.

Bounds are typically applied in one of two ways:

1) At the first instance of the type
2) In a where clause which directly precedes the { in the impl
*/

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
struct Vec2<T> {
    x: T,
    y: T,
}

// Apply bound to `T` at first instance of `T`. `T`
// must implement the `Add` trait.
impl<T: Add<T, Output = T>> Add<Vec2<T>>
        for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            // `x` and `y` are of type `T`, and implement the `add` method
            x: self.x.add(rhs.x),
            // The sugary `+` operator can also be used
            y: self.y + rhs.y,
        }
    }
}

// Bound: `T` must implement the `Sub` trait
impl<T> Sub<Vec2<T>> for Vec2<T>
        where T: Sub<T, Output = T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Bound: `T` must implement *both* the `Add` trait and the `Mul` trait
impl<T> Vec2<T>
        where T: Add<T, Output = T> + Mul<T, Output = T> {
    fn dot(self, rhs: Vec2<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

fn main() {
    // Floats implement the `Add`, `Mul` and `Sub` traits
    let v1 = Vec2 { x: 1.2_f32, y: 3.4 };
    let v2 = Vec2 { x: 5.6_f32, y: 7.8 };

    println!("{:?} + {:?} = {:?}", v1, v2, v1 + v2);
    println!("{:?} - {:?} = {:?}", v1, v2, v1 - v2);
    println!("{:?} ⋅ {:?} = {:?}", v1, v2, v1.dot(v2));

    // Error! `char` doesn't implement the `Add` trait
    // println!("{:?}", Vec2 { x: ' ', y: 'b' } + Vec2 { x: 'c', y: 'd' });
    // FIXME ^ Comment out this line
}


