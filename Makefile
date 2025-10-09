fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

build:
	cargo build --locked --release --target wasm32-unknown-unknown

init_wallet:
	linera wallet init --faucet $(faucet)

init_wallet_local:
	make init_wallet faucet=http://localhost:8080

init_wallet_testnet:
	make init_wallet faucet=https://faucet.testnet-conway.linera.net

request_chain:
	linera wallet request-chain --faucet $(faucet)

request_chain_local:
	make request_chain faucet=http://localhost:8080

request_chain_testnet:
	make request_chain faucet=https://faucet.testnet-conway.linera.net

publish:
	linera publish-and-create \
  	target/wasm32-unknown-unknown/release/trusted_chain_{contract,service}.wasm

local_net:
	RUST_LOG="info,linera_execution::wasm=debug" linera net up --with-faucet --faucet-port 8080

local_service:
	linera service --port 7070
