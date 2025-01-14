use clap::{Parser, Subcommand};
use version_command::VersionCommandArgs;

pub(crate) mod version_command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Version(VersionCommandArgs),
}

pub(crate) fn run() {
    match Cli::parse().command {
        Commands::Version(args) => version_command::run(args),
    }
}
