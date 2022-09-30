use clap::Parser;

mod chain_id;
mod protocol_version;

#[derive(Parser)]
pub enum EthSubCommand {
    #[clap()]
    ChainId(chain_id::Args),

    #[clap()]
    ProtocolVersion(protocol_version::Args),
}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    args: EthSubCommand,
) -> anyhow::Result<()> {
    match args {
        EthSubCommand::ChainId(args) => chain_id::exec(globals, args).await,
        EthSubCommand::ProtocolVersion(args) => protocol_version::exec(globals, args).await,
    }
}
