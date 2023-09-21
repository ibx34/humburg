#![feature(let_chains)]
pub mod items;
use hg_common::Indexer;
use items::Item;
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

    pub fn lex(&mut self, to_lex: Option<&char>) -> Option<&Item<'a>> {
        let to_lex = if let Some(to_lex) = to_lex {
            *to_lex
        } else {
            *self.indexer.current()
        };
        match to_lex {
            '@' => self.push_back(Item::AtSign, false),
            _ => None,
        }
    }

    pub fn lex_all(&mut self) {
        while let Some(_) = self.lex(None) {
            self.indexer.advance();
        }
    }
}
