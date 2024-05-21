pub mod syntax;

use lexer::Lexer;
use rowan::{GreenNode, GreenNodeBuilder};
use syntax::SyntaxKind;

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub builder: GreenNodeBuilder<'static>,
}

pub struct GreenTree {
    pub green_node: GreenNode,
    pub errors: Vec<String>, // TODO: create custom error
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> GreenTree {
        self.builder.start_node(SyntaxKind::Root.into());
        self.builder.finish_node();

        GreenTree {
            green_node: self.builder.finish(),
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{syntax::SyntaxNode, Parser};

    #[test]
    fn parse_nothing() {
        let green_tree = Parser::new("").parse();

        assert_eq!(
            format!("{:#?}", SyntaxNode::new_root(green_tree.green_node)),
            r#"Root@0..0
"#,
        );
    }
}
