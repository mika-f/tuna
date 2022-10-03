use clap::Parser;
use jsonrpsee::rpc_params;

use crate::client::eth;

#[derive(Parser)]
#[clap()]
pub struct Args {}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    _args: Args,
) -> anyhow::Result<()> {
    let response = eth::request::<bool>(globals.endpoint(), "eth_mining", rpc_params![]).await?;

    match response.print_json::<bool>(globals.no_colors(), globals.no_deserialize(), None) {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}
