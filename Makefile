fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

build:
	cargo build --locked --release --target wasm32-unknown-unknown

init_wallet:
	linera wallet init --faucet https://faucet.testnet-conway.linera.net

request_chain:
	linera wallet request-chain --faucet https://faucet.testnet-conway.linera.net

publish:
	linera publish-and-create \
  	target/wasm32-unknown-unknown/release/trusted_chain_{contract,service}.wasm
