use dialoguer::{Input, Select};
use std::process::Command;
use colored::*;

pub fn main_menu() {
    let options = vec![
        "Initialize a Repository",
        "Clone a Repository",
    ];

    let selection = Select::new()
        .with_prompt("Select a command")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => initialize_repository(),
        1 => clone_repository(),
        _ => println!("{}",
            "Invalid selection".red()),
    }
}

fn initialize_repository() {
    let output = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn clone_repository() {
    let url: String = Input::new()
        .with_prompt("Enter the repository URL to clone")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
