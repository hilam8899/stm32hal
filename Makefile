
stm32builder ?= cargo run --package stm32builder --bin stm32builder --

# We need a part to build for.
# You can overwrite by running:
#
#    make PART=<part_number> [<command>]
#
PART ?= stm32f051K8T6
FEATURES = --features ${PART}
LIB_FILE = src/lib.rs

all: update check build test doc

# Update the Cargo.toml with devices features.
update:
	$(stm32builder) update-cargo Cargo.toml devices/*.yaml
check:
	cargo check ${FEATURES}
build:
	cargo build ${FEATURES}
doc:
	cargo doc ${FEATURES}
doc-open:
	cargo doc ${FEATURES} --open
# The test can't run on the host.
# But we at least compile check them.
test:
	cargo test --no-run ${FEATURES}
fmt:
	cargo fmt --check

# We don't want our generated appear on status but we need it to be checked in
# so assume it is unchanged.
config-git:
	git update-index --assume-unchanged $(LIB_FILE)

clean:
	# Remove generated files.
	@rm -f \
		src/lib.rs \
		src/rcc.rs \
		src/rcc/peripherals.rs \
		src/gpio.rs \
	# Remove the files that can results from experimenting.
	@rm -i -f \
		src/*.rs \
		src/rcc/*.rs \
	# Clean `lib.rs`.
	@echo "// This crate is generated at build time." > $(LIB_FILE)
