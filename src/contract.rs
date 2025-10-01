use linera_sdk::{Contract, ContractRuntime, abi::WithContractAbi};

use crate::{TrustedChainAbi, state::TrustedChainState};

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
