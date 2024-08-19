/*
types: expression (2+2), identifier (my_cool_function, main), constant (int), statement (return)
grammar:
    <program> ::= <function>
    <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
    <statement> ::= "return" <exp> ";"
    <exp> ::= <int>
    <identifier> ::= ? An identifier token ?
    <int> ::= ? A constant token ?

Token: int_keyword, Value: int
Token: main_keyword, Value: main
Token: open_parenthesis, Value: (
Token: void_keyword, Value: void
Token: close_parenthesis, Value: )
Token: open_brace, Value: {
Token: return_keyword, Value: return
Token: constant, Value: 2
Token: semicolon, Value: ;
Token: close_brace, Value: }

*/

use std::error::Error;

pub struct ASTNode {
    program: Program,
}

impl ASTNode {
    pub fn recursive_print(&self) {
        self.program.recursive_print(0);
    }
}

struct Program {
    program: Function,
}

impl Program {
    fn recursive_print(&self, indent: usize) {
        self.program.recursive_print(indent);
    }
}

struct Function {
    declaration: Declaration,
    body: Body,
}

impl Function {
    fn recursive_print(&self, indent: usize) {
        self.declaration.recursive_print(indent);
        self.body.recursive_print(indent);
    }
}

struct Declaration {
    return_type: String,
    name: String,
    parameters: String,
}

impl Declaration {
    fn recursive_print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}Declaration:", indent_str);
        println!("{}  Return Type: {}", indent_str, self.return_type);
        println!("{}  Name: {}", indent_str, self.name);
        println!("{}  Parameters: {}", indent_str, self.parameters);
    }
}

struct Body {
    statement: Statement,
}

impl Body {
    fn recursive_print(&self, indent: usize) {
        self.statement.recursive_print(indent);
    }
}

enum Statement {
    Return(Expression),
}

impl Statement {
    fn recursive_print(&self, indent: usize) {
        match self {
            Statement::Return(expr) => {
                let indent_str = " ".repeat(indent);
                println!("{}Return Statement:", indent_str);
                expr.recursive_print(indent + 2);
            }
        }
    }
}

struct Expression {
    r#const: Constant,
    end_expression: EndExpression,
}

impl Expression {
    fn recursive_print(&self, indent: usize) {
        self.r#const.recursive_print(indent);
        self.end_expression.recursive_print(indent);
    }
}

enum EndExpression {
    Semicolon,
}

impl EndExpression {
    fn recursive_print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        match self {
            EndExpression::Semicolon => println!("{};", indent_str),
        }
    }
}

enum Constant {
    Val(i64),
}

impl Constant {
    fn recursive_print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        match self {
            Constant::Val(val) => println!("{}Constant: {}", indent_str, val),
        }
    }
}

fn parse_statement(tokenized_list: &mut Vec<(String, String)>) -> Result<Statement, String> {
    if tokenized_list.is_empty() {
        return Err("Parse statement failed".to_string());
    }
    let token = tokenized_list.remove(0);
    match token.0.as_str() {
        "return_keyword" => {
            let expression = parse_expression(tokenized_list).unwrap();
            let statement = Statement::Return(expression);
            Ok(statement)
        },
        _ => Err("Parse statement failed".to_string())
    }
}

fn parse_expression(tokenized_list: &mut Vec<(String, String)>) -> Result<Expression, String> {
    if tokenized_list.len() < 2 {
        return Err("Parse expression failed".to_string());
    }
    let mut token = tokenized_list.remove(0);
    match token.0.as_str() {
        "constant" => {
            let constant: i64 = token.1.as_str().parse().unwrap();
            token = tokenized_list.remove(0);
            match token.0.as_str() {
                "semicolon" => {
                    let expression = Expression {
                        r#const: Constant::Val(constant),
                        end_expression: EndExpression::Semicolon
                    };
                    Ok(expression)
                },
                _ => return Err("Parse expression failed".to_string()),
            }
        },
        _ => return Err("Parse expression failed".to_string())
    }
}

fn expect(expect: String, actual: String) -> bool {
    if expect.eq(&actual) {
        return true
    } 
    return false
    
}

fn parse_declaration(tokenized_list: &mut Vec<(String, String)>) -> Result<Declaration, String> {
    if tokenized_list.len() < 3 {
        return Err("Parse declaration failed".to_string());
    }
    let return_type = tokenized_list.remove(0).1;
    match return_type.as_str() {
        "int" => {
            let name = tokenized_list.remove(0).1;
            match name.as_str() {
                "main" => {
                    if expect("(".to_string(),tokenized_list.remove(0).1) {
                        let parameters = tokenized_list.remove(0).1;
                        if expect(")".to_string(), tokenized_list.remove(0).1) {
                            Ok(Declaration {
                                return_type,
                                name,
                                parameters,
                            })
                        } else {
                            Err("Parse declaration failed".to_string())
                        }

                    } else {
                        Err("Parse declaration failed".to_string())
                    }
                },  _ => return Err("Parse declaration failed".to_string())
            }
        }, _ => return Err("Parse declaration failed".to_string())
    }
}

fn parse_body(tokenized_list: &mut Vec<(String, String)>) -> Result<Body, String> {
    let statement = parse_statement(tokenized_list)?;
    Ok(Body { statement })
}

fn parse_function(tokenized_list: &mut Vec<(String, String)>) -> Result<Function, String> {
    let declaration = parse_declaration(tokenized_list)?;
    if expect("{".to_string(),tokenized_list.remove(0).1){
        let body = parse_body(tokenized_list)?;
        if expect("}".to_string(), tokenized_list.remove(0).1){
            Ok(Function {declaration, body})
        }else {return Err("Parse body failed".to_string())}
    } else {return Err("Parse body failed".to_string())}
    
    
}

fn parse_program(tokenized_list: &mut Vec<(String, String)>) -> Result<Program, String> {
    let function = parse_function(tokenized_list)?;
    Ok(Program { program: function })
}

pub fn parser(mut tokenized_list: Vec<(String, String)>) -> Result<ASTNode, Box<dyn Error>> {
    let program = parse_program(&mut tokenized_list)?;
    Ok(ASTNode {program: program})
}
