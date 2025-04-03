use core::str::Utf8Error;

pub type Result<T> = core::result::Result<T, BunyError>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BunyError {
    pub reason: BunyErrorKind,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BunyErrorKind {
    /// Error when typecasting a ValueRef to their types.
    ValueCast(ValueCastError),

    /// There was an error handling UTF-8 data.
    Utf8Error(Utf8Error),

    /// This feature is unimplemented.
    Unimplemented,

    /// Error when handling HTML content through [html::Node](crate::html::Node)
    NodeError(NodeError),

    /// JSON parsing error.
    JsonParseError,

    /// The defaults key doesn't have a value set.
    DefaultNotFound,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum NodeError {
    /// There was an error parsing HTML.
    ParseError,

    /// There was an error modifying HTML.
    ModifyError,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ValueCastError {
    NotArray,
    NotObject,
    NotString,
    NotInt,
    NotFloat,
    NotBool,
    NotNode,
}

impl From<ValueCastError> for BunyError {
    fn from(why: ValueCastError) -> Self {
        Self {
            reason: BunyErrorKind::ValueCast(why),
        }
    }
}

impl From<Utf8Error> for BunyError {
    fn from(why: Utf8Error) -> Self {
        Self {
            reason: BunyErrorKind::Utf8Error(why),
        }
    }
}

impl From<NodeError> for BunyError {
    fn from(why: NodeError) -> Self {
        Self {
            reason: BunyErrorKind::NodeError(why),
        }
    }
}
