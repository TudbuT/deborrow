# deborrow

Splits mutable references safely into their fields, and helps with reference manipulation.

## What does it do?

deborrow!() - split mutable references into their fields.
```rs
let fields = deborrow!(object, field1 field2 field3);
*fields.0 = 5;
*fields.1 = ":D";
*fields.2 = Vec::new();
```

Reference - an unsafe, sharable reference manipulation type.
```rs
// a crude example. please see examples/reference.rs for one without race conditions.
fn main() {
  let mut done: u32 = 0;
  for i in 0..5 {
    unsafe {
      let done_ref = done.as_deborrowed_mut_reference();
      thread::spawn(move || {
        // do things
        *done_ref.as_mut() += 1;
        println!("Thread {i} finished ({} done).", done_ref.as_ref());
      });
    }
    println!("{} things done so far.", done);
  }
  thread::sleep(Duration::from_millis(50));
  println!("{} things done.", done);
}
```

## Why is deborrow!() safe?

This does NOT allow any lifetimes to be extended beyond the object's main lifetime.
All lifetimes STAY bound. With these two conditions, even the normal rust compiler allows
this operation: A struct that has a method which takes in &mut self can return
(&mut A, &mut B, &mut C), where that is a tuple of the fields it contains, referenced 
mutably all at the same time. For a demo, look at the examples. bad.rs will not compile
and demonstrates the memory safety guarantee, good.rs demonstrates the macro as it is meant
to be used, and normal.rs shows an alternative to good.rs not using the macro at all.

## What is the Reference type for?

As explained in the example, it is meant for independent threads writing and reading to
the same buffer in applications where the reader doesn't need perfect accuracy, e.g. in
rendering a progressbar or even the output of a simulation (for whcih some synchronization 
code would still be required to avoid having a cut mid-frame). It is much easier to have
a proxy that does the pointer manipulation in a more readable, high-level, and verbose way.

It can also do other things (e.g. convert a non-mut reference into a mut one and disconnect
lifetimes).
