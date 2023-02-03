use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn print_game_heading() {
    println!("Welcome player, lets play the guessing game.");
    println!("The game works just like this");
    println!("first the game generates a number between 1 and 100");
    println!("then you will be asket to input a number, if you hit the number generated you win");
}

fn main() {
    print_game_heading();

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();

    let result = io::stdin().read_line(&mut guess);
    result.expect("Something went wring while trying to read the input");

    let guess: u8 = guess.trim().parse().expect("Please type a number between 1 and 100!");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!!!"),
        Ordering::Less => println!("Smaller!"),
        Ordering::Greater => println!("Bigger!"),
    }
    
    println!("You guessed: {guess}");
}
