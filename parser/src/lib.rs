pub mod syntax;

use lexer::Lexer;
use rowan::{GreenNode, GreenNodeBuilder};

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub builder: GreenNodeBuilder<'static>,
    pub errors: Vec<String>, // TODO: create custom error
}

struct GreenTree {
    green_node: GreenNode,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
            errors: vec![],
        }
    }

    pub fn parse(&mut self) {
        todo!()
    }
}
