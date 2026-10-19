# Run formatting check, strict clippy, and tests in one command
check:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-targets

# Auto-fix formatting and clippy issues
fix:
    cargo fmt --all
    cargo clippy --fix --allow-dirty --allow-staged

# build and symlink
dev: clean
	@cargo build -q
	@ln -s ./target/debug/vortx ./vortx
	@echo "Build done"

# build release build
build: clean
	cargo build -r
	@ln -s ./target/release/vortx ./vortx

clean:
	cargo clean
	@rm -f ./vortx
