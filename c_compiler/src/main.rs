/*
usage:
    c_compiler [options] src.c -o output

options:
    1. call lexer
       support --lex, stop before parsing
    2. call parser
       support --parse, stop before assembly
    3. call codegen
       --codegen ​directs it to perform lexing, parsing, and assembly generation, but stop before code emission
    *None of these options should produce any output files*
    4. emit assembly -S
       print assembly, stop before asseembling and compiling
      
*/
mod args;
mod compiler;

use std::borrow::Borrow;
use std::env;
use std::process::exit;
use std::process::Command;
use compiler::lexer;
use compiler::parser;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    /*
    1. get the arguments
    2. parse the arguments and save:
        src path
        output name
            if there is no -o then use src name
    3. call gcc preprocessor
        create .i file
    4. call compiler [stub]
        create .s file
    5.call assembler and linker
     */
    let arguments: Option<args::ReturnStruct> = args::arg_parse();

    match arguments{
        None => {
            println!("Arguments struct is none");
            exit(-1);
        }
        Some(arguments) => {
            println!("Arguments are fine");
            //gcc -E -P INPUT_FILE -o PREPROCESSED_FILE
            Command::new("gcc").args(&[
                "-E", "-P", &arguments.src_path, "-o", &(arguments.output_path.clone()+".i")])       
                .output()
                .expect("Failed to preprocess");
            //compiler command goes here

            //lexer
            let lexed = lexer::lexer(arguments.borrow()).unwrap();
            if arguments.lex == true {
                for (token, value) in lexed.iter() {
                    println!("Token: {}, Value: {}",token,value);
                }
            }
            //parser
            let parsed = parser::parser(lexed).unwrap();
            if arguments.parse == true {
                parsed.recursive_print();
            }

            //gcc -m32 assembly.s -o out
            let assembly = compiler::assembly_generator::assembly_generator(&parsed, &arguments.output_path);
            Command::new("gcc").args(&[
                "-c", &arguments.output_path, "-o", &arguments.output_path])       
                .output()
                .expect("Failed to preprocess");

            //assembler & linker command goes here
        }
    }
}