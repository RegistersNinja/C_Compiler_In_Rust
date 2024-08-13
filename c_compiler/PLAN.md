main: 
1. call gcc preprocessor
   gcc -E -P INPUT_FILE -o PREPROCESSED_FILE
2. call the compiler. use mod compiler
   1. call lexer
      1. support --lex
   2. call parser
      1. support --parse
   3. assembly generator
      1. support --codegen
   4. code emission

3. call assembler and linker
   gcc ASSEMBLY_FILE -o OUTPUT_FILE



4. Use 'match' to parse arguments in driver
   https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
5. implement lexer

### Lexer
1. Use regex