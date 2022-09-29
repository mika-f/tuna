use clap::Parser;

mod client_version;

#[derive(Parser)]
pub enum Web3SubCommand {
    #[clap()]
    ClientVersion(client_version::Args),
}

pub async fn exec(
    globals: crate::commands::globals::GlobalArgs,
    args: Web3SubCommand,
) -> anyhow::Result<()> {
    match args {
        Web3SubCommand::ClientVersion(args) => client_version::exec(globals, args).await,
    }
}
