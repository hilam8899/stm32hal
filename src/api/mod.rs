//! Public API

/// Constrain a peripheral.
pub trait Constrain: Sized {
    /// Take the peripheral if it is not already taken.
    fn take() -> Option<Self>;

    /// Release the peripheral.
    fn release(self);
}
