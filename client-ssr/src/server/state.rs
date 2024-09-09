use std::sync::Arc;

use axum::extract::FromRef;
use leptos::LeptosOptions;
use rpc::client::RpcClient;

use crate::server::Environment;

#[derive(Clone, FromRef)]
pub struct ServerStateWrapper {
    pub inner: Arc<ServerState>,
}

pub struct ServerState {
    pub leptos_options: LeptosOptions,
}

impl ServerStateWrapper {
    pub fn new(env: &Environment, leptos_options: LeptosOptions) -> Self {
        Self {
            inner: Arc::new(ServerState { leptos_options }),
        }
    }
}

impl axum::extract::FromRef<ServerStateWrapper> for LeptosOptions {
    fn from_ref(this: &ServerStateWrapper) -> LeptosOptions {
        this.inner.leptos_options.clone()
    }
}
