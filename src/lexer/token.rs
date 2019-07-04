
#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    Illegal,
    EOF,

    // Literals 
    UIdent(String),
    LIdent(String),
    Integer(String),

    // Symbols
    Backslash,
    Dot,
    LParen,
    RParen,
    Colon,
    Assign
}
