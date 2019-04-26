use std::io;
use std::io::Write;
use lexer::Lexer;
use parser::Parser;
use parser::Visitor;
use parser::ParseNode;
use parser::GrammarItem;

static INDENT_AMOUNT : i32 = 4;

struct PrintVisitor{
    current_indent: i32
}

impl PrintVisitor {
    fn print_indent(&self){
        print!("{}", " ".repeat((self.current_indent * INDENT_AMOUNT) as usize));
    }
}

impl Visitor<()> for PrintVisitor{
    fn visit(&mut self, n: &ParseNode){

    }

    fn visit_literal_int(&mut self, i: &ParseNode){
        if let GrammarItem::LiteralInt(val) = i.entry {
            self.print_indent();
            println!("Literal Int: {}", val);
        }
    }

    fn visit_variable(&mut self, i: &ParseNode){
        if let GrammarItem::Variable(ref val) = i.entry {
            self.print_indent();
            println!("Variable: {}", val);
        }
    }

    fn visit_abstraction(&mut self, i: &ParseNode){
        if let GrammarItem::Abstraction(ref name, ref body) = i.entry {
            self.print_indent();
            println!("Abstraction: {}", name);
            println!("-Body:");
            self.current_indent+=1;
        }
    }

    fn visit_application(&mut self, i: &ParseNode){
    }

    fn visit_assignment(&mut self, i: &ParseNode){
    }

    fn visit_program(&mut self, i: &ParseNode){
    }
}

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

