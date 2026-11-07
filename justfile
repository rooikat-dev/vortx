log_level := "vortx=info"
export RUST_LOG := env_var_or_default("RUST_LOG", log_level)

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
dev:
	@cargo build -q
	@ln -sf ./target/debug/vortx ./vortx

send file="./testfile": dev
	./vortx send {{file}}

receive phrase: dev
	./vortx receive {{phrase}}

build: clean
	cargo build -r
	@ln -sf ./target/release/vortx ./vortx

clean:
	cargo clean
	rm -f ./vortx
