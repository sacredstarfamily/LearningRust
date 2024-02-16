use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("{secret}");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{ println!("must guess number");continue},
        };

        println!("you guessed: {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!("you guessed too small"),
            Ordering::Greater => println!("you guessed to big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
