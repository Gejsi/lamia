pub mod lexer;

fn main() {
    println!("{:?}", lexer::lex_operator(">="));
}
