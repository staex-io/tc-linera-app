#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::Arc;

use linera_sdk::{Service, ServiceRuntime, abi::WithServiceAbi};
use tc_linera_app::TrustedChainAbi;

use crate::state::TrustedChainState;

mod state;

linera_sdk::service!(TrustedChainService);

pub struct TrustedChainService {
    state: TrustedChainState,
    runtime: Arc<ServiceRuntime<Self>>,
}

impl WithServiceAbi for TrustedChainService {
    type Abi = TrustedChainAbi;
}

impl Service for TrustedChainService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        todo!()
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        todo!()
    }
}
