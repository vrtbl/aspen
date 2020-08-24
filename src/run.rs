use std::path::PathBuf;

use passerine::{
    common::source::Source,
    compiler::{lex::lex, parse::parse, gen::gen},
    vm::vm::VM,
};

use crate::ENTRYPOINT;

pub fn run(path: PathBuf) -> Result<(), String> {
    // just one file, for now
    let file = path.join("src").join(ENTRYPOINT);

    let source = Source::path(file)
        .map_err(|_| "Could not open source")?;
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

    Ok(())
}
