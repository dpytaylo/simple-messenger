use rpc::error::RpcError;
use tracing::error;

use crate::components::alert_message::{use_alert_message, MessageVariant};

const RPC_ERROR_MSG: &str =
    "An error occurred while processing the request. Please try again later.";

pub fn log_rpc_error(rpc_error: RpcError) {
    let alert = use_alert_message();

    match rpc_error {
        RpcError::Network(err) => {
            error!(error = %err, "RPC network error");
            alert.create(RPC_ERROR_MSG, MessageVariant::Failure, Default::default());
        }

        RpcError::ValidationError(err) => {
            error!(error = ?err, "RPC validation error");
            alert.create(RPC_ERROR_MSG, MessageVariant::Failure, Default::default());
        }

        RpcError::Deserialize | RpcError::NotFound | RpcError::Other => {
            error!(error = ?rpc_error, "RPC error");
            alert.create(RPC_ERROR_MSG, MessageVariant::Failure, Default::default());
        }
    }
}
