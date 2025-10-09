use async_graphql::{Request, Response};
use linera_sdk::abi::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TrustedChainOperation {
    Land {
        id: String,        // telemetry id
        hash: String,      // telemetry hash base64
        signature: String, // telemetry signature base64
    },
}

pub struct TrustedChainAbi;

impl ContractAbi for TrustedChainAbi {
    type Operation = TrustedChainOperation;
    type Response = ();
}

impl ServiceAbi for TrustedChainAbi {
    type Query = Request;
    type QueryResponse = Response;
}
