#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::{
    Contract, ContractRuntime,
    abi::WithContractAbi,
    views::{RootView, View},
};
use tc_linera_app::{TrustedChainAbi, TrustedChainOperation};

use crate::state::{TelemetryData, TrustedChainState};

mod state;

linera_sdk::contract!(TrustedChainContract);

pub struct TrustedChainContract {
    state: TrustedChainState,
    runtime: ContractRuntime<Self>,
}

impl WithContractAbi for TrustedChainContract {
    type Abi = TrustedChainAbi;
}

impl Contract for TrustedChainContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TrustedChainState::load(runtime.root_view_storage_context())
            .await
            .unwrap();
        Self { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: TrustedChainOperation) -> Self::Response {
        log::info!("execute operation: {:?}", operation);
        match operation {
            TrustedChainOperation::Land {
                id,
                hash,
                signature,
            } => {
                self.state
                    .telemetry_data
                    .insert(&id, TelemetryData { hash, signature })
                    .unwrap();
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.unwrap()
    }
}
