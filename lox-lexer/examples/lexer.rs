use lox_lexer::*;

fn main() {
    let source = "var language=\n\"lox\";";
    Lexer::iter(source).for_each(|c| println!("{:?}", c));
}
