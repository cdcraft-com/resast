use crate::expr::{Expr, Prop};
use crate::{Ident, SourceSpan};
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum Pat<'a> {
    Ident(Ident<'a>),
    Obj(ObjPat<'a>),
    Array(Vec<Option<ArrayPatPart<'a>>>),
    RestElement(Box<Pat<'a>>),
    Assign(AssignPat<'a>),
}

impl<'a> Pat<'a> {
    pub fn ident_from(s: &'a str) -> Self {
        Pat::Ident(Ident::from(s))
    }

    pub fn ident_from_with_span(s: &'a str, s_loc: SourceSpan) -> Self {
        Pat::Ident(Ident::from_with_span(s, s_loc))
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ArrayPatPart<'a> {
    Pat(Pat<'a>),
    Expr(Expr<'a>),
}

/// similar to an `ObjectExpr`
pub type ObjPat<'a> = Vec<ObjPatPart<'a>>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ObjPatPart<'a> {
    Assign(Prop<'a>),
    Rest(Box<Pat<'a>>),
}

/// An assignment as a pattern
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct AssignPat<'a> {
    pub left: Box<Pat<'a>>,
    pub right: Box<Expr<'a>>,
}
