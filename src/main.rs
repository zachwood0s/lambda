extern crate clap;
use clap::{Arg, App};

fn main(){
    let matches = App::new("lambda")
                    .version("0.0.1")
                    .author("Zach W. <zach@hayzak.com>")
                    .about("Lambda Calculus Implementation")
                    .arg(Arg::with_name("MODE")
                            .help("What mode to run the program in")
                            .index(1)
                            .possible_values(&["repl", "make"])
                            .required(true))
                    .get_matches();

    match matches.value_of("MODE").unwrap(){
        "repl" => println!("chose repl mode"),
        "make" => println!("chose make mode"),
        _ => unreachable!()
    }
}
