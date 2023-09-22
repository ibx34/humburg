use std::{fmt::Debug, iter::Peekable};

/// A generic struct that stores information on where a parser, lexer, etc.
/// is at on an input of type B
pub struct Indexer<A, B>
where
    A: Iterator<Item = B>,
    B: Clone + Debug,
{
    pub iter: Peekable<A>,
    pub current: B,
    /// this is zero-indexed xoxo, why? fuck you future me.
    pub idx: usize,
    pub prev: Option<B>,
}

impl<A, B> Indexer<A, B>
where
    A: Iterator<Item = B>,
    B: Clone + Debug,
{
    pub fn init(mut iter: Peekable<A>) -> Option<Self> {
        let first = iter.nth(0)?;
        Some(Self {
            iter,
            current: first,
            idx: 0,
            prev: None,
        })
    }

    pub fn advance(&mut self) -> Option<&B> {
        self.prev = Some(self.current.to_owned());
        let next = self.iter.next()?;
        self.current = next;
        self.idx += 1;
        return Some(&self.current);
    }

    pub fn peek(&mut self) -> Option<&B> {
        self.iter.peek()
    }
    pub fn peek_rev(&self) -> Option<&B> {
        self.prev.as_ref()
    }
    pub fn current(&self) -> &B {
        &self.current
    }
}
