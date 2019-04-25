
#[macro_use]
extern crate clap;
use clap::{Arg, App};

pub mod lexer;
pub mod parser;
pub mod repl;

arg_enum!{
    enum Mode{
        Repl,
        Make 
    }
}

fn main(){

    //Convert mode options to lowercase
    let values = Mode::variants().iter().map(|c| c.to_lowercase()).collect::<Vec<_>>();
    let values: Vec<&str> = values.iter().map(String::as_ref).collect();

    let matches = App::new("lambda")
                    .version("0.0.1")
                    .author("Zach W. <zach@hayzak.com>")
                    .about("Lambda Calculus Implementation")
                    .arg(Arg::with_name("MODE")
                            .help("What mode to run the program in")
                            .index(1)
                            .possible_values(&values)
                            .required(true))
                    .get_matches();

    match value_t!(matches.value_of("MODE"), Mode).unwrap(){
        Mode::Repl => repl::start(),
        Mode::Make => println!("chose make mode"),
    }
}
