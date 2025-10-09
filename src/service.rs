#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{Service, ServiceRuntime, abi::WithServiceAbi, views::View};
use tc_linera_app::{TrustedChainAbi, TrustedChainOperation};

use crate::state::{TelemetryData, TrustedChainState};

mod state;

linera_sdk::service!(TrustedChainService);

pub struct TrustedChainService {
    state: Arc<TrustedChainState>,
    runtime: Arc<ServiceRuntime<Self>>,
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
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            MutationRoot {
                runtime: self.runtime.clone(),
            },
            EmptySubscription,
        )
        .finish();
        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<TrustedChainState>,
}

#[Object]
impl QueryRoot {
    async fn value(&self, id: String) -> Option<TelemetryData> {
        log::info!(
            "contains?: {:?}",
            self.state.telemetry_data.contains_key(&id).await.unwrap()
        );

        self.state.telemetry_data.get(&id).await.unwrap()
    }
}

struct MutationRoot {
    runtime: Arc<ServiceRuntime<TrustedChainService>>,
}

#[Object]
impl MutationRoot {
    async fn land(&self, id: String, hash: String, signature: String) -> [u8; 0] {
        self.runtime
            .schedule_operation(&TrustedChainOperation::Land {
                id,
                hash,
                signature,
            });
        []
    }
}
