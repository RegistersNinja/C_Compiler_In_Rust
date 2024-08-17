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

use std::fmt::Error;

enum ASTNode {
    Program(Program),
}

struct Program {
    program: Function,
}

struct Function {
    declaration: Declaration,
    body: Body,
}

struct Declaration {
    return_type: String,
    name: String,
    parameters: String,
}

struct Body {
    statement: Statement,
}
enum Statement {
    Return(Expression),
}

struct Expression {
    r#const: Constant,
    end_expression: EndExpression,
}

enum EndExpression {
    Semicolon,
}
enum Constant {
    Val(i64),
}
fn parse_statement(mut tokenized_list: Vec<(String, String)>) -> Result<Statement, Error> {
    if tokenized_list.is_empty() {
        return Err(Error);
    }
    let mut token = tokenized_list.remove(0);
    match token.0.as_str() {
        "return_keyword" => {
            token = tokenized_list.remove(0);
            match token.0.as_str() {
                "constant" => {
                    let constant: i64 = token.1.as_str().parse().unwrap();
                    token = tokenized_list.remove(0);
                    match token.0.as_str() {
                        "semicolon" => {
                            let expression = Expression {
                                r#const: Constant::Val(constant as i64),
                                end_expression: EndExpression::Semicolon,
                            };
                            let statement = Statement::Return(expression);
                            return Ok(statement);
                        }
                        _ => return Err(Error),
                    }
                }
                _ => return Err(Error),
            }
        }
        _ => return Err(Error),
    }
}

fn parse_expression(tokenized_list: Vec<(String, String)>) -> Result<Expression, Error> {}

fn parser(tokenized_list: Vec<(String, String)>) -> Result<ASTNode, Error> {
    let ast = ASTNode::Program;
}
