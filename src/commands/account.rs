use crate::*;

/// The `account` command
pub fn account(user: &types::user::User, command: &cli::Command) -> Result<(), Box<dyn std::error::Error>> {
    let detailed = command.args_set.contains("--detailed") || command.args_set.contains("-d");
    for (account_index, account) in user.accounts.iter().enumerate() {
        let details = if detailed {
            let mut details = format!(
                "\n\
                ACCOUNT INDEX: [{}]\n\
                CREATED: {}\n\
                CURRENCY: {}\n\
                ID: {}\n\n\
                ACCOUNT NUMBER: {}\n\
                SORT CODE: {}\n\n\
                OWNERS: ",
                account_index,
                account.created,
                account.currency,
                account.id,
                account.account_number,
                account.sort_code,
            );
            for owner in &account.owners {
                details.push_str(&format!(
                    "{} [{}]\n        ",
                    owner.preferred_name, owner.user_id
                ))
            }
            details
        } else {
            let mut details = format!(
                "\n\
                ACCOUNT INDEX: [{}]\n\
                ACCOUNT NUMBER: {}\n\
                SORT CODE: {}\n\
                OWNERS: ",
                account_index, account.account_number, account.sort_code,
            );
            for owner in &account.owners {
                details.push_str(&format!("{}\n        ", owner.preferred_name))
            }
            details
        };
        println!("{}", details);
    }

    Ok(())
}
