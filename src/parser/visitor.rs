use parser::ParseNode;
use parser::GrammarItem;

pub trait Visitor<T> {
  fn visit(&mut self, n: &ParseNode) -> T{
    match n.entry {
      GrammarItem::LiteralInt(_) => self.visit_literal_int(n),
      GrammarItem::Variable(_) => self.visit_variable(n),
      GrammarItem::Abstraction(_,_) => self.visit_abstraction(n),
      GrammarItem::Application(_,_) => self.visit_application(n),
      GrammarItem::Assignment(_,_) => self.visit_assignment(n),
      GrammarItem::Program(_) => self.visit_program(n),
    }
  }
  fn visit_program(&mut self, a: &ParseNode) -> T;
  fn visit_abstraction(&mut self, a: &ParseNode) -> T;
  fn visit_application(&mut self, a: &ParseNode) -> T;
  fn visit_assignment(&mut self, a: &ParseNode) -> T;

  fn visit_literal_int(&mut self, i: &ParseNode) -> T;

  fn visit_variable(&mut self, v: &ParseNode) -> T;
}
