use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item<'a> {
    String(Cow<'a, str>),
    AtSign,
}
