use std::io::{stdin, stdout, Write};
use rand::{RngCore, SeedableRng};
use rand::rngs::OsRng;
use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};


fn main() {
    // Load the user's password database from a file
    let database = load_database();

    // Start the main loop of the program
    loop {
        // Print the main menu
        println!("Welcome to the password manager!");
        println!("1. Create a new user account");
        println!("2. Log in to an existing account");
        println!("3. Quit");

        // Get the user's input
        let choice = get_input("Enter your choice: ");

        // Perform the appropriate action based on the user's input
        match choice.as_str() {
            "1" => create_account(&database),
            "2" => login(&database),
            "3" => {
                // Save the database to a file before quitting
                save_database(&database);
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
