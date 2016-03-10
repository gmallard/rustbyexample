/*
Allows simple conversion from one type to another, however both types must have 
the same size and alignment:
*/

// transmute.rs
fn main() {
    let u: &[u8] = &[49, 50, 51];

    unsafe {
        assert!(u == std::mem::transmute::<&str, &[u8]>("123"));
    }
}

