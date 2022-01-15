#![allow(unused)]

use crate::tokenizer::*;

macro_rules! num_ast {
    ($a: expr) => {
        AST {
            num: Some($a),
            ..Default::default()
        }
    };
}

macro_rules! oper_ast {
    ($o: expr, $l: expr, $r: expr) => {
        AST {
            operator: Some($o),
            left_operand: Some(Box::new($l)),
            right_operand: Some(Box::new($r)),
            ..Default::default()
        }
    };
}

pub(crate) use {num_ast, oper_ast};

pub enum Operator {
    Add,
    Deduct,
    Multiply,
    Divide,
    Power,
}

#[derive(Default)]
pub struct AST {
    pub operator: Option<Operator>,
    pub left_operand: Option<Box<AST>>,
    pub right_operand: Option<Box<AST>>,
    pub num: Option<f64>,
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new_parser(inp_expr: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new_tokenizer(inp_expr),
        }
    }

    pub fn parse(&self) -> AST {
        todo!()
    }
}
