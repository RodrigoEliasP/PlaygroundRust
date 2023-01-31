use::std::io;

fn print_game_heading() {
    println!("Welcome player, lets play the guessing game.");
    println!("The game works just like this");
    println!("first the game generates a number between 1 and 100");
    println!("then you will be asket to input a number, if you hit the number generated you win");
}

fn main() {
    print_game_heading();

    let mut guess = String::new();

    let result = io::stdin().read_line(&mut guess);
    result.expect("Something went wring while trying to read the input");
    
    println!("You guessed: {guess}");
}
