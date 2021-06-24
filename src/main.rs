mod database;
mod ferroql;

use std::io::{self, Write};


fn main() {
    // let mut arguments = std::env::args().skip(1);
    // let key = arguments.next().unwrap();
    // let value = arguments.next().unwrap();
    // println!("Input -> key='{}' and value='{}'", key, value);
    // let mut db = database::Database::new().expect("database creation crashed!");
    // db.insert(key.clone(), value.clone());
    //db.flush().unwrap();

    loop {
        print!("ferroQL>: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");

        let mut l = ferroql::lexer::Lexer::new(input.chars().collect());
        let tokens = l.lex();

        for token in tokens.into_iter() {
            println!("{}", token);
        }
    }    
}