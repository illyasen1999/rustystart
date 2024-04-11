use rand::Rng;
use std::io;
use std::cmp::Ordering;

#[warn(dead_code)]
pub fn guessing_game(){
    println!("Guess the number(1-10): ");

    // immutable
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        // mutable
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failure to read");
    
        // shadowing
        let guess: u32 = match guess
        .trim()
        .parse(){
            // returns a Result type
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You've guessed {}", guess);
    
        // pattern matching
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win");
                println!("The secret number is: {}", secret_number);
                break;
            }
        }
    }
}