use namada::proto::Tx;
use namada::tendermint::node::info::ListenAddress;
use namada_apps::facade::tendermint_rpc::{Client, WebSocketClient};
use namada_apps::tendermint_config::net::Address as TendermintAddress;

const RAW_RPC_ADDR: &str = "tcp://127.0.0.1:26657";

async fn get_tm_websocket_client(rpc_addr: &str) -> eyre::Result<WebSocketClient> {
    let listen_addr = ListenAddress::new(rpc_addr.to_string());
    let remote_addr = TendermintAddress::from_listen_address(&listen_addr).unwrap();
    let (client, driver) = WebSocketClient::new(remote_addr).await?;
    tokio::spawn(async move {
        driver.run().await.unwrap();
    });
    Ok(client)
}

pub async fn submit(tx: Tx) -> eyre::Result<()> {
    let client = get_tm_websocket_client(RAW_RPC_ADDR).await?;
    let tm_status = client.status().await.unwrap();
    tracing::info!(?tm_status, "Got status from Tendermint");
    let resp = client.broadcast_tx_commit(tx.to_bytes().into()).await?;
    tracing::info!("Broadcast transaction");
    println!("{:#?}", resp);
    Ok(())
}
