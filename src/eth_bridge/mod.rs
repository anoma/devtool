use std::str::FromStr;

use crate::args::FakeTransferToNamada;
use borsh::BorshSerialize;
use eyre::{Context, Result};
use hyper::{Body, Client, Method, Request, Response};
use namada::types::{
    address,
    ethereum_events::{EthAddress, EthereumEvent, TransferToNamada},
};
use rand::Rng;

pub const ETHEREUM_EVENT_ENDPOINT: &str = "http://0.0.0.0:3030/eth_events";

pub const DAI_ERC20_ETH_ADDRESS_CHECKSUMMED: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

pub(crate) async fn send_fake_transfer_to_namada(
    FakeTransferToNamada {
        amount,
        receiver,
        asset,
        nonce,
    }: FakeTransferToNamada,
) -> Result<()> {
    let receiver = address::Address::decode(receiver)?;
    let asset = EthAddress::from_str(&asset)?;
    let transfer = TransferToNamada {
        amount,
        asset,
        receiver,
    };
    let transfers = vec![transfer];
    let nonce = match nonce {
        Some(nonce) => nonce.into(),
        None => {
            let mut rng = rand::thread_rng();
            let rn: u64 = rng.gen();
            println!("No nonce provided, generated random nonce {}", rn);
            rn.into()
        }
    };
    let event = EthereumEvent::TransfersToNamada { nonce, transfers };
    println!("posting event - {:#?}", event);
    let resp = send(&event).await?;
    println!("Response: {:#?}", resp);
    Ok(())
}

async fn send(event: &EthereumEvent) -> Result<Response<Body>> {
    let event = event.try_to_vec()?;

    let req = Request::builder()
        .method(Method::POST)
        .uri(ETHEREUM_EVENT_ENDPOINT)
        .header("content-type", "application/octet-stream")
        .body(Body::from(event))?;

    let client = Client::new();
    client.request(req).await.wrap_err_with(|| "sending event")
}
