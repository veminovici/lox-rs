use lox_lexer::*;

fn main() {
    let source = "var language=\n\"lox\";";
    Lexer::with_source(source).for_each(|c| println!("{:?}", c));
}
