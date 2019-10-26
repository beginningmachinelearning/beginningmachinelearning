use std::io;

fn main() {
    let actual = 42;
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    if guess.parse::<i32>().unwrap() == actual {
        println!("Your guess is correct.");
    } else {
        println!("Sorry but your guess is not correct. Please play again.");
    }
}
