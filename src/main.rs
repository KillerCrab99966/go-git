use crate::args::InitArgs;
use clap::Parser;
use std::process::Command;

mod args;

fn main() {
    // Get the user input
    let args = InitArgs::parse();

    // Initialise git
    Command::new("git").arg("init").status().expect("Failed to initialise git");

    // Initialise a golang module
    Command::new("go").args(["mod", "init", args.module_name.as_str()]).status().expect("Failed to initialise git");
}
