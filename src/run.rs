use std::path::PathBuf;

use passerine::{
    common::{closure::Closure, source::Source},
    compiler::{lex, parse, desugar, gen},
    vm::vm::VM,
};

use crate::{
    SOURCE,
    ENTRYPOINT,
    manifest::Manifest,
};

pub fn run(path: PathBuf) -> Result<(), String> {
    // just one file, for now
    let (_manifest, path) = Manifest::package(&path)?;
    let file = path.join("src").join(ENTRYPOINT);

    let source = Source::path(file)
        .map_err(|_| format!("Could not find source entrypoint '{}/{}'", SOURCE, ENTRYPOINT))?;

    let tokens    =   lex(source).map_err(|e| e.to_string())?;
    let ast       = parse(tokens).map_err(|e| e.to_string())?;
    let cst       =  desugar(ast).map_err(|e| e.to_string())?;
    let bytecode  =      gen(cst).map_err(|e| e.to_string())?;

    let mut vm = VM::init();
    vm.run(Closure::wrap(bytecode))
        .map_err(|e| e.to_string())?;

    Ok(())
}
