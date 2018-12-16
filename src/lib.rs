//! stm32f051R8T6 hardware abstraction layer (HAL)
//!
//! This crate provide an hardware abstraction layer (HAL) for the stm32f051R8T6 micro-controller.
//! This micro-controller has 8Kb of RAM, 64Kb of flash on a LQFP64 package.
//! Read its [datasheet] for the complete device's characteristics.
//!
//! Read the [reference manual] for exhaustive descriptions of the device's features.
//!
//! [datasheet]: https://www.st.com/resource/en/datasheet/stm32f051R8.pdf
//! [reference manual]: https://www.st.com/resource/en/reference_manual/dm00031936.pdf

#![no_std]

// Device's peripherals
pub mod rcc;
pub mod gpio;

mod api;

// Re-export 
pub use crate::api::Constrain;
pub use crate::api::ConstrainFrom;
