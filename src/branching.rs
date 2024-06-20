use crate::utils::{clear_screen, display_banner, read_key_selection, wait_for_any_key};
use colored::Colorize;
use dialoguer::Input;
use std::io::Result;
use std::process::Command;

pub fn branching_menu() -> Result<()> {
    loop {
        display_branching_and_merging_menu()?;

        let selection = read_key_selection(&["1", "2", "3", "4", "m", "q"])?;

        match selection.as_str() {
            "1" => create_branch()?,
            "2" => switch_branch()?,
            "3" => merge_branch()?,
            "4" => delete_branch()?,
            "m" => break,
            "q" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => println!("{}", "Invalid selection".bright_red()),
        }
        wait_for_any_key();
        break;
    }
    Ok(())
}

fn display_branching_and_merging_menu() -> Result<()> {
    // Clear the terminal screen
    clear_screen();
    display_banner();

    println!("{}", "[1] Create a New Branch".bright_cyan());
    println!("{}", "[2] Switch Branches".bright_cyan());
    println!("{}", "[3] Merge Branches".bright_cyan());
    println!("{}", "[4] Delete Branch".bright_cyan());
    println!("{}", "[m] Back to Main Menu".bright_yellow());
    println!("{}", "[q] Quit".bright_red());
    Ok(())
}

fn create_branch() -> Result<()> {
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
        println!(
            "{}",
            format!("Branch '{}' created successfully.", branch_name.clone()).green()
        );
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
    Ok(())
}

fn switch_branch() -> Result<()> {
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
        println!(
            "{}",
            format!("Switched to branch '{}'.", branch_name.clone()).green()
        );
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
    Ok(())
}

fn merge_branch() -> Result<()> {
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
        println!(
            "{}",
            format!("Branch '{}' merged successfully.", branch_name.clone()).green()
        );
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
    Ok(())
}

fn delete_branch() -> Result<()> {
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
        println!(
            "{}",
            format!("Branch '{}' deleted successfully.", branch_name.clone()).green()
        );
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
    Ok(())
}
