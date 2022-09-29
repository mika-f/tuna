use clap::Parser;

mod listening;
mod version;

#[derive(Parser)]
pub enum NetSubCommand {
    #[clap()]
    Listening(listening::Args),

    #[clap()]
    Version(version::Args),
}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    args: NetSubCommand,
) -> anyhow::Result<()> {
    match args {
        NetSubCommand::Listening(args) => listening::exec(globals, args).await,
        NetSubCommand::Version(args) => version::exec(globals, args).await,
    }
}
