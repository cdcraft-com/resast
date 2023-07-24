#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;

pub mod decl;
pub mod expr;
pub mod pat;
#[cfg(feature = "esprima")]
pub mod serde;
pub mod stmt;

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct SourcePos {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct SourceSpan {
    pub start: SourcePos,
    pub in_map: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct Ident<'a> {
    pub name: Cow<'a, str>,
    pub s_loc: SourceSpan,
}

impl<'a> Ident<'a> {
    pub fn new(s: String, s_loc: SourceSpan) -> Self {
        Ident {
            name: Cow::Owned(s),
            s_loc,
        }
    }

    pub fn from_with_span(s: &'a str, s_loc: SourceSpan) -> Self {
        Ident {
            name: Cow::Borrowed(s),
            s_loc
        }
    }

    pub fn from(s: &'a str) -> Self {
        Ident {
            name: Cow::Borrowed(s),
            s_loc: SourceSpan::default(),
        }
    }

    pub fn from_with_pos(s: &'a str, line: u32, column: u32) -> Self {
        Ident {
            name: Cow::Borrowed(s),
            s_loc: SourceSpan { start: SourcePos { line: line - 1, col: column - 1 }, in_map: true },
        }
    }
}

/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum Program<'a> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<'a>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<'a>>),
}

impl<'a> Program<'a> {
    pub fn module(parts: Vec<ProgramPart<'a>>) -> Self {
        Program::Mod(parts)
    }
    pub fn script(parts: Vec<ProgramPart<'a>>) -> Self {
        Program::Script(parts)
    }
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ProgramPart<'a> {
    /// A Directive like `'use strict';`
    Dir(Dir<'a>),
    /// A variable, function or module declaration
    Decl(Decl<'a>),
    /// Any other kind of statement
    Stmt(Stmt<'a>),
}

impl<'a> ProgramPart<'a> {
    pub fn decl(inner: Decl<'a>) -> Self {
        ProgramPart::Decl(inner)
    }
    pub fn stmt(inner: Stmt<'a>) -> Self {
        ProgramPart::Stmt(inner)
    }
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct Dir<'a> {
    pub expr: Lit<'a>,
    pub dir: Cow<'a, str>,
}

/// A function, this will be part of either a function
/// declaration (ID is required) or a function expression
/// (ID is optional)
/// ```js
/// //function declaration
/// function thing() {}
/// //function expressions
/// var x = function() {}
/// let y = function q() {}
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct Func<'a> {
    pub id: Option<Ident<'a>>,
    pub params: Vec<FuncArg<'a>>,
    pub body: FuncBody<'a>,
    pub generator: bool,
    pub is_async: bool,
}

impl<'a> Func<'a> {
    pub fn new(
        id: Option<Ident<'a>>,
        params: Vec<FuncArg<'a>>,
        body: FuncBody<'a>,
        generator: bool,
        is_async: bool,
    ) -> Self {
        Func {
            id,
            params,
            body,
            generator,
            is_async,
        }
    }
}

/// A single function argument from a function signature
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum FuncArg<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

impl<'a> FuncArg<'a> {
    pub fn expr(expr: Expr) -> FuncArg {
        FuncArg::Expr(expr)
    }
    pub fn pat(pat: Pat) -> FuncArg {
        FuncArg::Pat(pat)
    }
}

/// The block statement that makes up the function's body
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct FuncBody<'a>(pub Vec<ProgramPart<'a>>);
/// A way to declare object templates
/// ```js
/// class Thing {
///     constructor() {
///         this._a = 0;
///     }
///     stuff() {
///         return 'stuff'
///     }
///     set a(value) {
///         if (value > 100) {
///             this._a = 0;
///         } else {
///             this._a = value;
///         }
///     }
///     get a() {
///         return this._a;
///     }
/// }
/// let y = class {
///     constructor() {
///         this.a = 100;
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct Class<'a> {
    pub id: Option<Ident<'a>>,
    pub super_class: Option<Box<Expr<'a>>>,
    pub body: ClassBody<'a>,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct ClassBody<'a>(pub Vec<Prop<'a>>);

impl<'a> Class<'a> {
    pub fn new(
        id: Option<Ident<'a>>,
        super_class: Option<Expr<'a>>,
        body: Vec<Prop<'a>>,
    ) -> Class<'a> {
        Class {
            id,
            super_class: super_class.map(Box::new),
            body: ClassBody(body),
        }
    }
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(
    all(feature = "serde", feature = "esprima"),
    serde(rename_all = "camelCase", untagged)
)]
pub enum VarKind {
    Var,
    Let,
    Const,
}

/// The available operators for assignment Exprs
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum AssignOp {
    Equal,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivEqual,
    ModEqual,
    LeftShiftEqual,
    RightShiftEqual,
    UnsignedRightShiftEqual,
    OrEqual,
    XOrEqual,
    AndEqual,
    PowerOfEqual,
}

/// The available logical operators
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum LogicalOp {
    Or,
    And,
}

/// The available operations for `Binary` Exprs
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum BinaryOp {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Plus,
    Minus,
    Times,
    Over,
    Mod,
    Or,
    XOr,
    And,
    In,
    InstanceOf,
    PowerOf,
}

/// `++` or `--`
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum UpdateOp {
    Increment,
    Decrement,
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum UnaryOp {
    Minus,
    Plus,
    Not,
    Tilde,
    TypeOf,
    Void,
    Delete,
}

/// A flag for determining what kind of property
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum PropKind {
    /// A property with a value
    Init,
    /// A method with the get keyword
    Get,
    /// A method with the set keyword
    Set,
    /// A constructor
    Ctor,
    /// A standard method
    Method,
}

pub mod prelude {
    pub use crate::decl::{
        Decl, DefaultExportDecl, ExportSpecifier, ImportSpecifier, ModDecl, ModExport, ModImport,
        NamedExportDecl, NormalImportSpec, VarDecl,
    };
    pub use crate::expr::{
        ArrayExpr, ArrowFuncBody, ArrowFuncExpr, AssignExpr, AssignLeft, BinaryExpr, CallExpr,
        ConditionalExpr, Expr, Lit, LogicalExpr, MemberExpr, MetaProp, NewExpr, ObjExpr, ObjProp,
        Prop, PropKey, PropValue, RegEx, StringLit, TaggedTemplateExpr, TemplateElement,
        TemplateLit, UnaryExpr, UpdateExpr, YieldExpr,
    };
    pub use crate::pat::{ArrayPatPart, AssignPat, ObjPat, ObjPatPart, Pat};
    pub use crate::stmt::{
        BlockStmt, CatchClause, DoWhileStmt, ForInStmt, ForOfStmt, ForStmt, IfStmt, LabeledStmt,
        LoopInit, LoopLeft, Stmt, SwitchCase, SwitchStmt, TryStmt, WhileStmt, WithStmt,
    };
    pub use crate::{
        AssignOp, BinaryOp, Class, ClassBody, Dir, Func, FuncArg, FuncBody, Ident, LogicalOp,
        Program, ProgramPart, PropKind, SourceSpan, UnaryOp, UpdateOp, VarKind,
    };
}
