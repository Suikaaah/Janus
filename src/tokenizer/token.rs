#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int,
    Stack,
    Procedure,
    If,
    Then,
    Else,
    Fi,
    From,
    Do,
    Loop,
    Until,
    Push,
    Pop,
    Local,
    Delocal,
    Call,
    Uncall,
    Skip,
    Empty,
    Top,
    Nil,
    Plus,
    Minus,
    Caret,
    Asterisk,
    Slash,
    Percent,
    Ampersand,
    VerticalBar,
    Ampersand2,
    VerticalBar2,
    Less,
    Greater,
    Equal,
    ExclamationEqual,
    LessEqual,
    GreaterEqual,
    PlusEqual,
    MinusEqual,
    CaretEqual,
    Identifier(String),
    LParen,
    RParen,
    LSquareBracket,
    RSquareBracket,
    Comma,
    Constant(i32),
}
