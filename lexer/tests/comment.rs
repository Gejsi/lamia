use lexer::{err_first_token, ok_first_token, Comment, Token};

#[test]
fn match_line_comment() {
    ok_first_token!(
        "// test comment",
        Token::Comment(Comment::Line("// test comment"))
    );
    // new line should be captured
    ok_first_token!(
        "// test comment\n",
        Token::Comment(Comment::Line("// test comment\n"))
    );

    ok_first_token!("// ", Token::Comment(Comment::Line("// ")));
    ok_first_token!("//", Token::Comment(Comment::Line("//")));
}

#[test]
fn match_block_comment() {
    ok_first_token!("/*test*/", Token::Comment(Comment::Block("/*test*/")));
    ok_first_token!(
        "/* block comment */",
        Token::Comment(Comment::Block("/* block comment */"))
    );
}

#[test]
fn match_multiline_block_comment() {
    ok_first_token!(
        "/* Lorem\nipsum */",
        Token::Comment(Comment::Block("/* Lorem\nipsum */"))
    );
    ok_first_token!(
        "/* Line 1\nLine 2\nLine 3 */",
        Token::Comment(Comment::Block("/* Line 1\nLine 2\nLine 3 */"))
    );
}

#[test]
fn fail_missing_delimiter_block_comment() {
    err_first_token!("/* test", ());
}

#[test]
#[ignore] // TODO: support nested block comments
fn match_nested_block_comment() {
    ok_first_token!(
        "/*Nested /* Block */ Comment*/",
        Token::Comment(Comment::Block("/*Nested /* Block */ Comment*/"))
    );

    ok_first_token!(
        "/*Nested /* Block /* Lorem */ */ Comment*/",
        Token::Comment(Comment::Block("/*Nested /* Block /* Lorem */ */ Comment*/"))
    );

    ok_first_token!(
        "/*Nested /* Block */ /* Lorem */ Comment*/",
        Token::Comment(Comment::Block("/*Nested /* Block /* Lorem */ */ Comment*/"))
    );
}
