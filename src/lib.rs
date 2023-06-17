pub use deborrow_macro::*;
use std::mem;

/// Make the compiler forget about a borrow:
/// A slightly safer variant to transmuting plainly.
pub unsafe fn deborrow<'a, 'b, T>(r: &'a mut T) -> &'b mut T {
    mem::transmute(r)
}
