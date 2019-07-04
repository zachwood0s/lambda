use parser::Visitor;
use parser::ParseNode;
use parser::GrammarItem;

use colored::*;

static INDENT_AMOUNT : i32 = 2;

pub struct PrintVisitor{
  current_indent: i32
}

impl PrintVisitor {
  pub fn new() -> PrintVisitor{
    PrintVisitor{current_indent: 0}
  }
  fn print_indent(&self){
    print!("{}", " ".repeat((self.current_indent * INDENT_AMOUNT) as usize));
  }
}

impl Visitor<()> for PrintVisitor{
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
}
