use super::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a>{
    input: Peekable<Chars<'a>>,
    cache: Vec<Token>
}

impl<'a> Lexer<'a>{
    pub fn new(input: &str) -> Lexer{
        Lexer {input: input.chars().peekable(), cache: Vec::new()}
    }

    pub fn put_back(&mut self, token: Token){
        self.cache.push(token);
    }

    pub fn next_token(&mut self) -> Token{
        if let Some(top) = self.cache.pop() {
            return top;
        }

        self.skip_whitespace();

        match self.read_char(){
            Some('=') => Token::Assign,
            Some('.') => Token::Dot,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('\\') => Token::Backslash,
            Some(':') => Token::Colon,

            Some(ch @ _) => {
                match ch {
                    'a'...'z' => Token::LIdent(self.read_identifier(ch)),
                    'A'...'Z' => Token::UIdent(self.read_identifier(ch)),
                    '0'...'9' => Token::Integer(self.read_number(ch)),
                    _         => Token::Illegal
                }
            }
            None => Token::EOF
        }
    }

    fn skip_whitespace(&mut self){
        while let Some(&c) = self.peek_char(){
            if !c.is_whitespace(){
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self, first: char) -> String{
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek_char(){
            if !c.is_numeric() {
                break;
            }
            if let Some(ch) = self.read_char() {
                number.push(ch)
            }
        }
        number
    }

    fn read_identifier(&mut self, first: char) -> String{
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter(){
            if let Some(c) = self.read_char(){
                ident.push(c)
            } else {
                break;
            }
        }
        ident
    }

    fn read_char(&mut self) -> Option<char>{
        self.input.next()
    } 

    fn peek_char(&mut self) -> Option<&char>{
        self.input.peek()
    }

    fn peek_is_letter(&mut self) -> bool{
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false
        }
    }
}

fn is_letter(ch: char) -> bool{
    ch.is_alphabetic() || ch == '_'
}


#[test]
fn is_letter_test(){
    assert!(is_letter('_'));
    assert!(is_letter('a'));
    assert!(is_letter('Z'));

    assert!(!is_letter('*'));
    assert!(!is_letter('1'));
}

#[test]
fn next_token_test(){
    let input = r#"fake_identifier: Int = \a. \b. (b c) 109   "#;

    let expected = vec![
        Token::LIdent("fake_identifier".to_string()),
        Token::Colon,
        Token::UIdent("Int".to_string()),
        Token::Assign,
        Token::Backslash,
        Token::LIdent("a".to_string()),
        Token::Dot,
        Token::Backslash,
        Token::LIdent("b".to_string()),
        Token::Dot,
        Token::LParen,
        Token::LIdent("b".to_string()),
        Token::LIdent("c".to_string()),
        Token::RParen,
        Token::Integer("109".to_string()),
        Token::EOF
    ];

    let mut lexer = Lexer::new(input);

    for e in expected{
        assert_eq!(lexer.next_token(), e)
    }
}

#[test]
fn next_token_illegal(){
    let input = "_test_";

    let expected = vec![
        Token::Illegal,
        Token::LIdent("test_".to_string()),
    ];

    let mut lexer = Lexer::new(input);

    for e in expected{
        assert_eq!(lexer.next_token(), e)
    }
}

#[test]
fn next_token_cache(){
    let input = "(a b)";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::LIdent("a".to_string()));
    assert_eq!(lexer.next_token(), Token::LIdent("b".to_string()));
    assert_eq!(lexer.next_token(), Token::RParen);
    lexer.put_back(Token::RParen);
    lexer.put_back(Token::LIdent("b".to_string()));
    assert_eq!(lexer.next_token(), Token::LIdent("b".to_string()));
    assert_eq!(lexer.next_token(), Token::RParen);

}

