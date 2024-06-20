use crate::utils::{clear_screen, display_banner, read_key_selection};
use colored::*;
use std::io::Result;
use std::process::Command;

pub fn maintenance_menu() -> Result<()> {
    loop {
        display_inspection_and_maintenance_menu()?;

        let selection = read_key_selection(&["1", "2", "3", "4", "m", "q"])?;

        match selection.as_str() {
            "1" => view_log()?,
            "2" => check_status()?,
            "3" => apply_stash()?,
            "4" => drop_stash()?,
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

fn display_inspection_and_maintenance_menu() -> Result<()> {
    // Clear the terminal screen
    clear_screen();
    display_banner();

    println!("{}", "[1] View Commit Log".bright_cyan());
    println!("{}", "[2] Check Repository Status".bright_cyan());
    println!("{}", "[3] Apply Stash".bright_cyan());
    println!("{}", "[4] Drop Stash".bright_cyan());
    println!("{}", "[m] Back to Main Menu".bright_yellow());
    println!("{}", "[q] Quit".bright_red());
    Ok(())
}

fn view_log() -> Result<()> {
    let output = Command::new("git")
        .arg("log")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn check_status() -> Result<()> {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn apply_stash() -> Result<()> {
    let output = Command::new("git")
        .arg("stash")
        .arg("apply")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn drop_stash() -> Result<()> {
    let output = Command::new("git")
        .arg("stash")
        .arg("drop")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
