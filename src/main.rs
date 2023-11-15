use std::io::{stdin};




fn main(){
    while true{
        println!("ZenScript>")
        let mut input = String::new();
        stdin.read_line(&mut input);
        // Todo: Implement a Lexer which will scan and tokenize the input string.
        //lexer(input);
    }
}