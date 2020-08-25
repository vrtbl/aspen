use std::path::PathBuf;

use passerine::{
    common::source::Source,
    compiler::{lex::lex, parse::parse, gen::gen},
    vm::vm::VM,
};

use crate::{
    SOURCE,
    ENTRYPOINT,
    manifest::Manifest,
};

pub fn run(path: PathBuf) -> Result<(), String> {
    // just one file, for now
    let manifest = Manifest::package(&path)?;
    let file = path.join("src").join(ENTRYPOINT);

    let source = Source::path(file)
        .map_err(|_| format!("Could not find source entrypoint '{}/{}'", SOURCE, ENTRYPOINT))?;
    println!("source: {:#?}", source);

    let tokens = lex(source)
        .map_err(|e| e.to_string())?;
    println!("tokens: {:#?}", tokens);

    let ast = parse(tokens)
        .map_err(|e| e.to_string())?;
    println!("ast: {:#?}", ast);

    let bytecode = gen(ast);
    println!("bytecode: {:#?}", bytecode);
    let vm = VM::init();

    // vm.run(bytecode)
    //     .map_err(|e| e.to_string())?;

    Err("Not yet implemented".to_string())
}
