use parser::ParseNode;

pub trait Visitor<T> {
    fn visit(&mut self, n: &ParseNode) -> T;
    fn visit_literal_int(&mut self, i: &ParseNode) -> T;
    fn visit_variable(&mut self, v: &ParseNode) -> T;
    fn visit_application(&mut self, a: &ParseNode) -> T;
    fn visit_abstraction(&mut self, a: &ParseNode) -> T;
    fn visit_assignment(&mut self, a: &ParseNode) -> T;
    fn visit_program(&mut self, a: &ParseNode) -> T;
}
