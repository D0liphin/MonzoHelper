use crate::*;
use crate::types::*;

pub fn token(user: &user::User, _command: &cli::Command) -> Result<(), Box<dyn std::error::Error>> {
    println!("token expires at {}", user.access_token.expires);
    Ok(())
}