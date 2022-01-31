use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");
    println!("Enter your guess!!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp($secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }
}
