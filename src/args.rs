use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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
    #[clap(
        about = "Borsh-serialize a Tx in the protocol transaction format (arbitrarily this is an EthereumStateUpdate protocol tx)"
    )]
    WriteProtocolTx(WriteTx),
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
