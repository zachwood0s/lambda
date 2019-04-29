
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

enum PromptResult {
  Command(String, Option<String>),
  Input(String), 
  InvalidCommand
}

struct Prompt<'a>{
  options: Vec<PromptOption<'a>>
}

impl<'a> Prompt<'a>{
  fn new() -> Self {
    Prompt{options: vec!()}
  }

  fn option(mut self, option: PromptOption<'a>) -> Self {
    self.options.push(option);
    self
  }

  fn show(self) -> PromptResult {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input)
      .ok()
      .expect("Couldn't read line");

    let input = input.trim().to_string();

    if input.starts_with(":") {
      self.handle_command(input)
    } else {
      PromptResult::Input(input)
    }
  }

  fn handle_command(self, input: String) -> PromptResult {
    let command_parts = &input[1..].split(" ").collect::<Vec<_>>();
    let command_string = command_parts[0].to_string();

    if command_string == "h" || command_string == "help" {

      PromptResult::Command("HELP".to_string(), None)
    }
    else{
      let option = self.options.iter().find(
        |x| x.name == command_string || x.short_name == Some(&command_string));
      
      match option {
        Some(command) => PromptResult::Command(command.name.to_string(), Some(command_parts[1..].join(" "))),
        None => PromptResult::InvalidCommand
      }
    }

  }
}

struct PromptOption<'a> {
  name: &'a str,
  help: Option<&'a str>,
  short_name: Option<&'a str>,
}

impl<'a> PromptOption<'a> {
  fn with_name(name: &str) -> PromptOption{
    PromptOption { name: name, help: None, short_name: None}
  }

  fn help(mut self, help: &'a str) -> Self{
    self.help = Some(help);
    self
  }

  fn short(mut self, short_name: &'a str) -> Self{
    self.short_name = Some(short_name);
    self
  }
}

pub fn start(){
  main_loop();
}

fn main_loop(){

  let prompt = Prompt::new()
                .option(PromptOption::with_name("type")
                  .short("t")
                  .help("Displays the type of the expression provided "));

  let input = prompt.show();

  match input {
    PromptResult::Input(expr) => {
      let lexer = Lexer::new(expr.as_str());
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

    PromptResult::Command(c, rest) =>  
      match &*c {
        "h" => println!("help"),
        _ => println!("Other")
      }
    
    PromptResult::InvalidCommand => println!("invalid")
  }
}

