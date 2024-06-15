extern crate colored;

use std::process::{Command, exit};
use std::{io, str};
use std::io::Write;
use colored::*;
use crate::sleeper::Sleeper;


pub fn startup_check() {
    if !check_global_config() {
        setup_global_config();
    }

    match is_git_initialized() {
        Ok(false) => prompt_initialize_git(),
        Ok(true) => (),
        Err(e) => eprintln!("Error checking if Git is initialized: {}", e),
    }
}


fn is_git_initialized() -> Result<bool, std::io::Error> {
    let current_dir = std::env::current_dir()?;
    let git_dir = current_dir.join(".git");

    Ok(git_dir.exists() && git_dir.is_dir())
}


fn prompt_initialize_git() {
    println!("\nNo Git repository found in the current directory. Do you \
    want to initialize a new Git repository? (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("y") {
        initialize_git();
    } else {
        println!("Git repository initialization skipped. Ending program.");
        exit(0);
    }
}


fn initialize_git() {
    println!("\nRunning command: \n\t{}", "git init".cyan());
    let output = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute git init command");

    if output.status.success() {
        println!("\nSuccess! Git repository initialized.");
    } else {
        eprintln!(
            "Failed to initialize Git repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}


// Check if global username and email are set.
fn check_global_config() -> bool {
    let output = Command::new("git")
        .args(&["config", "--list", "--show-origin"])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Command failed with error: {}", str::from_utf8(&output
            .stderr).unwrap());
    }

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to parse output");

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


fn setup_global_config() {
    println!(
        r#"
Your global config variables are not set.
Let's set them up now!
        "#
    );

    let mut username = String::new();
    let mut email = String::new();

    prompt_for_username(&mut username);
    prompt_for_email(&mut email);

    println!("\nRunning commands:");
    println!(
        "{}{}\n{}{}\n",
        "\tgit config --global user.name ".cyan(),
        username.cyan(),
        "\tgit config --global user.email ".cyan(),
        email.cyan()
    );

     // Set Git username
    let username_output = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg(username.clone())
        .output()
        .expect("Failed to set Git username");

    let email_output = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg(email.clone())
        .output()
        .expect("Failed to set Git email");

    if username_output.status.success() {
            println!("Success! Set Git username to '{}'.", username);
        } else {
            eprintln!("Failed to set Git username.");
        }

    if email_output.status.success() {
            println!("Success! Set Git email to '{}'.", email);
        } else {
            eprintln!("Failed to set Git email.");
        }
}


fn prompt_for_username(username: &mut String) {
    print!("Enter your username: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(username).expect("Failed to read line");
    *username = username.trim().to_string();
}


fn prompt_for_email(email: &mut String) {
    print!("Enter your email: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(email).expect("Failed to read line");
    *email = email.trim().to_string();
}
