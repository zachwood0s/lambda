use dialoguer::{theme::ColorfulTheme, Checkboxes};

use parser::Parser;
use lexer::Lexer;
use parser::Visitor;
use super::printer::PrintVisitor;

use super::prompt::{PromptOption, Prompt, PromptResult};
use errors::error_index::Error::UnexpectedEOF;


#[derive(Default, Clone)]
struct Options{
  show_ast: bool,
  show_type_derivation: bool,
  emit_llvm_ir: bool
}

pub fn start(){
  main_loop();
}

fn main_loop() {
  let prompt = Prompt::new()
    .option(PromptOption::with_name("type")
      .short("t")
      .help("Displays the type of the expression provided "))
    .option(PromptOption::with_name("options")
      .short("o")
      .help("Allows you to choose various options for the REPL environment"))
    .option(PromptOption::with_name("quit")
      .short("q")
      .help("Exits the REPL environment"));

  let mut options = Options::default();
  options.show_ast = true;

  loop {
    match prompt.show() {
      PromptResult::Input(expr) => handle_expr(expr, &options),
      PromptResult::Command(ref c, _) if *c == "QUIT".to_string() => break,
      PromptResult::Command(c, rest) => handle_command(c, rest, &mut options),
      PromptResult::InvalidCommand(command) => ()
    }
  }
}

fn handle_expr(expr: String, options: &Options){
  let lexer = Lexer::new(expr.as_str());
  let mut parser = Parser::new(lexer);
  let mut printer = PrintVisitor::new();

  let result =
    parser.parse_toplevel_assignment()
    .or_else(|_| {
      parser.reset_lexer();
      parser.parse_expr()
    }).and_then(|res|{
      if parser.is_empty() { Result::Ok(res) }
      else { Result::Err(UnexpectedEOF) }
    });

  match result {
    Ok(ast) => {
      if options.show_ast {
        printer.visit(&ast)
      }
    },
    Err(e) => println!("Error parsing: {:?}",e)
  }
}

fn handle_command(command: String, rest: Option<String>, options: &mut Options){
  match &*command {
    "HELP" => println!("help"),
    "TYPE" => println!("type"),
    "OPTIONS" => show_options(options),
    _ => println!("Other")
  }
}

fn show_options(options: &mut Options){
  let checkboxes = &[
    ("Show AST", options.show_ast),
    ("Show type derivation", options.show_type_derivation),
    ("Emit LLVM Ir", options.emit_llvm_ir)
  ];

  *options = Options::default();

  let selections = Checkboxes::with_theme(&ColorfulTheme::default())
    .with_prompt("Options")
    .items_with_states(&checkboxes[..])
    .interact()
    .unwrap();

  {
    let handlers: &[Box<Fn(&mut Options)>] = &[
      Box::new(|ops| ops.show_ast = true),
      Box::new(|ops| ops.show_type_derivation = true),
      Box::new(|ops| ops.emit_llvm_ir = true)
    ];

    if !selections.is_empty() {
      for selection in selections {
        handlers[selection](options);
      }
    }
  }
}
