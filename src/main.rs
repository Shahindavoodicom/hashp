use sha2::{Sha256, Digest};
use colored::Colorize;
use std::io;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let secret = env::var("SECRET").expect("SECRET not found");

    loop {
        println!("{}", "input the secret and for quite input q".yellow() );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cant read input");

        if input.trim() == "q" {
            break
        }

        let mut hasher = Sha256::new();
        hasher.update(format!("{} {}", secret, input.trim()));
        let mut first_char_flag = false;

        let result = hasher.finalize();
        let result = format!("{:x}", result).chars().map(|c| {
            if !first_char_flag && !c.is_numeric() {
                first_char_flag = true;
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        }).collect::<String>() + "@";

        println!("{}", result.green());
    }
}