use std::process::Command;

fn main() {
    Command::new("pass").arg("ls").status().unwrap(); 
}
