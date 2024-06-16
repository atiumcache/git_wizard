extern crate colored;
extern crate git_wizard;
extern crate dialoguer;
extern crate crossterm;

use std::io::Result;
use colored::*;

use git_wizard::startup;

mod init_and_clone;
mod make_changes;
mod branching;
mod syncing;
mod maintenance;
mod utils;
pub mod sleeper;


fn main() -> Result<()> {
    println!("\nWelcome to Git Wizard! 🧙");
    startup::startup_check();

    loop {
        utils::display_main_menu()?;

        let category_selection = utils::read_key_selection(&["1", "2", "3",
        "4", "5", "q"])?;

        match category_selection.as_str() {
            "1" => init_and_clone::main_menu()?,
            "2" => make_changes::main_menu()?,
            "3" => branching::branching_and_merging_menu()?,
            "4" => syncing::synchronization_menu()?,
            "5" => maintenance::inspection_and_maintenance_menu()?,
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid selection"),
        }
    }

    Ok(())
}

