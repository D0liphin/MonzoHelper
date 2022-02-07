use std::io::Write;

use monzo::types::user::User;
use monzo::*;

#[tokio::main]
async fn main() {
    let mut command = cli::Command::new(std::env::args());
    if command.args.len() <= 1 {
        command.args.push("help".to_owned());
    }

    match command.args[1].as_str() {
        "help" => {
            println!("{}", include_str!("../txt/help"));
            return;
        }
        "auth" => {
            let _ = commands::auth().await;
            return;
        }
        command_ident => {
            let (user, client) = ensure_authorized_user().await;
            match command_ident {
                "balance" => commands::balance(&user, &client, &command),
                "account" => commands::account(&user, &command),
                _ => println!("ERROR: Unknown command, use `help` for a list of commands"),
            }
        }
    }
}

/// Ensures that there is a valid user file present on this system (does not necessarily
/// mean it has not expired)
async fn ensure_authorized_user() -> (types::user::User, reqwest::Client) {
    loop {
        match user_file::load_user_file() {
            Ok(user) => {
                let client = user.create_authorized_client();
                break (user, client);
            }
            Err(_) => {
                println!("Could not load user file, please authorize this application");
                if let Ok(user_client) = commands::auth().await {
                    break user_client;
                }
                continue;
            }
        }
    }
}
