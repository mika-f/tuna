use clap::Parser;

mod listening;
mod peer_count;
mod version;

#[derive(Parser)]
pub enum NetSubCommand {
    #[clap()]
    Listening(listening::Args),

    #[clap()]
    PeerCount(peer_count::Args),

    #[clap()]
    Version(version::Args),
}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    args: NetSubCommand,
) -> anyhow::Result<()> {
    match args {
        NetSubCommand::Listening(args) => listening::exec(globals, args).await,
        NetSubCommand::PeerCount(args) => peer_count::exec(globals, args).await,
        NetSubCommand::Version(args) => version::exec(globals, args).await,
    }
}
