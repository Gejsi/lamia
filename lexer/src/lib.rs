#![feature(result_flattening)]
use logos::{Lexer, Logos};

#[derive(Debug, PartialEq, Eq)]
pub enum Comment<'source> {
    Line(&'source str),
    // Block(&'source [BlockComment<'source>]),
}

// Waiting for: https://github.com/maciejhirsz/logos/issues/148
#[derive(Debug, Logos)]
#[logos(extras = i64)]
enum BlockComment {
    #[regex(r"/\*", |l| { l.extras += 1 })]
    Open,

    #[regex(r"\*/", block_comment_close)]
    Close,

    #[regex(r".", |l| {
        l.extras == -1 
    })]
    Any,
}

fn block_comment_close(lex: &mut Lexer<BlockComment>) -> Result<(), ()> {
    lex.extras -= 1;
    if lex.extras < 0 {
        Err(())
    } else {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,

    Plus,
    Minus,
    Bang,
    Star,
    Slash,
    Modulo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Punctuation {
    Comma,
    Semicolon,
    Colon,
    RightArrow,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Number<'source> {
    Integer(&'source str),
    HexInteger(&'source str),
    OctalInteger(&'source str),
    BinaryInteger(&'source str),

    // NOTE: Not sure we want to keep the sign in the float regexp, other
    // numbers (in particular, the integer) don't have it.
    Float(&'source str),
    HexFloat(&'source str),
}

// many patters are from https://github.com/maciejhirsz/logos/issues/133
#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(subpattern ident = r"[\p{XID_Start}_]\p{XID_Continue}*")]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
#[logos(subpattern hex = r"[0-9a-fA-F][_0-9a-fA-F]*")]
#[logos(subpattern octal = r"[0-7][_0-7]*")]
#[logos(subpattern binary = r"[0-1][_0-1]*")]
#[logos(subpattern exp = r"[eE][+-]?[0-9][_0-9]*")]
pub enum Token<'source> {
    #[regex("//[^\n]*\n?", |c| Comment::Line(c.slice()))]
    LineComment(Comment<'source>),

    #[regex(r"/\*", |lex| {
        let mut lexer = BlockComment::lexer(lex.remainder());
        let token = lexer.next();
        println!("{:?}", token);
        token.ok_or(()).flatten().map(|_| lexer.slice())
    })]
    BlockComment(&'source str),

    #[token(",", |_| Punctuation::Comma)]
    #[token(";", |_| Punctuation::Semicolon)]
    #[token(":", |_| Punctuation::Colon)]
    #[token("->", |_| Punctuation::RightArrow)]
    Punctuation(Punctuation),

    #[regex("(?&ident)")]
    Identifier(&'source str),

    // TODO: probably more escapes are needed
    #[regex(r"'(?:[^']|\\')*'")]
    Character(&'source str),
    #[regex("\"(?:[^\"]|\\\")*\"")]
    String(&'source str),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[regex("(?&decimal)", |n| Number::Integer(n.slice()))]
    #[regex("0[xX](?&hex)", |n| Number::HexInteger(n.slice()))]
    #[regex("0[oO](?&octal)", |n| Number::OctalInteger(n.slice()))]
    #[regex("0[bB](?&binary)", |n| Number::BinaryInteger(n.slice()))]
    // NOTE: Not sure we want to keep the sign in the float regexp, other
    // numbers (in particular, the integer) don't have it.
    #[regex(r#"[+-]?(((?&decimal)\.(?&decimal)?(?&exp)?[fFdD]?)|(\.(?&decimal)(?&exp)?[fFdD]?)|((?&decimal)(?&exp)[fFdD]?)|((?&decimal)(?&exp)?[fFdD]))"#, |n| Number::Float(n.slice()))]
    #[regex(r"0[xX](((?&hex))|((?&hex)\.)|((?&hex)?\.(?&hex)))[pP][+-]?(?&decimal)[fFdD]?", |n| Number::HexFloat(n.slice()))]
    Number(Number<'source>),

    #[token("==", |_| Operator::Equal)]
    #[token("!=", |_| Operator::NotEqual)]
    #[token("<", |_| Operator::LessThan)]
    #[token(">", |_| Operator::GreaterThan)]
    #[token("<=", |_| Operator::LessThanEqual)]
    #[token(">=", |_| Operator::GreaterThanEqual)]
    #[token("&&", |_| Operator::LogicalAnd)]
    #[token("||", |_| Operator::LogicalOr)]
    #[token("+", |_| Operator::Plus)]
    #[token("-", |_| Operator::Minus)]
    #[token("!", |_| Operator::Bang)]
    #[token("*", |_| Operator::Star)]
    #[token("/", |_| Operator::Slash)]
    #[token("%", |_| Operator::Modulo)]
    Operator(Operator),
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! ok_first_token {
        ($src: expr, $expect: expr) => {
            let mut lexer = Token::lexer($src);
            assert_eq!(lexer.next(), Some(Ok($expect)));
        };
    }

    macro_rules! err_first_token {
        ($src: expr, $expect: expr) => {
            let mut lexer = Token::lexer($src);
            assert_eq!(lexer.next(), Some(Err($expect)));
        };
    }

    #[test]
    fn match_comment() {
        ok_first_token!(
            "// test comment",
            Token::LineComment(Comment::Line("// test comment"))
        );
        // new line should be captures
        ok_first_token!(
            "// test comment\n",
            Token::LineComment(Comment::Line("// test comment\n"))
        );
        // ok_first_token!(
        //     "/* block comment */",
        //     Token::Comment(Comment::Block("/* block comment */"))
        // );
        // TODO: nested block comments should be allowed
        // ok_first_token!(
        //     "/* block /* comment */ */",
        //     Token::BlockComment("/* block /* comment */ */")
        // );
        err_first_token!("/* block comment */ */", ());
    }

    #[test]
    fn match_punctuation() {
        ok_first_token!(",", Token::Punctuation(Punctuation::Comma));
        ok_first_token!(";", Token::Punctuation(Punctuation::Semicolon));
        ok_first_token!(":", Token::Punctuation(Punctuation::Colon));
        ok_first_token!("->", Token::Punctuation(Punctuation::RightArrow));
    }

    #[test]
    fn match_identifier() {
        ok_first_token!("varname", Token::Identifier("varname"));
        ok_first_token!("var_name", Token::Identifier("var_name"));
        ok_first_token!("var_name1", Token::Identifier("var_name1"));
        ok_first_token!("_var_name", Token::Identifier("_var_name"));
        // TODO: should fail, but instead the 1 gets lexed into a number
        // err_first_token!("1var_name", ());
    }

    #[test]
    fn match_char() {
        ok_first_token!(r#"'\n'"#, Token::Character("'\\n'"));
        ok_first_token!(r#"'\r'"#, Token::Character("'\\r'"));
        ok_first_token!(r#"'\t'"#, Token::Character("'\\t'"));
        ok_first_token!(r#"'\\'"#, Token::Character("'\\\\'"));
        ok_first_token!(r#"'/'"#, Token::Character("'/'"));
        ok_first_token!(r#"'"'"#, Token::Character("'\"'"));
        ok_first_token!(r#"' '"#, Token::Character("' '"));
        ok_first_token!(r#"'\''"#, Token::Character("'\\''"));
        ok_first_token!("'\\u{1F600}'", Token::Character("'\\u{1F600}'"));
        ok_first_token!("'😀'", Token::Character("'😀'"));
        ok_first_token!("'東'", Token::Character("'東'"));
        ok_first_token!("'д'", Token::Character("'д'"));
        ok_first_token!("'ل'", Token::Character("'ل'"));
        // TODO: these should fail
        // err_first_token!("'ab'", ());
        // err_first_token!("'  '", ());
        // err_first_token!("'東京'", ());
        // err_first_token!("'''", ());
    }

    #[test]
    fn match_integer() {
        ok_first_token!("1", Token::Number(Number::Integer("1")));
    }
}
