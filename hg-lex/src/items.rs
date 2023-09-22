use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item<'a> {
    At,
    Eq,
    BackSlash,
    ForwardSlash,
    OpenParenthesis,
    CloseParenthesis,
    OpenSquare,
    CloseSquare,
    /// one in the same really
    Identifier(Cow<'a, str>),
    String(Cow<'a, str>),
}
