use deborrow::*;
use std::mem;

fn main() {
    println!("This won't compile anyway.");
}

// need to exclude from tests since it wont compile
#[test]
#[ignore]
fn cant_compile() {
    #[derive(Debug)]
    struct AStruct {
        a: u64,
        b: u64,
    }
    let mut thing = AStruct { a: 0, b: 10 };
    let b = deborrow!(thing, a b);
    *b.0 = 30;
    *b.1 = 15;
    mem::drop(b.1);
    let a = b.0;
    mem::drop(thing);
    println!("{a}");
}
