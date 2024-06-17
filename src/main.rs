extern crate colored;
extern crate crossterm;
extern crate dialoguer;

use std::io::Result;

pub mod init_and_clone;
pub mod branching;
pub mod maintenance;
pub mod make_changes;
pub mod syncing;
pub mod sleeper;
pub mod startup;
pub mod utils;

fn main() -> Result<()> {
    println!("\nWelcome to Git Wizard! 🧙");
    startup::startup_check();

    loop {
        utils::display_main_menu()?;

        let category_selection = utils::read_key_selection(&["1", "2", "3", "4", "5", "q"])?;

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
