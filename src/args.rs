use clap::{Args, Parser, Subcommand};
use namada::types::token::Amount;
use std::path::PathBuf;

use crate::eth_bridge::DAI_ERC20_ETH_ADDRESS_CHECKSUMMED;

#[derive(Debug, Parser)]
#[clap(name = "devtool")]
#[clap(about = "Tool for developing on Anoma", long_about = None)]
pub(crate) struct Devtool {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[clap(about = "Borsh-serialize a Tx in the wrapper transaction format")]
    WriteTx(WriteTx),
    #[clap(about = "Read a Borsh-serialized Tx from a file and pretty print it")]
    #[clap(arg_required_else_help = true)]
    ExamineTx {
        #[clap(required = true, parse(from_os_str))]
        tx: PathBuf,
    },
    #[clap(about = "Read a Borsh-serialized Tx from a file and submit it via Tendermint RPC")]
    SubmitTx {
        #[clap(required = true, parse(from_os_str))]
        tx: PathBuf,
    },
    #[clap(about = "Generate a random secret key and print the Borsh serialization")]
    PrintRandomKey,
    #[clap(about = "Submit a fake transfer to Namada")]
    SubmitFakeTransferToNamada(FakeTransferToNamada),
}

#[derive(Args, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct WriteTx {
    #[clap(long, parse(from_os_str))]
    pub(crate) code: PathBuf,
    #[clap(long, parse(from_os_str))]
    pub(crate) data: Option<PathBuf>,
    #[clap(long, parse(from_os_str))]
    pub(crate) out: PathBuf,
    #[clap(long, parse(from_os_str))]
    pub(crate) key: PathBuf,
}

#[derive(Args, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct FakeTransferToNamada {
    #[clap(long, default_value_t = Amount::whole(100), value_parser)]
    pub(crate) amount: Amount,
    #[clap(
        long,
        default_value_t = String::from("atest1v4ehgw36xuunwd6989prwdfkxqmnvsfjxs6nvv6xxucrs3f3xcmns3fcxdzrvvz9xverzvzr56le8f"),
        value_parser
    )]
    pub(crate) receiver: String,
    #[clap(
        long,
        default_value_t = String::from(DAI_ERC20_ETH_ADDRESS_CHECKSUMMED),
        value_parser
    )]
    pub(crate) asset: String,
    #[clap(long)]
    pub(crate) nonce: Option<u64>,
}
