use std::io;
use rand::Rng;
use std::cmp::Ordering

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number)


    println!("Guess the number!");
    println!("Enter your guess!!");

    let mut guess = String::new();
    let guess: u32 = guess.trim().parse().expect("Please type a number!")

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    match guess.cmpp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Large"),
        Ordering::Equal => println!("Bingo!! You win!")
    }
}
