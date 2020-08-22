use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Creates a new Passerine project")]
    New,
    // Update,
    // Publish,
    Run,
    // Test,
    // Bench,
    // Doc,
    // Debug,
}

#[derive(StructOpt, Debug)]
#[structopt(no_version, about)]
pub struct Aspen {
    #[structopt(flatten)]
    command: Command,

    #[structopt(parse(from_os_str))]
    project: Option<PathBuf>, // if `None` assume cwd

    #[structopt(short, long)]
    verbose: bool,
}
