use std::io;

fn main(){
    println!("~Guess the number between 1 and 100~");
    println!("Enter your guess: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {guess}")
}