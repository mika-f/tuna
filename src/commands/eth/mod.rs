use clap::Parser;

mod accounts;
mod block_number;
mod chain_id;
mod protocol_version;

#[derive(Parser)]
pub enum EthSubCommand {
    #[clap()]
    Accounts(accounts::Args),

    #[clap()]
    BlockNumber(block_number::Args),

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
        EthSubCommand::Accounts(args) => accounts::exec(globals, args).await,
        EthSubCommand::BlockNumber(args) => block_number::exec(globals, args).await,
        EthSubCommand::ChainId(args) => chain_id::exec(globals, args).await,
        EthSubCommand::ProtocolVersion(args) => protocol_version::exec(globals, args).await,
    }
}
