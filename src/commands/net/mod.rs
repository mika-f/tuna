use clap::Parser;

mod version;

#[derive(Parser)]
pub enum NetSubCommand {
    #[clap()]
    Version(version::Args),
}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    args: NetSubCommand,
) -> anyhow::Result<()> {
    match args {
        NetSubCommand::Version(args) => version::exec(globals, args).await,
    }
}
