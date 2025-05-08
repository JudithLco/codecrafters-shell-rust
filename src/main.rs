#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
    
        stdin.read_line(&mut input).unwrap();
        input = input.trim();
        
        if (input == "exit 0"){
            return;
        } 

        println!("{}: command not found", input)
    }

}
