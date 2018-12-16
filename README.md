stm32hal 
========

This project provides a Rust HAL (hardware abstraction layer) for all STM32 micro-controllers.

<!-- TODO Add crates.io, doc.rs, travis-ci.org, license badges -->
<!-- TODO Add Documentation, Repositories, Supported Devices, Example Project links -->

## What it is?

This crate is an experiment into providing a hardware abstraction layer supporting all the devices
and peripherals found across the stm32 micro-controller families.

It use the [stm32ral](https://github.com/adamgreig/stm32ral) (register access layer) to configure
hardware.

Please consider trying it out and contributing or leaving feedback!

## What it provides?

  - Support for multiple stm32 devices.
  - A library tailored to the stm32 device you want to use.
  - Exposure of the peripherals and functionalities a given stm32 device support.
  - High-level hardware description.
  - Support for the RCC and GPIO peripherals out of the box.

## Using it in your own crates

In your `Cargo.toml`, depends on exact device you want to use.  Replace `stm32f051R8T6` with the
device chip name.  See [Supported Devices][] for the full list.

```toml
[dependencies.stm32hal]
version = "0.1.0"
features = ["stm32f051R8T6"]
```

At the top level of the `stm32hal` crate, there is a module for each peripheral
type, such as `stm32hal::gpio`.
Inside each peripheral module there is a struct for each supported peripheral,
such as `stm32hal::gpio::GPIOC`.

As this crate is generated at build time, access the documentation with:
```
cargo doc --open
```

## Quick Example

```rust
// First you must `take()` the peripheral instance.
// This returns `Some(hal)` if that instance is not already taken; otherwise it returns `None`.
// This ensure that no other code can be simultaneously accessing the peripheral,
// which could lead to race condition.
let mut rcc = stm32hal::rcc::RCC::take().unwrap();

// Some peripheral depends on other peripheral before they can be used.
// Use the `take_from()` to get the peripheral instance.
// Here the gpio clock line need to be enabled by the RCC peripheral.
let gpioa = stm32hal::gpio::GPIOA::take_from(&mut rcc).unwrap();

// Get a pin.
let pin = gpioa.pa1;

// ... Do something with gpio pins...

// Give the pin back to its gpio.
gpioa.pa1 = pin;

// When done with a peripheral that depends on other, use `release_to()` to release it.
gpioa.release_to(&mut rcc);

// When done with peripheral `release()` it.
rcc.release();
```

See the documentation for more examples.

## Supporting a new device

The device are described in a `<device>.yaml` file inside the `devices/` folder. A device file can
support multiple device part.
