use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct InitArgs {
    /// The module name (passed to `go mod init`)
    pub module_name: String,
}