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
  	target/wasm32-unknown-unknown/release/trusted_chain_contract.wasm \
		target/wasm32-unknown-unknown/release/trusted_chain_service.wasm

local_net:
	RUST_LOG="info,linera_execution::wasm=debug" linera net up --with-faucet --faucet-port 8080

local_service:
	linera service --port 7070

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1

build_linera_image:
	docker build \
		-t swr.eu-de.otc.t-systems.com/staex/trustedchain/linera:v0.0.1 \
		-f Dockerfile \
		.
	docker push swr.eu-de.otc.t-systems.com/staex/trustedchain/linera:v0.0.1

run_linera_image:
	docker run --rm -it \
		-v "${HOME}/.config/linera/:/root/.config/linera/" \
		linera-service \
		wallet show
