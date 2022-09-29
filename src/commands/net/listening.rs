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
    let response =
        eth::request::<bool>(globals.endpoint(), "net_listening", rpc_params![]).await;

    match response {
        Ok(response) => response.print_json(globals.no_colors())?,
        Err(err) => return Err(err),
    };

    Ok(())
}
