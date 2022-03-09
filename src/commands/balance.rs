use crate::types::*;
use crate::*;

/// Returns the balance of the account
pub async fn get_balance(
    user: &types::user::User,
    client: &reqwest::Client,
    account_index: usize,
) -> Result<types::Balance, Box<dyn std::error::Error>> {
    let balance = client
        .get(&format!(
            "{}/balance?account_id={}",
            consts::MONZO_API,
            user.accounts[account_index].id
        ))
        .send()
        .await?
        .bytes()
        .await?;

    Ok(serde_json::from_slice(&balance)?)
}

/// The `balance` command
pub fn balance(
    user: &types::user::User,
    client: &reqwest::Client,
    command: &cli::Command,
) -> Result<(), Box<dyn std::error::Error>> {
    let account_index = if let Some(n) = command.uint_kwarg::<u64>("account") {
        let i = n? as usize;
        if user.accounts.len() >= i - 1 {
            Ok(i)
        } else {
            Err(error::BadArgumentError(format!(
                "the account index [{}] doest not exist for this user",
                i
            )))
        }?
    } else {
        0
    };
    let account_index = if let Some(i) = command.kwargs.get("account") {
        if let Ok(i) = i.parse::<usize>() {
            if user.accounts.len() >= i - 1 {
                i
            } else {
                return Err(error::BadArgumentError(format!(
                    "the account index [{}] doest not exist for this user",
                    i
                ))
                .into());
            }
        } else {
            return Err(error::InvalidArgumentError(format!(
                "kwarg `account` must be a valid integer, `{}` is not",
                i
            ))
            .into());
        }
    } else {
        0
    };

    let balance = pollster::block_on(get_balance(user, client, account_index))?;

    if command.args_set.contains("--detailed") || command.args_set.contains("-d") {
        println!(
            "BALANCE: {}\n\
            TOTAL BALANCE: {}",
            balance.balance_string(),
            balance.total_balance_string()
        );
    } else {
        println!("{}", balance.balance_string());
    };

    Ok(())
}
