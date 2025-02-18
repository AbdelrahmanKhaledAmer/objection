use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

use super::super::lexer::tokens::Token;
use super::super::lexer::tokens::TokenType;
use super::ast::NodeBlock;
use super::ast::NodeExpr;
use super::ast::NodeFunc;
use super::ast::NodeIdent;
use super::ast::NodeLiteral;
use super::ast::NodeProgram;
use super::ast::NodeReturn;
use super::ast::NodeStatement;
use super::ast::NodeType;

pub fn parse(tokens: Vec<Token>) -> NodeProgram {
    let mut token_iter = tokens.iter().peekable();
    let mut functions = vec![];
    let mut vars: HashMap<String, String> = HashMap::new();

    while let Some(token) = token_iter.peek() {
        if token.token_type == TokenType::EOF {
            break;
        }
        let function = parse_function(&mut token_iter, &mut vars);
        functions.push(function);
    }

    NodeProgram::new(functions)
}

fn parse_function(
    token_iter: &mut Peekable<Iter<Token>>,
    vars: &mut HashMap<String, String>,
) -> NodeFunc {
    let token = token_iter.peek().unwrap();
    let ident_name = token.value.clone();

    if vars.contains_key(ident_name.as_str()) {
        panic!("Function {} already defined", ident_name);
    }

    let mut identifier = parse_ident(token_iter, vars);
    parse_symbol(token_iter, TokenType::LP);
    parse_symbol(token_iter, TokenType::RP);
    parse_symbol(token_iter, TokenType::Colon);
    let return_type = parse_type(token_iter);
    let func_type = format!("() -> {}", return_type.get_name());
    vars.insert(identifier.get_name(), func_type.clone());
    identifier.set_metatype(func_type);
    parse_symbol(token_iter, TokenType::Assign);
    let body = parse_block(token_iter, vars);
    NodeFunc::new(identifier, return_type, body)
}

fn parse_ident(
    token_iter: &mut Peekable<Iter<Token>>,
    vars: &mut HashMap<String, String>,
) -> NodeIdent {
    let token = token_iter.next().unwrap();
    let value = token.value.clone();
    let possible_type = vars.get(value.as_str());
    if token.token_type != TokenType::Id {
        panic!("{}", panic_str(token, TokenType::Id));
    }
    NodeIdent::new(token.value.clone(), possible_type)
}

fn parse_type(token_iter: &mut Peekable<Iter<Token>>) -> NodeType {
    let token = token_iter.next().unwrap();
    // TODO: I will need to figure out how to handle types ...
    if token.token_type != TokenType::Int {
        panic!("{}", panic_str(token, TokenType::Int));
    }
    NodeType::new(token.value.clone())
}

fn parse_block(
    token_iter: &mut Peekable<Iter<Token>>,
    vars: &mut HashMap<String, String>,
) -> NodeBlock {
    parse_symbol(token_iter, TokenType::LB);
    let mut statements = vec![];

    while let Some(token) = token_iter.peek() {
        if token.token_type == TokenType::RB {
            break;
        }
        let statement = parse_statement(token_iter, vars);
        statements.push(statement);
    }

    parse_symbol(token_iter, TokenType::RB);
    NodeBlock::new(statements)
}

fn parse_statement(
    token_iter: &mut Peekable<Iter<Token>>,
    vars: &mut HashMap<String, String>,
) -> NodeStatement {
    let next_token = token_iter.peek().unwrap();
    if next_token.token_type == TokenType::Ret {
        let statement = NodeStatement::Return(parse_return(token_iter, vars));
        parse_symbol(token_iter, TokenType::Semi);
        return statement;
    }
    panic!(
        "Unexpected start of statement:\n\tLine: {}, Col: {}\n\t{:?}",
        next_token.line, next_token.column, next_token.value
    );
}

fn parse_return(
    token_iter: &mut Peekable<Iter<Token>>,
    vars: &mut HashMap<String, String>,
) -> NodeReturn {
    parse_symbol(token_iter, TokenType::Ret);
    let expr = parse_expression(token_iter, vars);
    NodeReturn::new(expr)
}

fn parse_expression(
    token_iter: &mut Peekable<Iter<Token>>,
    vars: &mut HashMap<String, String>,
) -> NodeExpr {
    let mut peeker = token_iter.clone();
    let token = peeker.next().unwrap();
    // TODO: This will need to change once other types are supported
    if token.token_type == TokenType::IntLit {
        return NodeExpr::Literal(parse_literal(token_iter));
    } else if token.token_type == TokenType::Id {
        return NodeExpr::Id(parse_ident(token_iter, vars));
    } else {
        panic!("{}", panic_str(token, TokenType::IntLit));
    }
}

fn parse_literal(token_iter: &mut Peekable<Iter<Token>>) -> NodeLiteral {
    let token = token_iter.next().unwrap();
    if token.token_type != TokenType::IntLit {
        panic!("{}", panic_str(token, TokenType::IntLit));
    }
    NodeLiteral::new(token.value.clone())
}

fn parse_symbol(token_iter: &mut Peekable<Iter<Token>>, token_type: TokenType) {
    let token = token_iter.peek().unwrap();
    if token.token_type != token_type {
        panic!("{}", panic_str(token, token_type));
    }
    token_iter.next();
}

fn panic_str(token: &Token, expected: TokenType) -> String {
    let mut err_str = String::from("Error: Unexpected token.\n");
    err_str.push_str(format!("\tLine: {}, Col: {}\n", token.line, token.column).as_str());
    err_str.push_str(format!("\tExpected: {:?}, Found: {:?}", expected, token.token_type).as_str());
    err_str
}
