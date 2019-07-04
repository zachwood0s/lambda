use lexer::Lexer;
use lexer::Token;
use parser::ParseNode;
use parser::Type;
use parser::GrammarItem;
use errors::error_index::{Error};

pub type ParseError = Error;
pub type ParseResult = Result<ParseNode, ParseError>;

pub struct Parser<'a>{
    lexer: Lexer<'a>
}

impl<'a> Parser<'a>{
    pub fn new(input: Lexer<'a>) -> Parser<'a>{
        Parser { lexer: input }
    }

    pub fn reset_lexer(&mut self){
        self.lexer.reset();
    }

    pub fn parse(&mut self) -> ParseResult{
        let mut assignments: Vec<ParseNode> = Vec::new();
        while let Ok(assign) = self.parse_toplevel_assignment() {
            assignments.push(assign);
        }
        let tok = self.lexer.next_token();
        match tok {
            Token::EOF => Ok(ParseNode::new(
                GrammarItem::Program(assignments),
                Type::Unknown
            )),
            _ => Err(Error::ExpectedEOF(tok))
        }
        /*
        self.parse_toplevel_assignment().and_then(
            |x| match self.lexer.next_token() {
                Token::EOF => Ok(x),
                _ => Err(0)             //Expected EOF
            }
        )
        */
    }

    pub fn parse_toplevel_assignment(&mut self) -> ParseResult{
        let tok = self.lexer.next_token();
        match tok {
            Token::LIdent(id) => {
                self.consume(Token::Assign)?;
                self.parse_expr().and_then(
                    |expr| Ok(ParseNode::new(
                        GrammarItem::Assignment(id, Box::new(expr)),
                        Type::Unknown
                    )),
                )
            },
            _ => Err(Error::ExpectedToken(Token::LIdent("".to_string()), tok))
        }
    }

    pub fn parse_expr(&mut self) -> ParseResult{
        self.parse_base_expr().and_then(
            |expr| self.parse_expr_prime(expr)
        )
    }

    pub fn is_empty(&mut self) -> bool {
        self.lexer.is_empty()
    }

    fn parse_base_expr(&mut self) -> ParseResult{
        let tok = self.lexer.next_token();
        match tok {
            Token::LParen => self.parse_paren_expr(),
            Token::LIdent(id) => self.parse_identifier_expr(id),
            Token::Integer(s) => self.parse_literal_int(s),
            Token::Backslash => self.parse_abstraction_expr(),
            Token::Illegal => Err(Error::IllegalToken(tok)),
            Token::EOF => Err(Error::UnexpectedEOF),
            _ => {
                self.lexer.put_back(tok.clone());
                Err(Error::IllegalToken(tok))
            }
        }
    }

    fn parse_paren_expr(&mut self) -> ParseResult{
        self.parse_expr().and_then(
            |expr| {
                let tok = self.lexer.next_token();
                match tok {
                    Token::RParen => Ok(expr),
                    _ => Err(Error::ExpectedToken(Token::RParen, tok))
                }
            }
        )
    }

    fn parse_literal_int(&mut self, num_string: String) -> ParseResult{
        match num_string.parse() {
            Ok(num) => Ok(ParseNode::new(GrammarItem::LiteralInt(num), Type::Unknown)),
            Err(_) => Err(Error::IntegerParseError)
        }
    }

    fn parse_identifier_expr(&mut self, id: String) -> ParseResult{
        Ok(ParseNode::new(GrammarItem::Variable(id), Type::Unknown))
    }

    fn parse_abstraction_expr(&mut self) -> ParseResult{
        let tok = self.lexer.next_token();
        match tok {
            Token::LIdent(id) => {
                let t = self.parse_type()?;
                self.consume(Token::Dot)?;  
                self.parse_expr().and_then(
                    |expr| Ok(ParseNode::new(
                        GrammarItem::Abstraction(id, Box::new(expr)), t))
                )
            }
            _ => Err(Error::ExpectedToken(Token::LIdent("".to_string()), tok))     //Expected identifier
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        Ok(Type::Unknown)
    }

    fn parse_expr_prime(&mut self, left: ParseNode) -> ParseResult{
        let tok = self.lexer.next_token();
        match tok {
            Token::LParen | Token::Backslash | Token::LIdent(_) | Token::Integer(_) => {
                self.lexer.put_back(tok);
                self.parse_base_expr().and_then(
                    |expr| self.parse_expr_prime(ParseNode::new(
                        GrammarItem::Application(Box::new(left), Box::new(expr)),
                        Type::Unknown
                    ))
                )
            },
            _ => {
                self.lexer.put_back(tok);
                Ok(left)
            }
        }
    }

    fn consume(&mut self, tok : Token) -> Result<Token, ParseError> {
        let new_tok = self.lexer.next_token();
        if new_tok == tok {
            Ok(new_tok)
        } else {
            Err(Error::ExpectedToken(tok, new_tok))
        }
    }
}

/*
 * expr
 *  : ID expr'
 *  | (expr) expr'
 *  | \ID (: Type)? . expr expr'
 *
 * expr'
 *  : expr expr'
 *  | $
 */

#[test]
fn parse_variable(){
    let input = r#"a"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();

    assert_eq!(Ok(ParseNode::new(
        GrammarItem::Variable("a".to_string()),
        Type::Unknown
    )), node);
}

#[test]
fn parse_literal_int(){
    let input = r#"1234567890"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();

    assert_eq!(Ok(ParseNode::new(
        GrammarItem::LiteralInt(1234567890),
        Type::Unknown
    )), node);
}

#[test]
fn parse_expr_test_application(){
    let input = r#"a b c"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Ok(ParseNode::new(
        GrammarItem::Application(
            Box::new(ParseNode::new(
                GrammarItem::Application(
                    Box::new(ParseNode::new(
                        GrammarItem::Variable("a".to_string()),
                        Type::Unknown
                    )),
                    Box::new(ParseNode::new(
                        GrammarItem::Variable("b".to_string()),
                        Type::Unknown
                    )),
                ),
                Type::Unknown
            )),
            Box::new(ParseNode::new(
                GrammarItem::Variable("c".to_string()),
                Type::Unknown
            ))
        ),
        Type::Unknown
    )), node);
}

#[test]
fn parse_expr_test_abstraction(){
    let input = r#"\a. \b. a"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Ok(ParseNode::new(
        GrammarItem::Abstraction(
            "a".to_string(),
            Box::new(ParseNode::new(
                GrammarItem::Abstraction(
                    "b".to_string(),
                    Box::new(ParseNode::new(
                        GrammarItem::Variable("a".to_string()),
                        Type::Unknown
                    ))
                ),
                Type::Unknown
            ))
        ),
        Type::Unknown 
    )), node);
}

#[test]
fn parse_expr_test_paren(){
    let input = r#"a (b c)"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Ok(ParseNode::new(
        GrammarItem::Application(
            Box::new(ParseNode::new(
                GrammarItem::Variable("a".to_string()),
                Type::Unknown
            )),
            Box::new(ParseNode::new(
                GrammarItem::Application(
                    Box::new(ParseNode::new(
                        GrammarItem::Variable("b".to_string()),
                        Type::Unknown
                    )),
                    Box::new(ParseNode::new(
                        GrammarItem::Variable("c".to_string()),
                        Type::Unknown
                    ))
                ),
                Type::Unknown
            ))
        ),
        Type::Unknown
    )), node);
}

#[test]
fn parse_expr_test_error_2(){
    let input = r#"(a b"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Err(Error::ExpectedToken(Token::RParen, Token::EOF)), node);
}

#[test]
fn parse_expr_test_error_3(){
    let input = r#"\. a"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Err(Error::ExpectedToken(Token::LIdent("".to_string()), Token::Dot)), node);
}

#[test]
fn parse_expr_test_error_4(){
    let input = r#"\a( a"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Err(Error::ExpectedToken(Token::Dot, Token::LParen)), node);
}

#[test]
fn parse_expr_test_error_5(){
    let input = r#"$"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Err(Error::IllegalToken(Token::Illegal)), node);
}

#[test]
fn parse_expr_test_error_6(){
    let input = r#"\a. "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let node = parser.parse_expr();
    assert_eq!(Err(Error::UnexpectedEOF), node);
}
