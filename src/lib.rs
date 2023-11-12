#![allow(clippy::needless_lifetimes)]
pub use deborrow_macro::*;
use std::mem;

mod reference;
pub use reference::*;

/// Make the compiler forget about a borrow:
/// A slightly safer variant to transmuting plainly.
///
/// # SAFETY:
/// Safe if and only if you have manually checked the lifetimes and are 100% sure
/// the borrow checker is wrong.
pub unsafe fn deborrow_mut<'a, 'b, T: ?Sized>(r: &'a mut T) -> &'b mut T {
    mem::transmute(r)
}

/// Make the compiler forget about a borrow:
/// A slightly safer variant to transmuting plainly.
///
/// # SAFETY:
/// Safe if and only if you have manually checked the lifetimes and are 100% sure
/// the borrow checker is wrong.
pub unsafe fn deborrow<'a, 'b, T: ?Sized>(r: &'a T) -> &'b T {
    mem::transmute(r)
}

/// Transforms a reference into a mutable reference of the same lifetime
///
/// # SAFETY:
/// Not safe unless you know EXACTLY what you're doing. Can be used to share
/// things between threads mutably.
pub unsafe fn ref_to_mut<'a, T: ?Sized>(r: &'a T) -> &'a mut T {
    #[allow(mutable_transmutes)]
    mem::transmute(r)
}
