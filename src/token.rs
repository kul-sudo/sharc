use std::fmt::Formatter;

use crate::span::Span;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum TokenKind {
    EOF,
    NewLine,
    Identifier,

    Ret,

    FloatLiteral,

    BinaryIntLiteral,
    OctalIntLiteral,
    DecimalIntLiteral,
    HexadecimalIntLiteral,

    StringLiteral,
    CharLiteral,

    Tilde,
    Bang,
    At,
    Pound,
    Dollar,
    Percent,
    Caret,
    CaretCaret,
    Ampersand,
    AmpersandAmpersand,
    Star,
    LParen,
    RParen,
    Minus,
    Underscore,
    Equals,
    Plus,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Pipe,
    PipePipe,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Slash,
    Question,
    ArrowLeft,
    ArrowRight,
    FatArrowRight,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    MinusMinus,
    NotEquals,
    PlusPlus,
    EqualsEquals,
}

#[derive(Debug, Clone)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub span: Span,
    pub text: &'source str,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, {:?}", self.kind, self.span)?;
        if !self.text.is_empty() {
            write!(f, ", {:?}", self.text)?;
        };
        write!(f, ")")
    }
}

impl TokenKind {
    pub fn matching(self) -> Self {
        match self {
            Self::LBrace => Self::RBrace,
            Self::RBrace => Self::LBrace,
            Self::LBracket => Self::RBracket,
            Self::RBracket => Self::LBracket,
            Self::LParen => Self::RParen,
            Self::RParen => Self::LParen,
            a => a,
        }
    }
}
