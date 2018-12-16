//! Reset and Clock Control peripheral
//!
//! Supports:
//!   - Constraining the RCC peripheral.
//!
//! Not supported (yet):
//!   - Configuring the clocks tree.
//!
//! ## Examples
//!
//! ### RCC lifetime
//! ```
//! # use stm32hal::Constrain;
//! // First, take the RCC peripheral.
//! let rcc = stm32hal::rcc::RCC::take().unwrap();
//!
//! // Now, do what you want with the rcc.
//!
//! // When done, release it.
//! rcc.release();
//! ```

use crate::api::Constrain;

// Include the RCC peripherals API.
mod peripherals;

/// The RCC peripheral
#[allow(non_camel_case_types)]
pub struct RCC {
    rcc: stm32ral::rcc::Instance,
}

impl Constrain for RCC {
    /// Take the RCC peripheral if not already taken.
    fn take() -> Option<RCC> {
        stm32ral::rcc::RCC::take().map(|rcc| RCC { rcc })
    }

    /// Release the RCC peripheral.
    fn release(self) {
        stm32ral::rcc::RCC::release(self.rcc)
    }
}
