use std::env::args;
#[allow(dead_code)]

#[derive(Debug)]
pub struct ReturnStruct{
    pub src_path: String,
    pub output_path: String,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
    pub emit_asm: bool
}

pub fn arg_parse() ->  Option<ReturnStruct> {
    /*
    1. get the src path -> return
    2. get the output path -> return 
    3. check for aditional arguments to compiler
     */
    let args: Vec<String> = args().skip(1).collect();
    if args.len() < 2 {
        println!("Error: Supplied {} arguments. At least 2 arguments must be supplied: source and output.",args.len());
        return None;
    }
    let lex = args.contains(&("--lex".to_string()));
    let parse = args.contains(&("--parse".to_string()));
    let codegen = args.contains(&("--codegen".to_string()));
    let emit_asm = args.contains(&("--emit_asm".to_string()));
    
    let num_options = lex as usize+parse as usize + codegen as usize +emit_asm as usize;
    let src: String = args[num_options].clone();
    let out:String;
    if args.contains(&("-o".to_string())){
        out = args[args.len()-1].clone();
    }
    else {
        let parts: Vec<&str> = src.split(".").collect();
        out = if parts.len()>1{
            parts[..parts.len() - 1].join(".")
        }else {
            src.to_string()
        };
    }
    
    let ret = ReturnStruct{src_path: src,
                               output_path:out,
                               lex,
                               parse,
                               codegen,
                               emit_asm}; 

    return Some(ret);
    
} 