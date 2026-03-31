use anyhow::Result;
use clap::{Parser, Subcommand};

mod clippy;
mod common;
mod external;

#[derive(Parser)]
#[command(
    name = "cargo xtask",
    bin_name = "cargo xtask",
    about = "Workspace-related developer tools"
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmds,
}

#[derive(Subcommand)]
enum Cmds {
    Clippy(clippy::ClippyArgs),
    Openapi(external::External),
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Cmds::Clippy(args) => clippy::run_cmd(args),
        Cmds::Openapi(external) => external.exec_bin("lumen-dropshot-apis"),
    }
}
