use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A reference which can be shared between threads and converted back
/// into normal refs anywhere.
pub struct Reference<'a, T: ?Sized, const IS_MUT: bool> {
    ptr: *const T,
    phantom_t: PhantomData<&'a T>,
}

unsafe impl<'a, T: ?Sized, const IS_MUT: bool> Send for Reference<'a, T, IS_MUT> {}
unsafe impl<'a, T: ?Sized, const IS_MUT: bool> Sync for Reference<'a, T, IS_MUT> {}

impl<'a, T: ?Sized> Reference<'a, T, false> {
    /// Constructs a Reference from a non-mutable reference
    pub fn from_ref(r: &'a T) -> Self {
        Self {
            ptr: r as *const _,
            phantom_t: PhantomData,
        }
    }
}

impl<'a, T: ?Sized> Reference<'a, T, true> {
    /// Constructs a Reference from a mutable reference
    pub fn from_mut(r: &'a mut T) -> Self {
        Self {
            ptr: r as *const T,
            phantom_t: PhantomData,
        }
    }

    /// Turns the Reference into a mutable ref if it has been created
    /// from a mutable ref.
    pub unsafe fn as_mut(&self) -> Option<&'a mut T> {
        Some(self.ptr.cast_mut().as_mut().unwrap()) // never null
    }
}

impl<'a, T: ?Sized, const IS_MUT: bool> Reference<'a, T, IS_MUT> {
    /// Turns the Reference into a non-mutable ref.
    ///
    /// # SAFETY:
    /// This function is only safe until the reference has been
    /// "polluted" by a call to any as_mut.
    pub unsafe fn as_ref(&self) -> &'a T {
        unsafe { self.ptr.as_ref().unwrap() } // never null
    }

    /// Turns the Reference into a mutable ref even if it has been created
    /// from a non-mutable ref.
    pub unsafe fn mutable(&self) -> Reference<'a, T, true> {
        Reference {
            ptr: self.ptr,
            phantom_t: self.phantom_t,
        }
    }

    /// Deborrows the Reference, returning a new one with a
    /// disconnected lifetime.
    pub unsafe fn disconnect<'b>(&self) -> Reference<'b, T, IS_MUT> {
        Reference {
            phantom_t: PhantomData,
            ..*self
        }
    }
}

pub trait AsReference<T: ?Sized> {
    /// Turns this non-mutable ref into a Reference type
    fn as_reference<'a>(&'a self) -> Reference<'a, T, false>;
    /// Turns this mutable ref into a Reference type
    fn as_mut_reference<'a>(&'a mut self) -> Reference<'a, T, true>;
    /// Turns this non-mutable ref into a Reference type with a disconnected
    /// lifetime.
    unsafe fn as_deborrowed_reference<'a, 'b>(&'a self) -> Reference<'b, T, false>;
    /// Turns this mutable ref into a Reference type with a disconnected
    /// lifetime.
    unsafe fn as_deborrowed_mut_reference<'a, 'b>(&'a mut self) -> Reference<'b, T, true>;
}

impl<T: ?Sized> AsReference<T> for T {
    fn as_reference<'a>(&'a self) -> Reference<'a, T, false> {
        Reference::from_ref(self)
    }

    fn as_mut_reference<'a>(&'a mut self) -> Reference<'a, T, true> {
        Reference::from_mut(self)
    }

    unsafe fn as_deborrowed_reference<'a, 'b>(&'a self) -> Reference<'b, T, false> {
        self.as_reference().disconnect()
    }

    unsafe fn as_deborrowed_mut_reference<'a, 'b>(&'a mut self) -> Reference<'b, T, true> {
        self.as_mut_reference().disconnect()
    }
}
