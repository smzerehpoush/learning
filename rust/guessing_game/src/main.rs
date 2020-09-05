use std::io;

fn main() {

    println!("Guess The Number!");
    println!("input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You Guessed : {} ", guess);
}

