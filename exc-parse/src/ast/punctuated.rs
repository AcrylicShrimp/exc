use crate::Token;

pub const PUNCUATION_KIND_COMMA: u8 = PunctuationKind::Comma as u8; // ,
pub const PUNCUATION_KIND_PATH_SEP: u8 = PunctuationKind::PathSep as u8; // ::

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PunctuationKind {
    Comma,
    PathSep,
}

#[derive(Debug, Clone, Hash)]
pub struct Punctuated<T: std::fmt::Debug + Clone + std::hash::Hash, const KIND: u8> {
    pub items: Vec<PunctuatedItem<T>>,
}

#[derive(Debug, Clone, Hash)]
pub enum PunctuatedItem<T: std::fmt::Debug + Clone + std::hash::Hash> {
    Punctuated { item: T, punctuation: Token },
    NotPunctuated { item: T },
}

impl<T: std::fmt::Debug + Clone + std::hash::Hash> PunctuatedItem<T> {
    pub fn into_item(self) -> T {
        match self {
            PunctuatedItem::Punctuated { item, .. } => item,
            PunctuatedItem::NotPunctuated { item } => item,
        }
    }
}
