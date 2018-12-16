//! General Purpose Input and Output peripherals
//!
//! Supports:
//!   - Constrain a gpio peripheral
//!   - Access to individual pin
//!
//! Don't support (for now):
//!   - Configuring a pin into :
//!       - push-pull and open-drain output,
//!       - ploating, pull-up and pull-down input,
//!       - analog.
//!   - Changing the pins' speeds.
//!   - Freezing the pins' mode.
//!   - Configuring a pin into alternate function.
//!
//! ## Examples
//!
//! ### Pin lifetime
//! ```
//! # use stm32hal::{Constrain, ConstrainFrom};
//! // First, get the rcc and the pin's gpio, then get the pin from its gpio.
//! let mut rcc = stm32hal::rcc::RCC::take().unwrap();
//! let mut gpio = stm32hal::gpio::GPIOA::take_from(&mut rcc).unwrap();
//! let pin = gpio.pa1;
//!
//! // Now, do what you want with the pin.
//!
//! // When done, give the pin back to its gpio, then release the pin's gpio and the rcc.
//! gpio.pa1 = pin;
//! gpio.release_to(&mut rcc);
//! rcc.release();
//! ```
//!

pub use embedded_hal::digital::{
    toggleable::Default as ToggleablePin, InputPin, OutputPin, StatefulOutputPin,
};
use stm32ral::modify_reg;
use crate::api::ConstrainFrom;
use crate::rcc::RCC;
use core::marker::PhantomData;

{{~ #each banks }}

/// Proxy for {{GPIO}} peripheral.
pub struct {{GPIO}}Proxy(pub(crate) stm32ral::gpio::Instance);

{{~ #each pins }}

/// The {{../gpio}} pin {{n}}.
pub struct {{PIN}}<MODE> {
    mode: PhantomData<MODE>,
}

{{~ /each }}

/// The {{GPIO}} peripheral.
pub struct {{GPIO}} {
    /// {{GPIO}} proxy
    pub gpio: {{GPIO}}Proxy,
    {{~ #each pins }}
    /// Pin {{n}}
    pub {{pin}}: {{PIN}}<{{{initial_mode}}}>,
    {{~ /each }}
}

impl ConstrainFrom for {{GPIO}} {
    type Peripheral = RCC;

    /// Take the {{GPIO}} peripheral if not already taken.
    fn take_from(rcc: &mut RCC) -> Option<{{GPIO}}> {
        stm32ral::gpio::{{GPIO}}::take().map(|gpio| {
            rcc.enable_{{gpio}}();
            rcc.reset_{{gpio}}();
            Self {
                gpio: {{GPIO}}Proxy(gpio),
                {{~ #each pins }}
                {{pin}}: {{PIN}} { mode: PhantomData },
                {{~ /each }}
            }
        })
    }

    /// Release the {{GPIO}} peripheral.
    fn release_to(self, rcc: &mut RCC) {
        rcc.disable_{{gpio}}();
        stm32ral::gpio::GPIOA::release(self.gpio.0);
    }
}

{{~ /each }}

/// Pin input mode.
pub struct Input<MODE> {
    mode: PhantomData<MODE>,
}

/// Pin floating input.
pub struct Floating;
/// Pin pulled down input.
pub struct PullDown;
/// Pin pulled up input.
pub struct PullUp;

/// Pin output mode.
pub struct Output<MODE> {
    mode: PhantomData<MODE>,
}

/// Pin push pull output.
pub struct PushPull;
/// Pin open drain output.
pub struct OpenDrain;

/// Pin analog mode.
pub struct Analog;
