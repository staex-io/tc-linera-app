#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::{Contract, ContractRuntime, abi::WithContractAbi};
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
        todo!()
    }

    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        todo!()
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        todo!()
    }

    async fn execute_message(&mut self, message: Self::Message) {
        todo!()
    }

    async fn store(self) {
        todo!()
    }
}
