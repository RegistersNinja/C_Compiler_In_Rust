/*
.globl _main
_main:
    movl    $2, %eax
    ret
*/
use std::fs::File;
use std::io::{self, Write};
use super::parser::ASTNode;

pub fn assembly_generator(ast: &ASTNode, output_path: &String) -> io::Result<String> {
    let mut assembly = String::new();
    ast.program.generate_assembly(&mut assembly, 0);

    if let path = output_path {
        let mut file = File::create(path)?;
        file.write_all(assembly.as_bytes())?;
    }

    Ok(assembly)
}