use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct GoGitArgs {
    #[command(subcommand)]
    pub command: GoGitCommand,

    #[arg(short, long)]
    /// Don't initialise git (defaults to false)
    pub git: bool,
}

#[derive(Debug, Subcommand)]
pub enum GoGitCommand {
    /// Initialises golang and git in the current directory.
    Init(InitArgs),
    /// Creates a new subdirectory and initialises it with golang and git.
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
