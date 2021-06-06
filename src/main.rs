use std::io;
mod chapter_04;
mod p00_unit_tests;
mod p21_guessing_game;
mod p22_console_cli;

fn main() {
    menu();
}

fn menu() {
    loop {
        println!("Run code in module:");
        println!("  4. Ownership");
        println!(" 21. Guessing Game");
        println!(" 22. Console Cli");
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
            21 => p21_guessing_game::guessing_game(),
            22 => p22_console_cli::console_cli(),
            0  => break,
            _ => continue,
        };

    }
}