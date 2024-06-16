use dialoguer::{Input, Select};
use colored::Colorize;
use std::process::Command;

pub fn main_menu() {
    let options = vec![
        "Add Changes",
        "Commit Changes",
        "Amend Last Commit",
        "Stash Changes",
    ];

    let selection = Select::new()
        .with_prompt("Select a command")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => add_changes(),
        1 => commit_changes(),
        2 => amend_last_commit(),
        3 => stash_changes(),
        _ => println!("{}",
            "Invalid selection".red()),
    }
}

fn add_changes() {
    let files: String = Input::new()
        .with_prompt("Enter the files to add")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("add")
        .arg(files)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn commit_changes() {
    let message: String = Input::new()
        .with_prompt("Enter the commit message")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn amend_last_commit() {
    let message: String = Input::new()
        .with_prompt("Enter the amended commit message")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("-m")
        .arg(message)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn stash_changes() {
    let output = Command::new("git")
        .arg("stash")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
