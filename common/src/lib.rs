#![allow(dead_code)]
use std::{fmt::Debug, iter::Peekable};

#[derive(Debug)]
pub struct HumburgCursor<A, B>
where
    A: Iterator<Item = B>,
    B: Clone + Debug,
{
    pub iter: Peekable<A>,
    pub cc: Option<B>,
    // Idk it might be nice to know?
    pub prev: Option<B>,
}

impl<A, B> HumburgCursor<A, B>
where
    A: Iterator<Item = B>,
    B: Clone + Debug,
{
    pub fn new(iter: A) -> Self {
        Self {
            iter: iter.peekable(),
            cc: None,
            prev: None,
        }
    }

    // pub fn get_prev(&mut self) -> Option<char> {
    //     self.prev
    // }

    pub fn peek(&mut self) -> Option<&B> {
        self.iter.peek()
    }

    pub fn advance(&mut self) -> Option<B> {
        if let Some(prev) = &self.cc {
            self.prev = Some(prev.to_owned());
        }
        self.cc = Some(self.iter.next()?);
        self.cc.to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keywords {
    Fn,
}

impl Keywords {
    pub fn try_from_option(from: &str) -> Option<Keywords> {
        match from {
            "fn" => Some(Keywords::Fn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LexResult {
    Bang,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    Identifier(String),
    Keyword(Keywords),
    Comma,
    Space,
    Eq,
    Colon,
    Plus,
    Dash,
    GreatherThan,
    LessThan,
    AtSign,
}
