use std::io::Write;

use monzo::commands::get_transactions;
use monzo::types::*;
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
            let (user, client) = match ensure_authorized_user().await {
                Ok(uc) => uc,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let res = match command_ident {
                "balance" => commands::balance(&user, &client, &command),
                "account" => commands::account(&user, &command),
                "token" => commands::token(&user, &command),
				"transactions" => commands::transactions(&user, &client, &command),
                _ => Err(error::BadArgumentError(format!(
                    "invalid command `{}`, use `help` for a list of commands",
                    command_ident
                ))
                .into()),
            };
            if let Err(e) = res {
                println!("{}", e);
            }
        }
    }
}

/// Ensures that there is a valid user file present on this system (does not necessarily
/// mean it has not expired)
async fn ensure_authorized_user() -> Result<(user::User, reqwest::Client), error::UserFileError> {
    let user = match user_file::load_user_file() {
        Ok(user) => user,
        Err(_) => return Err(error::UserFileError::InvalidOrAbsent),
    };

    let now = time::Time::now();
    if now >= user.access_token.expires {
        println!(
            "found an access token, but it expired -- please authorize this \
            application again"
        );
        return Err(error::UserFileError::Expired);
    }
    let client = user.create_authorized_client();

    Ok((user, client))
}
