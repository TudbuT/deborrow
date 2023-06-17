use std::mem;

fn main() {
    #[derive(Debug)]
    struct AStruct {
        a: u64,
        b: u64,
    }
    impl AStruct {
        fn test(&mut self) -> (&mut u64, &mut u64) {
            (&mut self.a, &mut self.b)
        }
    }
    let mut thing = AStruct { a: 0, b: 10 };
    let b = thing.test();
    *b.0 = 30;
    *b.1 = 15;
    let a = b.0;
    println!("{a}");
    mem::drop(thing);
}
