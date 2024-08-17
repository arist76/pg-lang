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
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line_inp = String::new();
            match io::stdin().read_line(&mut line_inp) {
                Ok(_) => run_interactive(&line_inp),
                Err(_) => println!("Error reading line")
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
    print!("{}", line_inp);
    io::stdout().flush().unwrap();
}
//
//fn run() {
//}
