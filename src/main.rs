use std::io::{stdin};
use std::str;
// use std::vec;



//////////////////////////////////////////////////////////////////////
//                             Constants                            //
//////////////////////////////////////////////////////////////////////
static  DIGITS: &str = "0123456789";



//////////////////////////////////////////////////////////////////////
//                             Tokens                               //
//////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Tokens{
    NUM,
    PLUS,
    MINUS,
    DIV,
    MUL
}




//////////////////////////////////////////////////////////////////////
//                             Lexer                                //
//////////////////////////////////////////////////////////////////////


fn lexer(source: String) -> Vec<Tokens>{
    // println!("{}", string);
    let mut token: Vec<Tokens> = Vec::new();
    for c in source.chars(){
        if DIGITS.contains(c){
            token.push(Tokens::NUM);
        }
        if c == '+'{
            token.push(Tokens::PLUS);
        }
        if c == '-'{
            token.push(Tokens::MINUS);
        }
        if c == '*'{
            token.push(Tokens::MUL);
        }
        if c == '/'{
            token.push(Tokens::DIV);
        }

    }
    // println!("{:?}", token);
    return token;
}





fn main(){
    let flag = true;
    while flag{
        println!("ZenScript> ");
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let input = input.trim().to_string();
        // Todo: Implement a Lexer which will scan and tokenize the input string.
        println!("{:?}", lexer(input));
    }
}


