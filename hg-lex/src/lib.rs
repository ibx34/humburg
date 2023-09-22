#![feature(let_chains)]
pub mod items;
use hg_common::Indexer;
use items::Item;
use std::borrow::Cow;
use std::fmt::Debug;

pub struct Lexer<'a, A>
where
    A: Iterator<Item = char> + Debug,
{
    pub indexer: Indexer<A, char>,
    pub results: Vec<Item<'a>>,
}

impl<'a, A> Lexer<'a, A>
where
    A: Iterator<Item = char> + Debug,
{
    pub fn new(input: A) -> Option<Self> {
        Some(Self {
            results: Vec::new(),
            indexer: Indexer::init(input.peekable())?,
        })
    }

    pub fn push_back(&mut self, to_push_back: Item<'a>, should_advance: bool) -> Option<&Item<'a>> {
        self.results.push(to_push_back);
        if should_advance {
            self.indexer.advance()?;
        }
        Some(self.results.last()?)
    }

    pub fn lex(&mut self, to_lex: &char) -> Option<&Item<'a>> {
        match to_lex {
            '/' => {
                if let Some(peeked) = self.indexer.advance() {
                    if peeked == &'/' {
                        while let Some(next) = self.indexer.advance() {
                            if next == &'\n' {
                                break;
                            }
                            self.indexer.advance();
                        }
                        return None;
                    } else if peeked == &'*' {
                        while let Some(next) = self.indexer.advance() {
                            if next == &'*' {
                                if let Some(n) = self.indexer.peek() {
                                    if n == &'/' {
                                        break;
                                    }
                                }
                            }
                            self.indexer.advance();
                        }
                        return None;
                    }
                }
                self.push_back(Item::ForwardSlash, true)
            }
            '\\' => self.push_back(Item::BackSlash, true),
            '@' => self.push_back(Item::At, true),
            ']' => self.push_back(Item::CloseSquare, true),
            '[' => self.push_back(Item::OpenSquare, true),
            '(' => self.push_back(Item::OpenParenthesis, true),
            ')' => self.push_back(Item::CloseParenthesis, true),
            a @ _ => {
                if a.is_ascii_alphanumeric() && a != &' ' {
                    self.indexer.advance();
                    let mut identifier = String::new();
                    identifier.push(*a);
                    while let Some(n) = self.indexer.peek() {
                        if (n == &' ' || !n.is_ascii_alphanumeric()) && n != &'_' {
                            break;
                        }
                        identifier.push(n.to_owned());
                        self.indexer.advance();
                    }
                    self.push_back(Item::Identifier(Cow::Owned(identifier)), false)
                } else {
                    self.indexer.advance();
                    None
                }
            }
        }
    }

    pub fn lex_all(&mut self) {
        while let Some(n) = self.indexer.peek() {
            let peeked = n.to_owned();
            self.lex(&peeked);
        }
    }
}
