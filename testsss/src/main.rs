use hg_lex::Lexer;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("/home/alfredo/humburg/tests/lex_1.hg").unwrap();
    let mut lex = Lexer::new(input.chars()).unwrap();
    lex.lex_all();
    println!("{:?}", lex.results);
}
