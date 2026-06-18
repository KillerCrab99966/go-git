use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct GoInitArgs {
    #[command(subcommand)]
    pub command: GoInitCommand,

    /// Create a package with a main.go file.
    #[arg(short, long)]
    pub bin: bool,

    /// Don't initialise git (defaults to false).
    #[arg(short, long)]
    pub git: bool,
}

#[derive(Debug, Subcommand)]
pub enum GoInitCommand {
    /// Initialises a golang module and git in the current directory.
    Init(InitArgs),

    /// Creates a new subdirectory and initialises it.
    New(NewArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The module name (passed to go mod init).
    pub module_name: String,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The name of the subdirectory to create.
    pub dir: String,

    /// The module name (passed to go mod init).
    pub module_name: String,
}
