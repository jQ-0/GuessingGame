use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let welcome = "\n*~~GUESS THE NUMBER~~*".bold();
    let winner = "~*YOU WIN!*~".bold();
    println!("{welcome}");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //    println!("The secret number is {secret_number}");

    loop {
        println!("Go on ahead and make a guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("{winner}");
                break;
            }
        }
    }
}
