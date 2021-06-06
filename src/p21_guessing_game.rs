use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
 * Project 21: Guessing Game
 * */
pub fn guessing_game() {
    println!("Guess the number!");

    // gen_range is inclusive on the lower bound but exclusive on the upper bound,
    // so we need to specify 1..101 to request a number between 1 and 100.
    // Alternatively, we could pass the range 1..=100, which is equivalent.
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess:");
        
        let mut guess: String = String::new();

        io::stdin()
            // The & indicates that this argument is a reference.
            // References are immutable by default. Hence, you need to write &mut guess.
            .read_line(&mut guess)
            // read_line returns a value, in this case, an io::Result
            // For Result, the variants are Ok or Err.
            .expect("Failed to read line");

        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
        // Also, let’s make the game ignore a non-number so the user can continue guessing
        // 'continue' tells the program to go to the next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}