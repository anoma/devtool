use namada::proto::{Signed, SignedTxData, Tx};
use namada::types::address::Address;
use namada::types::key::common::SecretKey;
use namada::types::storage::Epoch;
use namada::types::token::Amount;
use namada::types::transaction::protocol::ProtocolTxType;
use namada::types::transaction::{EncryptionKey, Fee, TxType, WrapperTx};

use borsh::de::BorshDeserialize;

pub fn read_tx(borsh_serialized_tx: &[u8]) -> eyre::Result<Tx> {
    let tx: Tx = Tx::try_from_slice(borsh_serialized_tx)?;
    {
        let data_len: usize = match tx.data {
            None => 0,
            Some(ref data) => data.len(),
        };
        let timestamp_dbg = format!("{:?}", tx.timestamp);
        let timestamp: &str = timestamp_dbg.as_str();
        tracing::info!(
            code_len = tx.code.len(),
            data_len = data_len,
            timestamp = timestamp,
            "Deserialized tx",
        );
    }
    Ok(tx)
}

pub fn print_tx(tx: &Tx) {
    println!("Code: {} bytes", tx.code.len());
    println!("Timestamp: {:?}", tx.timestamp);
    print_tx_data(tx.data.clone());
}

fn print_signed_tx_data(signed: SignedTxData) {
    println!("Signature: {:#?}", signed.sig);
    print_tx_data(signed.data);
}

fn print_signed(signed: Signed<Vec<u8>>) {
    println!("Signature: {:#?}", signed.sig);
    println!("Data: {} bytes", signed.data.len());
}

fn print_tx_data(data: Option<Vec<u8>>) {
    if data.is_none() {
        println!("Data: None");
        return;
    }
    let data = data.as_ref().unwrap();

    if let Ok(signed) = SignedTxData::try_from_slice(&data[..]) {
        println!("Data: (found SignedTxData)");
        print_signed_tx_data(signed);
    } else if let Ok(tx_type) = TxType::try_from_slice(&data[..]) {
        match tx_type {
            TxType::Protocol(protocol_tx) => {
                println!("Data: (found TxType::Protocol)");
                println!("Public key: {:#?}", protocol_tx.pk);
                match protocol_tx.tx {
                    ProtocolTxType::EthEventsVext(_) => {
                        println!("Tx: (found ProtocolTxType::EthEventsVext");
                    }
                    _ => println!("Tx: found ProtocolTxType"),
                }
            }
            _ => println!("Data: found TxType"),
        }
    } else if let Ok(signed) = Signed::<Vec<u8>>::try_from_slice(data) {
        println!("Data: (found Signed)");
        print_signed(signed);
    } else if let Ok(s) = String::from_utf8(data.to_owned()) {
        println!("Data: (found what might be a UTF-8 string)");
        println!("{}", s);
    } else {
        println!("Data: {} bytes", data.len());
    }
}

/// address of the XAN vp_token account that is commonly used in Anoma testnets
fn xan() -> Address {
    Address::decode(
        "atest1v4ehgw36x3prswzxggunzv6pxqmnvdj9xvcyzvpsggeyvs3cg9qnywf589qnwvfsg5erg3fkl09rg5",
    )
    .expect("The token address decoding shouldn't fail")
}

/// Create a normal signed wrapped tx (i.e. a SignedTxData)
pub fn create_tx(wasm: Vec<u8>, unsigned_data: Option<Vec<u8>>, sk: SecretKey) -> Tx {
    let inner_tx = Tx::new(wasm, unsigned_data).sign(&sk);
    let wrapped = WrapperTx::new(
        Fee {
            amount: Amount::whole(10),
            token: xan(),
        },
        &sk,
        Epoch(10),
        10000.into(),
        inner_tx,
        EncryptionKey::default(),
    );
    wrapped.sign(&sk).unwrap()
}
