use std::process::Command;
use dialoguer::Select;

pub fn inspection_and_maintenance_menu() {
    let options = vec![
        "View Commit Log",
        "Check Repository Status",
        "Apply Stash",
        "Drop Stash",
    ];

    let selection = Select::new()
        .with_prompt("Select a command")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => view_log(),
        1 => check_status(),
        2 => apply_stash(),
        3 => drop_stash(),
        _ => println!("Invalid selection"),
    }
}

fn view_log() {
    let output = Command::new("git")
        .arg("log")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn check_status() {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn apply_stash() {
    let output = Command::new("git")
        .arg("stash")
        .arg("apply")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn drop_stash() {
    let output = Command::new("git")
        .arg("stash")
        .arg("drop")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
