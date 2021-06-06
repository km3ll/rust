use std::io;
mod chapter_04;
mod p01_guessing_game;

fn main() {
    menu();
}

fn menu() {
    loop {
        println!("Select a module:");
        println!("  4. Ownership");
        println!(" 20. Guessing Game");
        println!("  0. Exit");

        let mut option: String = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        
        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match option {
            4  => chapter_04::greeting(),
            20 => p01_guessing_game::guessing_game(),
            0  => break,
            _ => continue,
        };

    }
}