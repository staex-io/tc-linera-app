use async_graphql::{Request, Response};
use linera_sdk::abi::{ContractAbi, ServiceAbi};

use crate::state::TelemetryData;

mod contract;
mod service;
mod state;

pub struct TrustedChainAbi;

impl ContractAbi for TrustedChainAbi {
    type Operation = TelemetryData;
    type Response = TelemetryData;
}

impl ServiceAbi for TrustedChainAbi {
    type Query = Request;
    type QueryResponse = Response;
}
