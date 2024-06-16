use dialoguer::{Input, Select};
use std::process::Command;
use colored::Colorize;

pub fn branching_and_merging_menu() {
    let options = vec![
        "Create a New Branch",
        "Switch Branches",
        "Merge Branches",
        "Delete Branch",
    ];

    let selection = Select::new()
        .with_prompt("Select a command")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => create_branch(),
        1 => switch_branch(),
        2 => merge_branch(),
        3 => delete_branch(),
        _ => println!("{}",
            "Invalid selection".red()),
    }
}

fn create_branch() {
    let branch_name: String = Input::new()
        .with_prompt("Enter the new branch name")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("branch")
        .arg(branch_name.clone())
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("{}", format!("Branch '{}' created successfully.", branch_name.clone()).green());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}

fn switch_branch() {
    let branch_name: String = Input::new()
        .with_prompt("Enter the branch name to switch to")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("checkout")
        .arg(branch_name.clone())
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("{}", format!("Switched to branch '{}'.", branch_name.clone()).green());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}

fn merge_branch() {
    let branch_name: String = Input::new()
        .with_prompt("Enter the branch name to merge")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("merge")
        .arg(branch_name.clone())
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("{}", format!("Branch '{}' merged successfully.", branch_name.clone()).green());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}

fn delete_branch() {
    let branch_name: String = Input::new()
        .with_prompt("Enter the branch name to delete")
        .interact_text()
        .unwrap();

    let output = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch_name.clone())
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("{}", format!("Branch '{}' deleted successfully.", branch_name.clone()).green());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}
