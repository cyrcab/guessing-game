use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_value) => {
                println!("{}", "The input must be a number".red().bold());
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".yellow().bold()),
            Ordering::Greater => println!("{}", "Too big".blue().bold()),
            Ordering::Equal => {
                println!("{}", "You win".green().bold());
                break;
            }
        }
    }
}