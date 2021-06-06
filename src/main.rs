use std::io;
mod chapter_04;

fn main() {
    menu();
}

fn menu() {
    loop {
        println!("Select the module:");
        println!("  4. Ownership");
        println!("  0. Exit");

        let mut option: String = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        
        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        if option == 4 {
            chapter_04::greeting();
        } else if option == 0 {
            break;
        } else {
            continue;
        }

    }
}