use std::process::{Command};

fn main() {

    println!("hello, world!");

    let mut cmd = Command::new(String::from("docker"));
    cmd.arg("--help");

    let output = cmd.output().expect("Error getting output");
    let result: String = String::from_utf8(output.stdout).expect("Error converting from utf8");

    let mut lines = result.lines().map(|s| s.trim());

    while let Some(line) = lines.next() {
        match line {
            "Commands:" => {
                while let Some(cmd) = lines.next() {
                    if cmd == "" {
                        break;
                    } else {
                        println!("{}", cmd);
                    }
                }
            }
            _ => continue
        }
    }

}