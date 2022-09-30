use clap::Parser;

pub mod base_entity;
pub mod eth;
pub mod globals;
pub mod net;
pub mod web3;

#[derive(Parser)]
pub enum SubCommandArgs {
    #[clap(subcommand)]
    Eth(eth::EthSubCommand),

    #[clap(subcommand)]
    Net(net::NetSubCommand),

    #[clap(subcommand)]
    Web3(web3::Web3SubCommand),
}
