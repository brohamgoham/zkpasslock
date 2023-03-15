// models/database.rs
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::models::UserAccount;

fn load_database() -> Result<Vec<UserAccount>, Box<dyn Error>> {
    let mut file = File::open("database.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let accounts: Vec<UserAccount> = serde_json::from_str(&contents)?;

    Ok(accounts)
}
