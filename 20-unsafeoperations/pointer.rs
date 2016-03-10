/*
Raw pointers * and references &T function similarly, but references are always 
safe because they are guaranteed to point to valid data due to the borrow 
checker. Dereferencing a raw pointer can only be done through an unsafe block.
*/

// pointer.rs
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}

