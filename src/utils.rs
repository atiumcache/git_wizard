use crossterm::{event::{self, Event, KeyCode, KeyEvent}, cursor, terminal, ExecutableCommand};
use std::io::Result;
use colored::*;
use std::io;

pub fn read_key_selection(valid_keys: &[&str]) -> Result<String> {
    use std::io::{self, Write};

    let mut input = String::new();

    loop {
        print!("Please enter your choice: ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        // Trim the input and check if it is a valid key
        let key_str = input.trim().to_string();
        if valid_keys.contains(&key_str.as_str()) {
            return Ok(key_str);
        } else {
            println!("Invalid selection: {}", key_str);
        }
    }
}

pub fn display_banner() -> Result<()> {
    println!("{}", "✨🔮 Welcome to Git Wizard 🔮✨\n");
    Ok(())
}

pub fn clear_screen() -> Result<()> {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    io::stdout().execute(cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn display_main_menu() -> Result<()> {
    clear_screen();
    display_banner();
    println!("{}", "[1] Initialization and Cloning".bright_cyan());
    println!("{}", "[2] Making Changes".bright_cyan());
    println!("{}", "[3] Branching and Merging".bright_cyan());
    println!("{}", "[4] Synchronization".bright_cyan());
    println!("{}", "[5] Inspection and Maintenance".bright_cyan());
    println!("{}", "[q] Exit".bright_red());
    Ok(())
}

pub fn wait_for_any_key() -> Result<()> {
    use std::io::{self, Write};
    println!("\nPress any key to return to the main menu...");
    io::stdout().flush()?;

    // Wait for any key press
    loop {
        if event::poll(std::time::Duration::from_secs(1))? {
            if let event::Event::Key(_) = event::read()? {
                break;
            }
        }
    }
    Ok(())
}
