use clap::{Args, Parser, Subcommand};
use command_autocomplete::carapace::{run_carapace, CarapaceArgs};
use command_autocomplete::nushell::{run_nushell, NushellArgs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Shell(ShellArgs),
    Bridge(BridgeArgs),
}

#[derive(Debug, Args)]
struct ShellArgs {
    #[clap(subcommand)]
    command: ShellCommand,
}

#[derive(Debug, Subcommand)]
enum ShellCommand {
    Nushell(NushellArgs),
}

#[derive(Debug, Args)]
struct BridgeArgs {
    #[clap(subcommand)]
    command: BridgeCommand,
}

#[derive(Debug, Subcommand)]
enum BridgeCommand {
    Carapace(CarapaceArgs),
}

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();

    match args.command {
        Command::Bridge(bridge) => match bridge.command {
            BridgeCommand::Carapace(args) => run_carapace(args),
        },
        Command::Shell(shell) => match shell.command {
            ShellCommand::Nushell(args) => run_nushell(args),
        },
    }
}
