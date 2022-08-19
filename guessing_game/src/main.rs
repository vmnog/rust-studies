use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut missed_points: u32 = 0;
    let mut score = 100;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                missed_points += 10;
                println!("Too small");
            },
            Ordering::Greater => {
                missed_points += 10;
                println!("Too big");
            },
            Ordering::Equal => {
                score -= missed_points;
                println!("You win!, your score is {score}");
                break;
            },
        }
    }
}
