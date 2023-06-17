# deborrow

Splits mutable references safely into their fields, or slightly less 
safely unborrows something completely.

## What does it do?

```rs
let fields = deborrow!(object, field1 field2 field3);
*fields.0 = 5;
*fields.1 = ":D";
*fields.2 = Vec::new();
```

## Why is it safe?

This does NOT allow any lifetimes to be extended beyond the object's main lifetime.
All lifetimes STAY bound. With these two conditions, even the normal rust compiler allows
this operation: A struct that has a method which takes in &mut self can return
(&mut A, &mut B, &mut C), where that is a tuple of the fields it contains, referenced 
mutably all at the same time. For a demo, look at the examples. bad.rs will not compile
and demonstrates the memory safety guarantee, good.rs demonstrates the macro as it is meant
to be used, and normal.rs shows an alternative to good.rs not using the macro at all.

## And there is a little extra

deborrow() as a function exists as well, although unsafe. It can be used to extend a
lifetime to whatever you want but with extra safety of knowing nothing else will change, 
but will not have any guarantees about the lifetime, which is why it's marked unsafe. It
effectively unborrows something.
