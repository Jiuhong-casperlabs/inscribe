PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

.PHONY:	build-contract
build-contract:
	cargo build --release --target wasm32-unknown-unknown -p cep18
	wasm-strip target/wasm32-unknown-unknown/release/cep18.wasm


clippy:
	cd cep18 && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd cep18 && cargo fmt -- --check

lint: clippy
	cd cep18 && cargo fmt

clean:
	cd cep18 && cargo clean
