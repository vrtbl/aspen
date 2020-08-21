#[derive(StructOpt)]
pub enum Command {
    New,
    // Update,
    // Publish,
    Run,
    // Test,
    // Bench,
    // Doc,
    // Debug,
}

#[derive(StructOpt)]
#[structopt(about = "A small extensible programming language")]
pub struct Aspen {
    command: Command,
    project: Option<PathBuf>,
    #[structopt(short, long, default_value = false)]
    verbose: bool,
}
