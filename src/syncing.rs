use crate::utils::{clear_screen, display_banner, read_key_selection};
use colored::Colorize;
use dialoguer::Input;
use std::io::Result;
use std::process::Command;

pub fn sync_menu() -> Result<()> {
    loop {
        display_synchronization_menu()?;

        let selection = read_key_selection(&["1", "2", "3", "4", "m", "q"])?;

        match selection.as_str() {
            "1" => push_changes()?,
            "2" => pull_changes()?,
            "3" => fetch_changes()?,
            "4" => view_remotes()?,
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

fn display_synchronization_menu() -> Result<()> {
    // Clear the terminal screen
    clear_screen();
    display_banner();

    println!("{}", "[1] Push Changes".bright_cyan());
    println!("{}", "[2] Pull Changes".bright_cyan());
    println!("{}", "[3] Fetch Changes".bright_cyan());
    println!("{}", "[4] View Remotes".bright_cyan());
    println!("{}", "[m] Back to Main Menu".bright_yellow());
    println!("{}", "[q] Quit".bright_red());
    Ok(())
}

fn view_remotes() -> Result<()> {
    let output = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn add_remote() -> Result<()> {
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
    Ok(())
}

fn push_changes() -> Result<()> {
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
    Ok(())
}

fn pull_changes() -> Result<()> {
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
    Ok(())
}

fn fetch_changes() -> Result<()> {
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
    Ok(())
}
