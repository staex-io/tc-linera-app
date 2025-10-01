#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::{
    Contract, ContractRuntime,
    abi::WithContractAbi,
    views::{RootView, View},
};
use tc_linera_app::TrustedChainAbi;

use crate::state::TrustedChainState;

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

    async fn execute_operation(&mut self, _operation: Self::Operation) -> Self::Response {}

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.unwrap()
    }
}
