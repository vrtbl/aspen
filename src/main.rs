use passerine;
use structopt::StructOpt;

// argument parser and configuation
pub mod cli;
pub mod manifest;

// command implementations
pub mod new;
pub mod update;
pub mod publish;
pub mod run;
pub mod test;
pub mod bench;
pub mod doc;
pub mod debug;

use std::env::current_dir;
use crate::cli::{Aspen, Command};

pub const MANIFEST:   &str = "Aspen.toml";
pub const SOURCE:     &str = "src";
pub const ENTRYPOINT: &str = "main.pn";

fn main() {
    let args = Aspen::from_args();

    // package root is the cwd if not specified
    let path = match args.package {
        Some(p) => p,
        None => current_dir()
            .expect("Could not retrieve current working directory"),
    };

    let result = match args.command {
        Command::New => new::new(path),
        Command::Run => run::run(path),
    };

    if let Err(r) = result { eprintln!("{}", r); }
}
