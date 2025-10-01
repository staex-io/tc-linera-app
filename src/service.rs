#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::Arc;

use linera_sdk::{Service, ServiceRuntime, abi::WithServiceAbi, views::View};
use tc_linera_app::TrustedChainAbi;

use crate::state::TrustedChainState;

mod state;

linera_sdk::service!(TrustedChainService);

pub struct TrustedChainService {
    _state: TrustedChainState,
    _runtime: Arc<ServiceRuntime<Self>>,
}

impl WithServiceAbi for TrustedChainService {
    type Abi = TrustedChainAbi;
}

impl Service for TrustedChainService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TrustedChainState::load(runtime.root_view_storage_context())
            .await
            .unwrap();
        Self {
            _state: state,
            _runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, _query: Self::Query) -> Self::QueryResponse {
        todo!()
    }
}
