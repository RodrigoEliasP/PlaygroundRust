use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn print_game_heading() {
    println!("Welcome player, lets play the guessing game.");
    println!("The game works just like this");
    println!("first the game generates a number between 1 and 100");
    println!("then you will be asked to input a number, if you hit the number generated you win");
}

fn main() {
    print_game_heading();

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();

        let result = io::stdin().read_line(&mut guess);
        result.expect("Something went wring while trying to read the input");

        println!("You guessed: {guess}");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
            Ordering::Less => println!("Smaller!"),
            Ordering::Greater => println!("Bigger!"),
        }
    }
}
