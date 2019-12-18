extern crate clap; 
use clap::{Arg, App};
use std::process::Command;
 
fn main() { 
    let matches = App::new("myapp")
       .version("1.0")
       .about("Pass your first argument to the second one")
       .author("Bastien NISOLE")
       .arg(Arg::with_name("in")
            .short("in")
            .long("in")
            .help("Set the first argument")
            .takes_value(true)
            )
        .arg(Arg::with_name("out")
            .short("out")
            .long("out")
            .help("Set the second argument")
            .takes_value(true)
            )
        .get_matches();
    if matches.value_of("in") == None || matches.value_of("out") == None {
        println!("Argument empty");
        return;
    }
    let input_arg = matches.value_of("in").unwrap();
    let ouput_arg = matches.value_of("out").unwrap();
    let output_of_input = Command::new(input_arg.to_string())
                            .output()
                            .expect("failed to execute process");
    let hello = output_of_input.stdout;
    let output_of_output = Command::new(ouput_arg.to_string())
                            .arg(String::from_utf8_lossy(&hello).to_string())    
                            .output()
                            .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output_of_output.stdout));
}

//  sudo cargo build --target-dir ici
//  ./ici/debug/inandout --in fortune --out cowsay