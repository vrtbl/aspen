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

use cli::Aspen;

fn main() {
    let args = Aspen::from_args();
    println!("{:?}", args);
}
