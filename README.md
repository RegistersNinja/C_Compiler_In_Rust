# C Compiler in Rust

## Description

A Rust implementation of a simple C static program along the lines of:

```C
int main(void) {
    return 2;
}
```

The compilation consists of three stages:

1. The lexer: tokenizes the program.
2. The parser: builds AST (as enums) and handles the proper syntax (as well as descending recursion that practically doesn't exist here because of simplicity)
3. The assembly generator: outputs the assembly

Preprocessor and linker that were used are from ```gcc```.

## Project structure

```/src
/src
├── args.rs
├── compiler
│   ├── assembly_generator.rs
│   ├── lexer.rs
│   ├── mod.rs
│   └── parser.rs
└── main.rs
```

## Reference

1. [Nora Sandler - Blog](https://norasandler.com/2017/11/29/Write-a-Compiler.html)
2. [Writing C compiler](https://nostarch.com/writing-c-compiler)
3. [Probably the best Stack post ever](https://softwareengineering.stackexchange.com/questions/165543/how-to-write-a-very-basic-compiler)
