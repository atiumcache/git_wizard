extern crate colored;
extern crate dialoguer;

use crate::sleeper::Sleeper;
use colored::*;
use std::io::Write;
use std::process::{Command, Output};
use std::{io, str};

/// Check for global git config variables
/// and an initialized git repository.
pub fn startup_check() {
    if !check_global_config() {
        setup_global_config();
    }
}

// Check if global username and email are set.
fn check_global_config() -> bool {
    let output = run_git_config_show_origin();

    if !output.status.success() {
        eprintln!(
            "Command failed with error: {}",
            str::from_utf8(&output.stderr).unwrap()
        );
    }

    let stdout = str::from_utf8(&output.stdout).expect("Failed to parse output");

    let mut username_set = false;
    let mut email_set = false;

    for line in stdout.lines() {
        if line.contains("user.name=") {
            username_set = true;
        }
        if line.contains("user.email=") {
            email_set = true;
        }
    }

    return username_set && email_set;
}

/// Returns the output of:
///    git config --list --show-origin
fn run_git_config_show_origin() -> Output {
    Command::new("git")
        .args(&["config", "--list", "--show-origin"])
        .output()
        .expect("Failed to execute command")
}

/// Walks the user through setting up
/// their global git config variables:
///     user.name and user.email
fn setup_global_config() {
    println!(
        r#"
Your global config variables are not set.
Let's set them up now!
        "#
    );
    let sleeper = Sleeper::new();

    let mut username = String::new();
    let mut email = String::new();

    prompt_for_username(&mut username);
    prompt_for_email(&mut email);

    display_global_config_commands(&mut username, &mut email);

    // Set Git username
    let username_output = run_git_config_username(&mut username);

    let email_output = run_git_config_email(&mut email);

    sleeper.sleep_medium();

    display_set_config_status(
        &mut username,
        &mut email,
        username_output,
        email_output,
        &sleeper,
    );
}

fn display_set_config_status(
    username: &mut String,
    email: &mut String,
    username_output: Output,
    email_output: Output,
    sleeper: &Sleeper,
) {
    if username_output.status.success() {
        println!("Success! Set Git username to '{}'.", username);
        sleeper.sleep_medium();
    } else {
        eprintln!("Failed to set Git username.");
    }

    if email_output.status.success() {
        println!("Success! Set Git email to '{}'.", email);
        sleeper.sleep_medium();
    } else {
        eprintln!("Failed to set Git email.");
    }
}

fn display_global_config_commands(_username: &mut String, _email: &mut String) {
    println!("\nRunning commands:");
    println!(
        "{}{}\n{}{}\n",
        "\tgit config --global user.name ".cyan(),
        "'username'".cyan(),
        "\tgit config --global user.email ".cyan(),
        "'email'".cyan()
    );
}

/// Returns the output of:
///     git config --global user.email <email>
fn run_git_config_email(email: &mut String) -> Output {
    Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg(email.clone())
        .output()
        .expect("Failed to set Git email")
}

/// Returns the output of:
///     git config --global user.name <username>
fn run_git_config_username(username: &mut String) -> Output {
    Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg(username.clone())
        .output()
        .expect("Failed to set Git username")
}

fn prompt_for_username(username: &mut String) {
    print!("Enter your username: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(username)
        .expect("Failed to read line");
    *username = username.trim().to_string();
}

fn prompt_for_email(email: &mut String) {
    print!("Enter your email: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(email).expect("Failed to read line");
    *email = email.trim().to_string();
}
