use dialoguer::{Input, Select};
use std::process::Command;
use colored::Colorize;

pub fn synchronization_menu() {
    let options = vec![
        "View Remote Repositories",
        "Add Remote Repository",
        "Push Changes",
        "Pull Changes",
        "Fetch Changes",
    ];

    let selection = Select::new()
        .with_prompt("Select a command")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => view_remotes(),
        1 => add_remote(),
        2 => push_changes(),
        3 => pull_changes(),
        4 => fetch_changes(),
        _ => println!("{}",
            "Invalid selection".red()),
    }
}

fn view_remotes() {
    let output = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn add_remote() {
    let name: String = Input::new()
        .with_prompt("Enter the remote name")
        .interact_text()
        .unwrap();
    let url: String = Input::new()
        .with_prompt("Enter the remote URL")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg(name)
        .arg(url)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn push_changes() {
    let remote: String = Input::new()
        .with_prompt("Enter the remote name")
        .interact_text()
        .unwrap();
    let branch: String = Input::new()
        .with_prompt("Enter the branch name")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("push")
        .arg(remote)
        .arg(branch)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn pull_changes() {
    let remote: String = Input::new()
        .with_prompt("Enter the remote name")
        .interact_text()
        .unwrap();
    let branch: String = Input::new()
        .with_prompt("Enter the branch name")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("pull")
        .arg(remote)
        .arg(branch)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn fetch_changes() {
    let remote: String = Input::new()
        .with_prompt("Enter the remote name")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("fetch")
        .arg(remote)
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
