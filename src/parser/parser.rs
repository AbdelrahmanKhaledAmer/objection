use std::iter::Peekable;
use std::slice::Iter;

use super::super::lexer::tokens::Token;
use super::super::lexer::tokens::TokenType;
use super::ast::*;

pub fn parse_prog(tokens: Vec<Token>) -> NodeProg {
    let mut token_iter = tokens.iter().peekable();
    let mut functions: Vec<NodeFunc> = Vec::new();
    while let Some(token) = token_iter.peek() {
        if token.token_type == TokenType::EOF {
            break;
        }
        functions.push(parse_func(&mut token_iter));
    }
    NodeProg { functions }
}

fn parse_func(token_iter: &mut Peekable<Iter<Token>>) -> NodeFunc {
    let ident = parse_ident(token_iter);
    parse_symbol(token_iter, TokenType::LP);
    parse_symbol(token_iter, TokenType::RP);
    parse_symbol(token_iter, TokenType::Colon);
    let r_type = parse_type(token_iter);
    parse_symbol(token_iter, TokenType::Assign);
    let block = parse_block(token_iter);
    NodeFunc {
        ident,
        r_type,
        block,
    }
}

fn parse_ident(token_iter: &mut Peekable<Iter<Token>>) -> NodeIdent {
    let token = token_iter.next().unwrap();
    if token.token_type != TokenType::Id {
        panic!(
            "Expected an identifier, got {:?} instead.",
            token.token_type
        );
    }
    NodeIdent {
        name: token.value.clone(),
    }
}

fn parse_symbol(token_iter: &mut Peekable<Iter<Token>>, symbol: TokenType) {
    let token = token_iter.next().unwrap();
    if token.token_type != symbol {
        panic!(
            "Expected the symbol {:?}, got {:?} instead.",
            symbol, token.token_type
        );
    }
}

fn parse_type(token_iter: &mut Peekable<Iter<Token>>) -> NodeType {
    let token = token_iter.next().unwrap();
    let meta = match token.token_type {
        TokenType::Int => TypeMeta::Primitive(PrimitiveType::Int),
        _ => panic!("Expected a known type, got {:?} instead.", token.token_type),
    };
    NodeType { meta }
}

fn parse_block(token_iter: &mut Peekable<Iter<Token>>) -> NodeBlock {
    let mut stmts: Vec<NodeStmt> = Vec::new();
    parse_symbol(token_iter, TokenType::LB);
    while let Some(token) = token_iter.peek() {
        if token.token_type == TokenType::RB {
            break;
        }
        stmts.push(parse_stmt(token_iter));
    }
    parse_symbol(token_iter, TokenType::RB);
    NodeBlock { stmts }
}

fn parse_stmt(token_iter: &mut Peekable<Iter<Token>>) -> NodeStmt {
    let stmt: NodeStmt;
    match token_iter.peek() {
        Some(token) => match token.token_type {
            TokenType::Ret => stmt = parse_return_stmt(token_iter),
            _ => panic!(
                "Expected the start of a statement, got {:?} instead.",
                token.token_type
            ),
        },
        None => panic!("Unexpected end of file."),
    }
    parse_symbol(token_iter, TokenType::Semi);
    stmt
}

fn parse_return_stmt(token_iter: &mut Peekable<Iter<Token>>) -> NodeStmt {
    parse_symbol(token_iter, TokenType::Ret);
    let expr = parse_expr(token_iter);
    NodeStmt::Return(expr)
}

fn parse_expr(token_iter: &mut Peekable<Iter<Token>>) -> NodeExpr {
    match token_iter.peek() {
        Some(token) => match token.token_type {
            TokenType::IntLit => parse_literal_expression(token_iter),
            TokenType::Id => parse_ident_expression(token_iter),
            _ => panic!(
                "Expected the start of an expression, got {:?} instead.",
                token.token_type
            ),
        },
        None => panic!("Unexpected end of file."),
    }
}

fn parse_literal_expression(token_iter: &mut Peekable<Iter<Token>>) -> NodeExpr {
    let token = token_iter.next().unwrap();
    match token.token_type {
        TokenType::IntLit => NodeExpr::Literal(NodeLiteral::IntLit(token.value.parse().unwrap())),
        _ => panic!(
            "Expected an integer literal, got {:?} instead.",
            token.token_type
        ),
    }
}

fn parse_ident_expression(token_iter: &mut Peekable<Iter<Token>>) -> NodeExpr {
    let ident = parse_ident(token_iter);
    NodeExpr::Ident(ident)
}
