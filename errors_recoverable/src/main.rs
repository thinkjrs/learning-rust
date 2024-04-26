use std::io::{self};

fn validate_username(username: &str) -> Result<String, io::Error> {
    match username.is_ascii() {
        true => Ok(username.to_string()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Username must be ASCII",
        )),
    }
}
fn main() {
    let username = "my_username ❤️";
    match validate_username(username) {
        Ok(_) => {
            print!("{} username is good to go!", username);
        }
        Err(e) => match e.kind() {
            io::ErrorKind::InvalidInput => {
                // ask the user for another user name
                println!("Handling InvalidInput error:");
                println!("{} username is not valid: {}", username, e);
            }
            _ => {
                println!("{} username is not valid: {}", username, e);
            }
        },
    }
}
