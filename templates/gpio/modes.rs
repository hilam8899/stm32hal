//! Pins modes API

{{!~
// The gpio version 1 (found on the F1 family) has different mode settings registries than the
// version 2 (found on all other families).
~}}{{ #if (not (or (eq version 1) (eq version 2))) ~}}
compile_error!("Only gpio version 1 and 2 are supported");
{{~ /if ~}}

use super::*;
use core::marker::PhantomData;

{{~ #each banks }}
{{~ #each pins }}

impl<MODE> {{PIN}}<MODE> {
    /// Configure the {{../gpio}} pin {{n}} to operate into its default mode.
    pub fn into_default(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<{{{initial_mode}}}> {
        {{ #if (str_eq initial_mode "Input<Floating>") ~}}
        self.into_floating_input(gpio)
        {{~ else }}{{ #if (str_eq initial_mode "Input<PullUp>") ~}}
        self.into_pull_up_input(gpio)
        {{~ else }}{{ #if (str_eq initial_mode "Input<PullDown>") ~}}
        self.into_pull_down_input(gpio)
        {{~ else ~}}
        compile_error!("{{{initial_mode}}} not implemented for into_default()")
        {{~ /if ~}}{{~ /if ~}}{{~ /if }}
    }

    /// Configure the {{../gpio}} pin {{n}} to operate into floating input mode.
    pub fn into_floating_input(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<Input<Floating>> {
        {{ #if (eq ../../version 1) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, {{ (low_or_high "CR" n) }}, CR{{n}}: 0b01_00);
        {{ /if ~}}
        {{ #if (eq ../../version 2) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, MODER, MODER{{n}}: Input);
        modify_reg!(stm32ral::gpio, gpio.0, PUPDR, PUPDR{{n}}: Floating);
        {{ /if ~}}
        {{PIN}} { mode: PhantomData }
    }

    /// Configure the {{../gpio}} pin {{n}} to operate into pull-up input mode.
    pub fn into_pull_up_input(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<Input<PullUp>> {
        {{ #if (eq ../../version 1) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, {{ (low_or_high "CR" n) }}, CR{{n}}: 0b10_00);
        modify_reg!(stm32ral::gpio, gpio.0, BSRR, BS{{n}}: Set);
        {{ /if ~}}
        {{ #if (eq ../../version 2) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, MODER, MODER{{n}}: Input);
        modify_reg!(stm32ral::gpio, gpio.0, PUPDR, PUPDR{{n}}: PullUp);
        {{ /if ~}}
        {{PIN}} { mode: PhantomData }
    }

    /// Configure the {{../gpio}} pin {{n}} to operate into pull-up input mode.
    pub fn into_pull_down_input(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<Input<PullDown>> {
        {{ #if (eq ../../version 1) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, {{ (low_or_high "CR" n) }}, CR{{n}}: 0b10_00);
        modify_reg!(stm32ral::gpio, gpio.0, BSRR, BR{{n}}: Reset);
        {{ /if ~}}
        {{ #if (eq ../../version 2) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, MODER, MODER{{n}}: Input);
        modify_reg!(stm32ral::gpio, gpio.0, PUPDR, PUPDR{{n}}: PullDown);
        {{ /if ~}}
        {{PIN}} { mode: PhantomData }
    }

    /// Configure the {{../gpio}} pin {{n}} to operate into push-pull output mode.
    pub fn into_push_pull_output(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<Output<PushPull>> {
        {{ #if (eq ../../version 1) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, {{ (low_or_high "CR" n) }}, CR{{n}}: 0b00_11); // max speed 50Mhz
        {{ /if ~}}
        {{ #if (eq ../../version 2) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, MODER, MODER{{n}}: Output);
        modify_reg!(stm32ral::gpio, gpio.0, OTYPER, OT{{n}}: PushPull);
        {{ /if ~}}
        {{PIN}} { mode: PhantomData }
    }

    /// Configure the {{../gpio}} pin {{n}} to operate into open-drain output mode.
    pub fn into_open_drain_output(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<Output<OpenDrain>> {
        {{ #if (eq ../../version 1) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, {{ (low_or_high "CR" n) }}, CR{{n}}: 0b01_11); // max speed 50Mhz
        {{ /if ~}}
        {{ #if (eq ../../version 2) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, MODER, MODER{{n}}: Output);
        modify_reg!(stm32ral::gpio, gpio.0, OTYPER, OT{{n}}: OpenDrain);
        {{ /if ~}}
        {{PIN}} { mode: PhantomData }
    }

    /// Configure the {{../gpio}} pin {{n}} to operate into analogue input mode.
    pub fn into_analog(self, gpio: &mut {{../GPIO}}Proxy) -> {{PIN}}<Analog> {
        {{ #if (eq ../../version 1) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, {{ (low_or_high "CR" n) }}, CR{{n}}: 0b00_00);
        {{ /if ~}}
        {{ #if (eq ../../version 2) ~}}
        modify_reg!(stm32ral::gpio, gpio.0, MODER, MODER{{n}}: Analog);
        {{ /if ~}}
        {{PIN}} { mode: PhantomData }
    }
}

{{~ /each }}
{{ /each ~}}
