use namada::proto::Tx;
use namada::tendermint::node::info::ListenAddress;
use namada_apps::client::tendermint_websocket_client::{
    TendermintWebsocketClient, WebSocketAddress,
};
use namada_apps::tendermint_config::net::Address as TendermintAddress;
use namada_apps::tendermint_rpc::Client;
use std::time::Duration;

const RAW_RPC_ADDR: &str = "tcp://127.0.0.1:26657";

fn get_tm_websocket_client(rpc_addr: &str) -> eyre::Result<TendermintWebsocketClient> {
    let listen_addr = ListenAddress::new(rpc_addr.to_string());
    let remote_addr = TendermintAddress::from_listen_address(&listen_addr).unwrap();
    let websocket_addr = WebSocketAddress::try_from(remote_addr)?;
    let client = TendermintWebsocketClient::open(websocket_addr, Some(Duration::from_secs(5)))?;
    Ok(client)
}

pub async fn submit(tx: Tx) -> eyre::Result<()> {
    let client = get_tm_websocket_client(RAW_RPC_ADDR)?;
    let tm_status = client.status().await.unwrap();
    tracing::info!(?tm_status, "Got status from Tendermint");
    let resp = client.broadcast_tx_commit(tx.to_bytes().into()).await?;
    tracing::info!("Broadcast transaction");
    println!("{:#?}", resp);
    Ok(())
}
