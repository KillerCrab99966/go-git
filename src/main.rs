use crate::args::{GoGitArgs, GoGitCommand};
use clap::Parser;
use std::env::set_current_dir;
use std::fs::create_dir;
use std::process::Command;

mod args;

fn main() {
    // Get the user input
    let args = GoGitArgs::parse();

    match args.command {
        GoGitCommand::Init(args) => init(args.module_name),
        GoGitCommand::New(args) => {
            // Make the target dir
            create_dir(args.dir.clone()).expect("Failed to create target directory.");

            // Set the program's cwd to the new dir
            set_current_dir(args.dir).expect("Failed to change directory.");

            init(args.module_name);
        }
    }
}

/// Initialises golang and git for the cwd.
fn init(module_name: String) {
    // Initialise git
    Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to initialise git.");

    // Initialise a golang module
    Command::new("go")
        .args(["mod", "init", module_name.as_str()])
        .status()
        .expect("Failed to initialise git.");
}
