use crate::utils::{clear_screen, display_banner, read_key_selection};
use colored::Colorize;
use dialoguer::Input;
use std::io::Result;
use std::process::Command;

pub fn main_menu() -> Result<()> {
    loop {
        display_making_changes_menu()?;

        let selection = read_key_selection(&["1", "2", "m", "q"])?;

        match selection.as_str() {
            "1" => add_changes()?,
            "2" => commit_changes()?,
            "m" => break,
            "q" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => println!("{}", "Invalid selection".bright_red()),
        }
    }
    Ok(())
}

fn display_making_changes_menu() -> Result<()> {
    // Clear the terminal screen
    clear_screen();
    display_banner();

    println!("{}", "[1] Add Changes".bright_cyan());
    println!("{}", "[2] Commit Changes".bright_cyan());
    println!("{}", "[m] Back to Main Menu".bright_yellow());
    println!("{}", "[q] Quit".bright_red());
    Ok(())
}

fn add_changes() -> Result<()> {
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
    Ok(())
}

fn commit_changes() -> Result<()> {
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
    Ok(())
}

fn amend_last_commit() -> Result<()> {
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
    Ok(())
}

fn stash_changes() -> Result<()> {
    let output = Command::new("git")
        .arg("stash")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
