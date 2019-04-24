
#[derive(Debug, PartialEq)]
pub enum GrammarItem{
    LiteralInt(i32),
    Variable(String),
    Application(Box<ParseNode>, Box<ParseNode>),
    Abstraction(String, Box<ParseNode>),
    Assignment(String, Box<ParseNode>),
    Program(Vec<ParseNode>)
}

#[derive(Debug, PartialEq)]
pub enum Type{
    Variable(String),
    Abstraction(Box<Type>, Box<Type>),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct ParseNode{
    entry: GrammarItem,
    node_type: Type
}

impl ParseNode{
    pub fn new(grammar: GrammarItem, node_type: Type) -> ParseNode {
        ParseNode { entry: grammar, node_type: node_type }
    }
}
