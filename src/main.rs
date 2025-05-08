#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if let Some((command, args)) = input.split_once(" "){
            match command {
                "exit" => {
                    println!("{}", args);
                    if args == "O" {
                        return;
                    }
                    else {
                        println!("{}: command not found", input);
                    }
                },
                "echo" => {
                    println!("{}", args);
                },
                _ => println!("{}: command not found", input)
            }
        }
        else {
            println!("{}: command not found", input)
        }
           
    }

}
