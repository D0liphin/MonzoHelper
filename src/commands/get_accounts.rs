use crate::*;

/// Get the associated accounts for a given `AccessTokenResponse`
/// `client` must be authorized
pub async fn get_accounts(
    client: &reqwest::Client,
) -> Result<Vec<types::Account>, types::error::AuthorizationError> {
    let response = client
        .get(&format!("{}/accounts", consts::MONZO_API))
        .send()
        .await?;

    #[derive(Deserialize, Serialize, Debug)]
    struct Accounts {
        accounts: Vec<types::Account>,
    }
    let accounts: Accounts = serde_json::from_slice(&response.bytes().await?)?;
    Ok(accounts.accounts)
}
