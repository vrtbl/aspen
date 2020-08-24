use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Creates a new Passerine package")]
    New,
    // Update,
    // Publish,
    #[structopt(about = "Run the specified package")]
    Run,
    // Test,
    // Bench,
    // Doc,
    // Debug,
}

#[derive(StructOpt, Debug)]
#[structopt(no_version, about)]
pub struct Aspen {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(parse(from_os_str))]
    pub package: Option<PathBuf>,
}
