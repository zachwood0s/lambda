
use std::io;
use std::io::Write;
use lexer::Lexer;
use parser::Parser;
use parser::Visitor;
use parser::ParseNode;
use parser::GrammarItem;

use colored::*;

static INDENT_AMOUNT : i32 = 2;

struct PrintVisitor{
    current_indent: i32
}

impl PrintVisitor {
  fn new() -> PrintVisitor{
    PrintVisitor{current_indent: 0}
  } 
  fn print_indent(&self){
      print!("{}", " ".repeat((self.current_indent * INDENT_AMOUNT) as usize));
  }
}

impl Visitor<()> for PrintVisitor{
  fn visit_literal_int(&mut self, i: &ParseNode){
    if let GrammarItem::LiteralInt(val) = i.entry {
      self.print_indent();
      println!("{} {}", "Literal Int:".green().underline(), val.to_string().red());
    }
  }

  fn visit_variable(&mut self, i: &ParseNode){
    if let GrammarItem::Variable(ref val) = i.entry {
      self.print_indent();
      println!("{} {}", "Variable:".green().underline(), val.cyan());
    }
  }

  fn visit_abstraction(&mut self, i: &ParseNode){
    if let GrammarItem::Abstraction(ref name, ref body) = i.entry {
      self.print_indent();
      println!("{}", "Abstraction:".green().underline());
      self.print_indent();
      println!("{} {}", "-Param:".bright_green().italic(), name.cyan());
      self.print_indent();
      println!("{}", "-Body:".bright_green().italic());
      self.current_indent += 1;
      self.visit(body);
      self.current_indent -= 1;
    }
  }

  fn visit_application(&mut self, i: &ParseNode){
    if let GrammarItem::Application(ref left, ref right) = i.entry {
      self.print_indent();
      println!("{}", "Application:".green().underline());
      self.print_indent();
      println!("{}", "-Left:".bright_green().italic());
      self.current_indent += 1;
      self.visit(left);
      self.current_indent -= 1;
      self.print_indent();
      println!("{}", "-Right:".bright_green().italic());
      self.current_indent += 1;
      self.visit(right);
      self.current_indent -= 1;

    }
  }

  fn visit_assignment(&mut self, i: &ParseNode){
    if let GrammarItem::Assignment(ref name, ref expr) = i.entry {
      self.print_indent();
      println!("{} {}", "Assignment:".green().underline(), name.cyan());
      self.print_indent();
      println!("{}", "-Value:".bright_green().italic());
      self.current_indent += 1;
      self.visit(expr);
      self.current_indent -= 1;
    }
  }

  fn visit_program(&mut self, i: &ParseNode){
    if let GrammarItem::Program(ref children) = i.entry {
      self.print_indent();
      println!("AST:");
      self.current_indent += 1;
      for child in children  {
        self.visit(&child);
      }
      self.current_indent -= 1;
    }
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
  let mut printer = PrintVisitor::new();

  let result = parser.parse()
      .or_else(|_| {
        parser.reset_lexer();
        parser.parse_expr()
      });

  match result {
    Ok(ast) => printer.visit(&ast),
    Err(e) => println!("Error parsing: {:?}",e)
  }
}

