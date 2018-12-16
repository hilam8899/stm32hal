//! Public API

/// Constrain a peripheral.
pub trait Constrain: Sized {
    /// Take the peripheral if it is not already taken.
    fn take() -> Option<Self>;

    /// Release the peripheral.
    fn release(self);
}

/// Constrain a peripheral that depends on another peripheral.
pub trait ConstrainFrom: Sized {
    /// The peripheral this peripheral depends on.
    type Peripheral;

    /// Take a peripheral from a master peripheral if it is not already taken.
    fn take_from(from: &mut Self::Peripheral) -> Option<Self>;

    /// Release the peripheral to a master peripheral.
    fn release_to(self, to: &mut Self::Peripheral);
}
