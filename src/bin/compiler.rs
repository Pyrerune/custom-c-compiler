use c_compiler::{Lexer, to_digit, Parser, generate};
use std::fs::File;
use std::env::args;
use std::io::{Read, Write};
use std::process::Command;
fn main() {
    let args: Vec<String> = args().collect();
    let mut file = File::open(&args[args.len()-1]).expect("File Does Not Exist");
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    let lexer = Lexer::new(buf);
    let test = lexer.lex();

    let parser = Parser::new(test);
    let res = parser.parse();

    let mut gcc = Command::new("gcc");

    if res.is_ok() {
        let asm = generate(res.ok().unwrap());
        //println!("{}", asm);
        let filename = format!("{}.s", &args[args.len()-1].split(".").collect::<Vec<&str>>()[0]);
        let new_file = File::create(&filename);
        let mut new_file = new_file.expect("File failed to create");
        new_file.write_all(asm.as_bytes());
        gcc.arg(&filename);
        gcc.arg("-o");
        gcc.arg("a.out");
        gcc.spawn();

    }

}


//TODO FIX TESTS