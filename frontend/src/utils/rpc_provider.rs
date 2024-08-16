use leptos::{expect_context, provide_context};
use rpc::client::RpcClient;

#[derive(Clone)]
struct RpcClientProvider(RpcClient);

pub fn provide_rpc_client(url: &'static str) {
    provide_context(RpcClientProvider(RpcClient::new(url).unwrap()));
}

pub fn use_rpc_client() -> RpcClient {
    expect_context::<RpcClientProvider>().0
}
