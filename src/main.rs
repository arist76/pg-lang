use std::collections::HashMap;
use std::io::{self, Write};
use std::env;
use std::fs;

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
                Err(_) => FileException::new().raise() 
            }
        }
    }
}

fn run_file(file_path : &String) -> () {
    let contents : String = match fs::read_to_string(file_path.clone()) {
        Ok(contents) => contents,
        Err(_) => format!("{:?}", format_args!("Error reading file: {}", file_path.clone()))
    };

    for line in contents.lines() {
        println!("{}", line);
    }

}

fn run_interactive(line_inp : &String) {
    print(line_inp);
}

//fn run() {
//}

fn print(msg : &String) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

trait Exception<T> {
    fn new() -> T;
    fn raise(&self) -> ();
}

struct FileException {
    msg : String,
    //context : Option<HashMap<String, String>>
}

impl Exception<FileException> for FileException {
    fn new() -> FileException {
        FileException {
            msg : String::new(),
            //context : None
        }
    }

    fn raise(&self) -> () {
        println!("Error: {}", self.msg);
    }
}

enum Command {
    Exit(T)
}

impl Command {
    fn execute(&self) -> () {
        match self {
            Command::Exit => Self::run_exit(),
        }
    }

    fn run_exit() -> () {
        print(&"Bye!".to_string());
    }

    fn parse_token -> () {

    }
}

