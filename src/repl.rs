use std::io;
use std::io::Write;
use lexer::Lexer;
use parser::Parser;

pub fn start(){
  main_loop();
}

fn main_loop(){

  let mut input = String::new();

  print!("> ");
  io::stdout().flush().unwrap();
  input.clear();


  io::stdin().read_line(&mut input)
    .ok()
    .expect("Couldn't read line");

  println!("{}", input);
  let lexer = Lexer::new(input.as_str());
  let mut parser = Parser::new(lexer);

  match parser.parse(){
    Ok(ast) => println!("{:?}", ast),
    Err(e) => println!("Error parsing: {:?}",e)
  }

}

