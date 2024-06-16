extern crate colored;
extern crate git_wizard;
extern crate dialoguer;

use dialoguer::Select;
use colored::*;

use git_wizard::startup;

mod init_and_clone;
mod make_changes;
mod branching;
mod syncing;
mod maintenance;

fn main() {
    println!("\nWelcome to Git Wizard! 🧙");
    startup::startup_check();

    let categories = vec![
        "Initialization and Cloning",
        "Making Changes",
        "Branching and Merging",
        "Synchronization",
        "Inspection and Maintenance",
    ];

    let category_selection = Select::new()
        .with_prompt("Select a category")
        .items(&categories)
        .interact()
        .unwrap();

    match category_selection {
        0 => init_and_clone::main_menu(),
        1 => make_changes::main_menu(),
        2 => branching::branching_and_merging_menu(),
        3 => syncing::synchronization_menu(),
        4 => maintenance::inspection_and_maintenance_menu(),
        _ => println!("Invalid selection"),
    }
}

