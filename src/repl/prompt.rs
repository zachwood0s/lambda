use dialoguer::{theme::CustomPromptCharacterTheme, Input};
pub enum PromptResult {
  Command(String, Option<String>),
  Input(String),
  InvalidCommand(String)
}

pub struct Prompt<'a>{
  options: Vec<PromptOption<'a>>
}

impl<'a> Prompt<'a>{
  pub fn new() -> Self {
    Prompt{options: vec!()}
  }

  pub fn option(mut self, option: PromptOption<'a>) -> Self {
    self.options.push(option);
    self
  }

  pub fn show(&self) -> PromptResult {
    let theme = CustomPromptCharacterTheme::new('>');
    let input: String = Input::with_theme(&theme)
      .interact()
      .unwrap();

    let input = input.trim().to_string();

    if input.starts_with(":") {
      self.handle_command(input)
    } else {
      PromptResult::Input(input)
    }
  }

  pub fn show_help(&self){

  }

  fn handle_command(&self, input: String) -> PromptResult {
    let command_parts = &input[1..].split(" ").collect::<Vec<_>>();
    let command_string = command_parts[0].to_string();

    match &*command_string {
      "h" | "help" => PromptResult::Command("HELP".to_string(), None),
      _ => {
        let option = self.options.iter().find(
          |x| x.name == command_string || x.short_name == Some(&command_string));
        match option {
          Some(command) => PromptResult::Command(command.name.to_uppercase(), Some(command_parts[1..].join(" "))),
          None => self.handle_invalid(command_string)
        }
      }
    }
  }


  fn handle_invalid(&self, command: String) -> PromptResult{

    PromptResult::InvalidCommand(command)
  }
}

pub struct PromptOption<'a> {
  name: &'a str,
  help: Option<&'a str>,
  short_name: Option<&'a str>,
}

impl<'a> PromptOption<'a> {
  pub fn with_name(name: &str) -> PromptOption{
    PromptOption { name, help: None, short_name: None}
  }

  pub fn help(mut self, help: &'a str) -> Self{
    self.help = Some(help);
    self
  }

  pub fn short(mut self, short_name: &'a str) -> Self{
    self.short_name = Some(short_name);
    self
  }
}
