use lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
  ExpectedEOF(Token),                   // Carries illegal token
  UnexpectedEOF,
  IntegerParseError,
  IllegalToken(Token),                  // Carries illegal token
  ExpectedToken(Token, Token)           // Carries expected, actual
}