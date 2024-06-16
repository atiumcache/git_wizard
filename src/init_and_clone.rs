use dialoguer::{Input};
use std::process::Command;
use colored::*;
use std::io::Result;
use crate::utils::{clear_screen, display_banner, read_key_selection};

pub fn main_menu() -> Result<()> {
        loop {
            display_initialization_and_cloning_menu()?;

            let selection = read_key_selection(&["1", "2", "m", "q"])?;

            match selection.as_str() {
                "1" => initialize_repository()?,
                "2" => clone_repository()?,
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

fn display_initialization_and_cloning_menu() -> Result<()> {
    clear_screen();
    display_banner();
    println!("{}", "[1] Initialize a Repository".bright_cyan());
    println!("{}", "[2] Clone a Repository".bright_cyan());
    println!("{}", "[m] Back to Main Menu".bright_yellow());
    println!("{}", "[q] Quit".bright_red());
    Ok(())
}

fn initialize_repository() -> Result<()> {
    let output = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

fn clone_repository() -> Result<()> {
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

    Ok(())
}
