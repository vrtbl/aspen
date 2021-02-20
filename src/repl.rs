// TODO: preserve values using module system.

// use std::io::{self, Read};
// use std::io::Write; // <--- bring the trait into scope
//
// use passerine::{
//     common::{closure::Closure, source::Source},
//     compiler::{lex, parse, desugar, gen},
//     vm::vm::VM,
// };
//
// pub fn repl() {
//     println!("Hit enter twice to evaluate.");
//     println!("Hit ^C to quit.\n");
//
//     let mut vm = VM::init();
//
//     loop {
//         let mut to_eval = String::new();
//         loop {
//             print!("| ");
//             std::io::stdout().flush().unwrap();
//             match io::stdin().read_line(&mut to_eval) {
//                 Ok(l) => if l <= 1 { break; },
//                 _ => ()
//             }
//         }
//
//         let source    = Source::source(&to_eval);
//         let tokens    =   lex(source);
//         let ast       = tokens.and_then(parse);
//         let cst       = ast.and_then(desugar);
//         let bytecode  = cst.and_then(gen);
//
//         let lambda = match bytecode {
//             Err(e) => {
//                 println!("\n{}\n", e);
//                 continue;
//             },
//             Ok(l) => l,
//         };
//
//         match vm.run(Closure::wrap(lambda)) {
//             Ok(()) => {
//                 let data = vm.stack.pop_data();
//                 println!(". {:?}\n", data)
//             },
//             Err(e) => {
//                 println!("\n{}\n", e);
//             },
//         }
//     }
// }
