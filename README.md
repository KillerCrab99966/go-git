# Go Git

A simple CLI written in Rust with the `clap` crate that creates or initialises a golang module and git with one command.

## Usage

```
go-git [OPTIONS] <COMMAND>

Commands:
  init  Initialises a golang module and git in the current directory
  new   Creates a new subdirectory and initialises it
  help  Print this message or the help of the given subcommand(s)

Options:
  -b, --bin      Create a package with a main.go file
  -g, --git      Don't initialise git (defaults to false)
  -h, --help     Print help
  -V, --version  Print version
```

Run `go-git help [COMMAND]` for more info.