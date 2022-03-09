use crate::types::*;
use crate::*;

/// get a list of `PartialTransaction` objects from `since` until `before`
pub async fn get_transactions(
    user: &types::user::User,
    client: &reqwest::Client,
    since: Option<time::Time>,
    before: Option<time::Time>,
    account_index: usize,
) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
    #[derive(Serialize)]
    struct QueryString<'a> {
        account_id: &'a str,
        #[serde(rename(serialize = "expand[]"))]
        expand: &'a str,
        since: Option<time::Time>,
        before: Option<time::Time>,
    }

    let transactions = client
        .get(&format!(
            "{}/transactions?{}",
            consts::MONZO_API,
            serde_qs::to_string(&QueryString {
                account_id: &user.accounts[account_index].id,
                expand: "merchant",
                since,
                before,
            })
            .unwrap()
        ))
        .send()
        .await?
        .bytes()
        .await?;

    #[derive(Deserialize)]
    struct Transactions {
        transactions: Vec<Transaction>,
    }
    println!("{}", String::from_utf8(transactions.to_vec()).unwrap());
    let transactions: Transactions = serde_json::from_slice(&transactions)?;
    println!("{:?}", transactions.transactions);
    Ok(transactions.transactions)
}
