use std::borrow::Borrow;

use crate::*;

/// Returns the balance of the account
pub async fn get_balance(
    user: &types::user::User,
    client: &reqwest::Client,
) -> Result<types::Balance, Box<dyn std::error::Error>> {
    let balance = client
        .get(&format!(
            "{}/balance?account_id={}",
            consts::MONZO_API,
            user.accounts[0].id
        ))
        .send()
        .await?
        .bytes()
        .await?;

    Ok(serde_json::from_slice(&balance)?)
}

/// The `balance` command
pub fn balance(user: &types::user::User, client: &reqwest::Client, command: &cli::Command) {
    let balance = match pollster::block_on(get_balance(user, client)) {
        Ok(balance) => balance,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if command.args_set.contains("--detailed") {
        println!(
            "BALANCE: {}\n\
            TOTAL BALANCE: {}",
            balance.balance_string(),
            balance.total_balance_string()
        );
    } else {
        println!("{}", balance.balance_string());
    }
}
