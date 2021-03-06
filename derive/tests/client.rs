use futures::prelude::*;
use jsonrpc_core::{IoHandler, Result};
use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
	#[rpc(name = "add")]
	fn add(&self, a: u64, b: u64) -> Result<u64>;
}

struct RpcServer;

impl Rpc for RpcServer {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}

#[test]
fn client_server_roundtrip() {
	let mut handler = IoHandler::new();
	handler.extend_with(RpcServer.to_delegate());
	let (client, rpc_client) = local::connect::<gen_client::Client, _, _>(handler);
	let fut = client
		.clone()
		.add(3, 4)
		.and_then(move |res| client.add(res, 5))
		.join(rpc_client)
		.map(|(res, ())| {
			assert_eq!(res, 12);
		})
		.map_err(|err| {
			eprintln!("{:?}", err);
			assert!(false);
		});
	tokio::run(fut);
}
