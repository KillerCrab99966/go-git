use crate::args::{GoInitArgs, GoInitCommand};
use clap::Parser;
use std::env::set_current_dir;
use std::fs::{create_dir, write};
use std::process::Command;

mod args;

fn main() {
    // Get the user input
    let input = GoInitArgs::parse();

    match input.command {
        GoInitCommand::Init(args) => init(InitOptions {
            module_name: args.module_name,
            git: !input.git,
            bin: input.bin,
        }),
        GoInitCommand::New(args) => {
            // Make the target dir
            create_dir(args.dir.clone()).expect("Failed to create target directory.");

            // Set the program's cwd to the new dir
            set_current_dir(args.dir).expect("Failed to change directory.");

            init(InitOptions {
                module_name: args.module_name,
                git: !input.git,
                bin: input.bin,
            });
        }
    }
}

struct InitOptions {
    /// The name of the created golang module.
    module_name: String,
    /// Whether to initialise git.
    git: bool,
    /// Wether to create a main.go file for a binary package.
    bin: bool,
}

/// Initialises golang and git (if `git`) for the cwd.
fn init(options: InitOptions) {
    // Initialise git
    if options.git {
        Command::new("git")
            .arg("init")
            .status()
            .expect("Failed to initialise git.");
    } else {
        eprintln!("Not initialising git.");
    }

    // Initialise a golang module
    Command::new("go")
        .args(["mod", "init", options.module_name.as_str()])
        .status()
        .expect("Failed to initialise git.");

    if !options.bin {
        return;
    }

    // Create a main.go file and add some hello world code
    let hello_world = "package main

import \"fmt\"

func main() {
    fmt.Println(\"Hello, 世界\")
}";

    write("main.go", hello_world).expect("Failed to create or write to main.go.");
    eprintln!("Created main.go")
}
