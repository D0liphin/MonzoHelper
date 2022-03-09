use std::io::Write;

use crate::types::error::AuthorizationError;
use crate::types::*;
use crate::util::FmtCurrencyOptions;
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
    let transactions: Transactions = serde_json::from_slice(&transactions)?;
    Ok(transactions.transactions)
}

pub fn transactions(
    user: &user::User,
    client: &reqwest::Client,
    command: &cli::Command,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = time::Time::now();
    let before = if let Some(time_str) = command.kwargs.get("before") {
        Some(time::Time::try_parse_str(time_str)?)
    } else {
        None
    };
    let earliest_since = now.add(&(chrono::Duration::days(-90) + chrono::Duration::seconds(100)));
    let since = if let Some(time_str) = command.kwargs.get("since") {
        let since = time::Time::try_parse_str(time_str)?;
        if since < earliest_since {
            if user.access_token.created < now.add(&chrono::Duration::seconds(-60 * 4 - 30)) {
                return Err(AuthorizationError::Custom(
                    "cannot access more than 90d of transactions, unles the user has been authorized \
                    in the last 5 minutes\nrun `auth` and try again".to_owned()
                ).into());
            }
        }
        Some(since)
    } else {
        Some(earliest_since)
    };
    let account_index = util::get_account_index(user, command)?;
    let output_type = if let Some(output_type_str) = command.kwargs.get("format") {
        types::OutputType::from_str(output_type_str)?
    } else {
        types::OutputType::Display
    };

    let transactions =
        pollster::block_on(get_transactions(user, client, since, before, account_index))?;

    let detailed = command.args_set.contains("--detailed") || command.args_set.contains("-d");

    // Some crap code
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    fn fmt_serializable<T: serde::Serialize, C: Fn(&T) -> Result<String>>(
        serializable: &T,
        output_type: OutputType,
        display: C,
    ) -> Result<String> {
        match output_type {
            OutputType::Display => display(serializable),
            OutputType::Json => Ok(serde_json::to_string_pretty(serializable)?),
            OutputType::Csv => Ok(util::serde_csv::to_string(serializable)?),
        }
    }
    let fmt_transaction = |transaction: &Transaction| -> Result<String> {
        if detailed {
            fmt_serializable(&transaction, output_type, |transaction| {
                Ok(format!(
                    "\
                    ACCOUNT_BALANCE : {}\n\
                    AMOUNT          : {}\n\
                    TIME            : {}\n\
                    CURRENCY        : {}\n\
                    DESCRIPTION     : \"{}\"\n\
                    MERCHANT        : {}\
                    ",
                    util::unwrap_to_string(&transaction.account_balance, "NULL"),
                    transaction.amount,
                    transaction.created,
                    transaction.currency,
                    transaction.description,
                    match &transaction.merchant {
                        Some(merchant) => format!(
                            "\n\t\t\
                                ADDRESS  : {}\n\t\t\
                                EMOJI    : {}\n\t\t\
                                NAME     : {}\n\t\t\
                                CATEGORY : {}\
                                ",
                            {
                                format!(
                                    "\n\t\t\t  \
                                        {}\n\t\t\t  \
                                        {}\n\t\t\t  \
                                        {}\
                                    ",
                                    merchant.address.address,
                                    merchant.address.postcode,
                                    merchant.address.city,
                                )
                            },
                            merchant.emoji,
                            merchant.name,
                            merchant.category,
                        ),
                        None => "NULL".to_string(),
                    }
                ))
            })
        } else {
            #[derive(Serialize)]
            struct MinimalTransaction<'a> {
                amount: i32,
                currency: &'a str,
                notes: String,
            }

            let transaction_was_declined = transaction.decline_reason.is_some();
            let transaction = MinimalTransaction {
                amount: transaction.amount,
                currency: &transaction.currency,
                notes: if let Some(merchant) = &transaction.merchant {
                    format!("at '{}'", merchant.name)
                } else {
                    if let Some(counterparty) = &transaction.counterparty {
                        let mut s = counterparty.name.clone().unwrap_or("".to_owned());
                        if let Some(notes) = transaction.metadata.get("notes") {
                            s.extend([" \"", notes, "\""]);
                        }
                        s
                    } else {
                        String::from("no notes")
                    }
                },
            };

            fn strikethrough(s: &str) -> String {
                cli::AnsiStringBuilder::new()
                    .set_strikethrough(true)
                    .push_str(s)
                    .build()
            }
            fmt_serializable(&transaction, output_type, |transaction| {
                let s = format!(
                    "{} {}",
                    util::fmt_currency(
                        transaction.amount,
                        transaction.currency,
                        &FmtCurrencyOptions {
                            include_positive_sign: true,
                            colored: true,
                        }
                    ),
                    if transaction_was_declined {
                        strikethrough(&transaction.notes)
                    } else {
                        transaction.notes.clone()
                    }
                );
                Ok(if transaction_was_declined {
                    strikethrough(&s)
                } else {
                    s
                })
            })
        }
    };

    let mut output = vec![];
    for transaction in transactions.iter() {
        output.push(fmt_transaction(transaction)?);
    }
    let output = match output_type {
        OutputType::Json => format!("[\n{}\n]", output.join(",\n")),
        OutputType::Csv => unimplemented!(),
        OutputType::Display => output.join("\n"),
    };

    std::io::stdout().write(&output.as_bytes())?;
    Ok(())
}
