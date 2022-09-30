use clap::Parser;

mod client;
mod commands;

#[derive(Parser)]
#[clap(author, about, version)]
struct Args {
    #[clap(subcommand)]
    subcommand: commands::SubCommandArgs,

    #[clap(long = "endpoint", global = true)]
    endpoint: Option<String>,

    #[clap(long = "no-colors", global = true)]
    no_colors: bool,

    #[clap(long = "no-deserialize", global = true)]
    no_serialize: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let globals =
        commands::globals::GlobalArgs::new(args.endpoint, args.no_colors, args.no_serialize);

    let result = match args.subcommand {
        commands::SubCommandArgs::Eth(args) => commands::eth::exec(globals, args).await,
        commands::SubCommandArgs::Net(args) => commands::net::exec(globals, args).await,
        commands::SubCommandArgs::Web3(args) => commands::web3::exec(globals, args).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}
