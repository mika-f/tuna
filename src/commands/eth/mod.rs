use clap::Parser;

mod chain_id;

#[derive(Parser)]
pub enum EthSubCommand {
    #[clap()]
    ChainId(chain_id::Args),
}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    args: EthSubCommand,
) -> anyhow::Result<()> {
    match args {
        EthSubCommand::ChainId(args) => chain_id::exec(globals, args).await,
    }
}
