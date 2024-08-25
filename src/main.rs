use std::io::{self, Write};
use std::env;
use std::fs;

mod token;
mod scanner;
mod exceptions;

use exceptions::PrintException;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        println!("PG-LANG version 0.0.1");
        loop {
            print(&">>> ".to_string());
            let mut line_inp = String::new();
            match io::stdin().read_line(&mut line_inp) {
                Ok(_) => run_interactive(&line_inp),
                Err(_) =>  print(&"Error reading input".to_string()),
            }
        }
    }
}

fn run_file(file_path : &String) -> () {
    let contents : String = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(_) => {
            print(&"Error reading file".to_string());
            std::process::exit(1);
        }
    };

    for (i, line) in contents.lines().enumerate() {
        run(line.to_string(),i + 1);
    }
}

fn run_interactive(line_inp : &String) {
    print(line_inp);
}

fn run(source : String, line : usize) {
    let mut scanner = scanner::Scanner::new(source, line);
    scanner.start();

    for exception in &mut scanner.exceptions {
        exception.print_exception();
    }

    for token in &mut scanner.tokens {
        println!("Token {:?}", token);
    }
}

fn print(msg : &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

