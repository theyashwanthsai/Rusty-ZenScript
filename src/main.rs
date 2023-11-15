use std::io::{stdin};


fn lexer(string: String){
    println!("Todo");
}

fn main(){
    let flag = true;
    while flag{
        println!("ZenScript>");
        let mut input = String::new();
        stdin().read_line(&mut input);
        // Todo: Implement a Lexer which will scan and tokenize the input string.
        lexer(input);
    }
}