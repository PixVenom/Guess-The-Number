use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num: u32 = rand::thread_rng().gen_range(1..=50);
    println!("Secret number {secret_num}");
    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid input");
                continue;
            }
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You Won!!");
                break;
            }
            Ordering::Greater => println!("Number is greater"),
            Ordering::Less => println!("Number is small"),
        }
    }
}