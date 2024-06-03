use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("~Guess the number between 1 and 100~");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 =  match guess.trim().parse() {
            Result::Ok(num) => num,
            Result::Err(e) => {
                println!("What are you doing? enter an number : {e}");
                continue;
            },
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
    
}