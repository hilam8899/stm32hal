//! Build script for stm32hal crate

use std::{env, error::Error, fs::File};
use stm32builder::{Device, DeviceId};

fn main() -> Result<(), Box<dyn Error>> {
    // Collect all device features cargo has passed to us, excluding the feature
    // it add to express the "stm32ral" dependency, and keeping only the part number.
    let features: Vec<String> = env::vars()
        .filter(|(env, _)| env.starts_with("CARGO_FEATURE_STM32"))
        .filter(|(env, _)| env != "CARGO_FEATURE_STM32RAL")
        .map(|(env, _)| env.replace("CARGO_FEATURE_", "").to_lowercase())
        .collect();

    // Get the part number and catch common error.
    let id = match features.len() {
        1 => &features[0],
        0 => panic!("Please, provide one part number as feature"),
        _ => panic!("Please, provide only one part number as feature"),
    };

    // Parse the part number as a device identifier.
    let id = DeviceId::from_str(&id)?;

    // Lookup the device's device file. By convention, we have one yaml file per supported product
    // line.
    let device = format!("devices/{}.yaml", id.name());

    // To ensure we always generate this crate with fresh data, re-run this script in case the
    // device file as been changed on our behalf.
    println!("cargo:rerun-if-changed={}", device);

    // Extract the device's data matching the device's identifier on the device's file.
    let device = Device::from_id_and_file(&id, &File::open(device)?)?;

    // Get a rendering context to render the templates with.
    let context = stm32builder::Context::new();

    macro_rules! render {
        // Render the `data` (it doesn't support the `object.field` syntax, so we provide helpers
        // below) on a `template` file to an `output` file.
        ( $data:tt on $template:tt to $output:tt ) => {

            stm32builder::render(
                &$data,
                &mut File::open($template)?,
                &mut File::create($output)?,
                &context,
            )?;

            // To ensure we always build fresh code, re-run this script in case the template has
            // been changed. Don't do the same for the output file. Otherwise the build script will
            // be re-run and regenerate the output files even if nothing has been changed.
            println!("cargo:rerun-if-changed={}", $template);
        };
        // Render the device's informations on a `template` file to an `output` file.
        ( @info on $template:tt to $output:tt ) => {
            let info = &device.info;
            render!(info on $template to $output);
        };
        // Render the device's `name` peripheral if it exists on a `template` file to an `output`
        // file.
        ( @peripheral $name:tt on $template:tt to $output:tt ) => {
            if let Some(peripheral) = &device.peripherals.$name {
                render!(peripheral on $template to $output);
            }
        };
    }

    // Generate the crate from template files.
    render!(@peripheral rcc on "templates/rcc.rs" to "src/rcc.rs");
    render!(@peripheral rcc on "templates/rcc/peripherals.rs" to "src/rcc/peripherals.rs");
    render!(@peripheral gpio on "templates/gpio.rs" to "src/gpio.rs");
    render!(@peripheral gpio on "templates/gpio/modes.rs" to "src/gpio/modes.rs");
    render!(@peripheral gpio on "templates/gpio/states.rs" to "src/gpio/states.rs");
    render!(device on "templates/lib.rs" to "src/lib.rs");

    // Generate a device's memory linker script if we are asked to.
    // Done by default unless build with the cargo `--no-default-features` option or with the
    // `default-feature = false` directive in the user 'Cargo.toml'.
    if env::var("CARGO_FEATURE_MEMORY_SCRIPT").is_ok() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let memory_file = format!("{}/memory.x", out_dir);
        println!("cargo:rustc-link-search={}", out_dir);

        render!(@info on "templates/memory.x" to memory_file);
    }

    // Cargo re-run the build script (this file) in case it has been changed.
    // So no need to add `println!("cargo:rerun-if-changed=build.rs");` here.

    // We have done our jobs, let cargo handle the build.
    Ok(())
}
