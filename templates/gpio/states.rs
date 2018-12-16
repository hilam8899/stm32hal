//! Pins states API

use stm32ral::{read_reg, write_reg};
use super::*;

{{~ #each banks }}
{{~ #each pins }}

impl<MODE> InputPin for {{PIN}}<Input<MODE>> {
    /// Is the {{../gpio}} pin {{n}} low?
    fn is_low(&self) -> bool {
        unsafe { read_reg!(stm32ral::gpio, {{../GPIO}}, IDR, IDR{{n}} == Low) }
    }
    /// Is the {{../gpio}} pin {{n}} high?
    fn is_high(&self) -> bool {
        unsafe { read_reg!(stm32ral::gpio, {{../GPIO}}, IDR, IDR{{n}} == High) }
    }
}

impl<MODE> StatefulOutputPin for {{PIN}}<Output<MODE>> {
    /// Is the {{../GPIO}} pin {{n}} set low?
    fn is_set_low(&self) -> bool {
        unsafe { read_reg!(stm32ral::gpio, {{../GPIO}}, ODR, ODR{{n}} == Low) }
    }
    /// Is the {{../GPIO}} pin {{n}} set high?
    fn is_set_high(&self) -> bool {
        unsafe { read_reg!(stm32ral::gpio, {{../GPIO}}, ODR, ODR{{n}} == High) }
    }
}

impl<MODE> OutputPin for {{PIN}}<Output<MODE>> {
    /// Set the {{../gpio}} pin {{n}} to high output
    fn set_high(&mut self) {
        unsafe { write_reg!(stm32ral::gpio, {{../GPIO}}, BSRR, BS{{n}}: Set) }
    }
    /// Set the {{../gpio}} pin {{n}} to low output
    fn set_low(&mut self) {
        unsafe { write_reg!(stm32ral::gpio, {{../GPIO}}, BSRR, BR{{n}}: Reset) }
    }
}

impl<MODE> ToggleablePin for {{PIN}}<Output<MODE>> {}

{{~ /each }}
{{ /each ~}}
