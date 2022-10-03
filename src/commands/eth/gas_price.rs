use clap::Parser;
use jsonrpsee::rpc_params;

use crate::{client::eth, commands::base_entity::JsonEntity};

#[derive(Parser)]
#[clap()]
pub struct Args {}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    _args: Args,
) -> anyhow::Result<()> {
    let response = eth::request::<String>(globals.endpoint(), "eth_gasPrice", rpc_params![]).await?;
    let deserializer = |w: String| {
        let val = u128::from_str_radix(&w[2..], 16).unwrap();
        JsonEntity::new(val)
    };

    match response.print_json::<u128>(
        globals.no_colors(),
        globals.no_deserialize(),
        Some(Box::new(deserializer)),
    ) {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}
