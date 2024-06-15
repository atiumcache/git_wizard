extern crate colored;
extern crate git_wizard;

use std::collections::HashSet;
use std::io::{self};

use git_wizard::startup;


fn main() {
    println!("\nWelcome to Git Wizard! 🧙");
    startup::startup_check();
    let mut user_input = receive_user_input();
    while validate_input(user_input) != true {
        user_input = receive_user_input();
    }
}


fn receive_user_input() -> String {
    println!("What operation do you want to perform?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}


fn validate_input(user_input: String) -> bool {
    let valid_inputs: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();

    match user_input.trim().parse::<i32>() {
        Ok(num) => {
            if valid_inputs.contains(&num) {
                println!("You entered valid input: {}", num);
                return true;
            } else {
                println!("The entered integer {} is not valid.", num);
                return false;
            }
        }
        Err(_) => {
            println!("Please enter an integer from the given choices.");
            return false;
        }
    }
}