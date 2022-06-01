#![allow(unused)]

use anoma::types::key::common::{self, SecretKey};
use borsh::BorshSerialize;
use color_eyre::eyre::Result;
use eyre::{eyre, Context, Report};
use std::{path::PathBuf, str::FromStr};

// some of these tendermint dependencies may need to be declared as `pub extern` in anoma_apps
use crate::{args, fs, keys, tendermint, tx};
use anoma_apps::tendermint_rpc_abci::Client;

fn deserialize_from_files(
    code: PathBuf,
    data: Option<PathBuf>,
    key: PathBuf,
) -> Result<(Vec<u8>, Option<Vec<u8>>, common::SecretKey)> {
    let wasm_bytes = fs::read_file(code)?;

    let data_bytes = match data {
        None => None,
        Some(data_path) => {
            let data_bytes = fs::read_file(&data_path)?;
            Some(data_bytes)
        }
    };

    let key_bytes = fs::read_file(key)?;
    let key_str = String::from_utf8(key_bytes)?;
    let key_str = key_str.trim_end();
    let key = SecretKey::from_str(key_str)?;

    Ok((wasm_bytes, data_bytes, key))
}

pub(crate) async fn run(cmd: args::Commands) -> Result<()> {
    match cmd {
        args::Commands::WriteProtocolTx(args::WriteTx {
            code,
            data,
            out,
            key,
        }) => {
            let (wasm_bytes, data_bytes, key) = deserialize_from_files(code, data, key)?;

            let protocol_tx = tx::create_protocol_tx(wasm_bytes, data_bytes, key);
            fs::write_file(out, protocol_tx.try_to_vec()?);
        }
        args::Commands::WriteTx(args::WriteTx {
            code,
            data,
            out,
            key,
        }) => {
            let (wasm_bytes, data_bytes, key) = deserialize_from_files(code, data, key)?;

            let tx = tx::create_tx(wasm_bytes, data_bytes, key);
            fs::write_file(out, tx.try_to_vec()?);
        }
        args::Commands::ExamineTx { tx } => {
            let tx_bytes = fs::read_file(&tx)?;
            let deserialized = tx::read_tx(&tx_bytes[..])?;

            tx::print_tx(&deserialized);
        }
        args::Commands::SubmitTx { tx } => {
            let tx_bytes = fs::read_file(&tx)?;
            let deserialized = tx::read_tx(&tx_bytes[..])?;

            tendermint::submit(deserialized).await?;
        }
        args::Commands::PrintRandomKey => {
            let (sk, _) = keys::random_keypair();
            let serialized = sk.try_to_vec()?;
            print!("{}", hex::encode(&serialized));
        }
    }
    Ok(())
}
