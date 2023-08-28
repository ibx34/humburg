#![feature(let_chains)]

use common::{HumburgCursor, Keywords, LexResult};
use parser::HumburgParser;
use std::{fmt::Debug, fs::read_to_string};

#[derive(Debug)]
struct HumburgLexer<A>
where
    A: Iterator<Item = char> + Debug,
{
    pub cursor: HumburgCursor<A, char>,
    pub res: Vec<LexResult>,
}

impl<A> HumburgLexer<A>
where
    A: Iterator<Item = char> + Debug,
{
    pub fn push_back(&mut self, l_res: LexResult, should_advance: bool) {
        self.res.push(l_res);
        if should_advance {
            self.cursor.advance();
        }
    }

    pub fn lex_all(&mut self) {
        while let Some(next) = self.cursor.peek() {
            let next = *next;
            if let Some(lr) = self.lex(next) {
                // Don't advance after an identifier or keyword (they come from the same place)
                // because like it advances to the char that isnt allowed in an identifier so we
                // can just not advance because of that.
                let should_advance = match lr {
                    LexResult::Identifier(_) | LexResult::Keyword(_) => false,
                    _ => true,
                };
                self.push_back(lr, should_advance);
            }
        }
    }

    pub fn lex(&mut self, c: char) -> Option<LexResult> {
        Some(match c {
            ' ' => LexResult::Space,
            '\n' => {
                self.cursor.advance();
                return None;
            }
            '#' => {
                self.cursor.advance();
                while let Some(n) = self.cursor.peek() {
                    if *n == '\n' {
                        self.cursor.advance();
                        break;
                    }
                    self.cursor.advance();
                }
                return None;
            }
            '@' => LexResult::AtSign,
            '>' => LexResult::GreatherThan,
            '<' => LexResult::LessThan,
            '-' => LexResult::Dash,
            ']' => LexResult::CloseSquare,
            '[' => LexResult::OpenSquare,
            '!' => LexResult::Bang,
            '=' => LexResult::Eq,
            '+' => LexResult::Plus,
            ':' => LexResult::Colon,
            '(' => LexResult::OpenParen,
            ')' => LexResult::CloseParen,
            ',' => LexResult::Comma,
            _ => {
                let mut identifier = String::new();
                while let Some(next) = self.cursor.peek() {
                    if !next.is_alphanumeric() {
                        break;
                    }
                    identifier.push(*next);
                    self.cursor.advance();
                }
                if identifier.len() <= 0 {
                    panic!("Invalid identifier.")
                }
                if let Some(keyword) = Keywords::try_from_option(&identifier) {
                    return Some(LexResult::Keyword(keyword));
                }
                LexResult::Identifier(identifier)
            }
        })
    }
}

fn main() {
    // let test_files = ["simple", "little_more_some_keywords"];
    // for test_file in test_files {
    //     let sample_file = read_to_string(format!("tests/1_lex/{test_file}")).unwrap();
    //     let cursor = HumburgCursor::new(sample_file.chars());
    //     let mut lexer = HumburgLexer {
    //         cursor,
    //         res: Vec::new(),
    //     };
    //     lexer.lex_all();
    //     println!("{lexer:#?}");
    // }
    let sample_file = read_to_string(format!("tests/2_parse/simple_lambda")).unwrap();
    let cursor = HumburgCursor::new(sample_file.chars());
    let mut lexer = HumburgLexer {
        cursor,
        res: Vec::new(),
    };
    lexer.lex_all();

    let cursor = HumburgCursor::new(lexer.res.into_iter());
    let mut parser = HumburgParser {
        cursor,
        res: Vec::new(),
    };
    println!("{:?}", parser.parse_expr(None));
}
