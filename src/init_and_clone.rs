use crate::utils::{clear_screen, display_banner, read_key_selection};
use crate::sleeper;
use colored::*;
use dialoguer::Input;
use std::io::Result;
use std::process::{Command, Output};

pub fn init_menu() -> Result<()> {
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

/// Initialize git in the current working directory.
fn initialize_repository() -> Result<()> {
    let sleeper = sleeper::Sleeper::new();

    println!("\nRunning command: \n\t{}", "git init".cyan());
    let output = run_git_init();
    sleeper.sleep_medium();

    if output.status.success() {
        println!("\nSuccess! Git repository initialized.");
    } else {
        eprintln!(
            "Failed to initialize Git repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

fn run_git_init() -> Output {
    Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute git init command")
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
