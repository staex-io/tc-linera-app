use linera_sdk::views::{MapView, RootView, ViewStorageContext, linera_views};
use serde::{Deserialize, Serialize};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct TrustedChainState {
    pub telemetry_data: MapView<String, TelemetryData>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, async_graphql:: SimpleObject)]
pub struct TelemetryData {
    pub hash: String,
    pub signature: String,
}
