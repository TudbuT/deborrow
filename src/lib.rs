#![allow(clippy::needless_lifetimes)]
pub use deborrow_macro::*;
use std::mem;

/// Make the compiler forget about a borrow:
/// A slightly safer variant to transmuting plainly.
///
/// # Safety
/// Safe if and only if you have manually checked the lifetimes and are 100% sure
/// the borrow checker is wrong.
pub unsafe fn deborrow_mut<'a, 'b, T: ?Sized>(r: &'a mut T) -> &'b mut T {
    mem::transmute(r)
}

/// Make the compiler forget about a borrow:
/// A slightly safer variant to transmuting plainly.
///
/// # Safety
/// Safe if and only if you have manually checked the lifetimes and are 100% sure
/// the borrow checker is wrong.
pub unsafe fn deborrow<'a, 'b, T: ?Sized>(r: &'a T) -> &'b T {
    mem::transmute(r)
}
