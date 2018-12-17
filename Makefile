
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

# Add src/lib.rs safety checks.
config-git:
	@rm .git/hooks/pre-commit
	@echo '#!/bin/sh' >> .git/hooks/pre-commit
	@echo '' >> .git/hooks/pre-commit
	@echo '# Ensure we do not commit modification to $(LIB_FILE)' >> .git/hooks/pre-commit
	@echo 'if git diff --cached --name-only -- $(LIB_FILE) | grep -q $(LIB_FILE)' >> .git/hooks/pre-commit
	@echo 'then' >> .git/hooks/pre-commit
	@echo '	 echo "You are going to commit changed to src/lib.rs."' >> .git/hooks/pre-commit
	@echo '	 echo "Please fixing it up or ignore me."' >> .git/hooks/pre-commit
	@echo '	 exit 1' >> .git/hooks/pre-commit
	@echo 'fi' >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit

# We don't want our generated appear on status but we need it to be checked in
# so assume it is unchanged.
ignore-lib-file:
	git update-index --assume-unchanged $(LIB_FILE)

clean:
	# Remove generated files.
	@rm -f \
		src/lib.rs \
		src/rcc.rs \
		src/rcc/peripherals.rs \
		src/gpio.rs \
		src/gpio/states.rs \
		src/gpio/modes.rs \
	# Remove the files that can results from experimenting.
	@rm -i -f \
		src/*.rs \
		src/rcc/*.rs \
		src/gpio/*.rs
	# Clean `lib.rs`.
	@echo "// This crate is generated at build time." > $(LIB_FILE)
release: clean
	@echo "Version to publish (without 'v' prefix):"
	@read version; \
		sed -i -r -e "s/^version = \"(.*)\"$$/version = \"$$version\"/" Cargo.toml ; \
		git add Cargo.toml ; \
		git commit --message "Release v$$version" --edit -v ; \
		git tag "v$$version"
publish:
		cargo publish --no-verify
		git push origin --tags master
