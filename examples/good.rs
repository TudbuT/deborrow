use deborrow::*;
use std::mem;

fn main() {
    #[derive(Debug)]
    struct AStruct {
        a: u64,
        b: u64,
    }
    // spacer so you can swich back n forth between normal.rs and good.rs
    // spacer to see the differences.
    // spacer
    // spacer
    let mut thing = AStruct { a: 0, b: 10 };
    let b = deborrow!(thing, a b);
    *b.0 = 30;
    *b.1 = 15;
    let a = b.0;
    println!("{a}");
    mem::drop(thing);
}
