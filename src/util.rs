use crate::types::*;
use crate::*;

pub fn get_account_index(
    user: &user::User,
    command: &cli::Command,
) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(if let Some(n) = command.uint_kwarg::<u64>("account") {
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
    })
}

pub struct FmtCurrencyOptions {
    pub include_positive_sign: bool,
    pub colored: bool,
}

impl Default for FmtCurrencyOptions {
    fn default() -> Self {
        Self {
            include_positive_sign: false,
            colored: false,
        }
    }
}

pub fn fmt_currency(amount: i32, currency: &str, options: &FmtCurrencyOptions) -> String {
    let amount_is_negative = amount.is_negative();
    let amount = amount.abs();
    let fmt_currency = |symbol: &str, delimeter: &str, minor_per_major: i32| {
        let major_units = amount / minor_per_major;
        let minor_units = amount - major_units * minor_per_major;
        format!(
            "{}{}{}{}{}",
            if amount_is_negative {
                "-"
            } else {
                if options.include_positive_sign {
                    "+"
                } else {
                    ""
                }
            },
            symbol,
            major_units,
            delimeter,
            minor_units,
        )
    };
    let string = match currency {
        "GBP" => fmt_currency("£", ".", 100),
        _ => format!("{} {}", amount.to_string(), currency),
    };
    if options.colored {
        if amount_is_negative {
            cli::AnsiStringBuilder::new()
                .set_foreground_color(255, 20, 20)
                .push_str(&string)
                .build()
        } else {
            cli::AnsiStringBuilder::new()
                .set_foreground_color(20, 255, 20)
                .push_str(&string)
                .build()
        }
    } else {
        string
    }
}

pub fn unwrap_to_string<'a, T: std::fmt::Display>(option: &'a Option<T>, or: &'a str) -> String {
    match option {
        Some(t) => t.to_string(),
        None => or.to_owned(),
    }
}

pub mod serde_csv {
    use std::io;

    pub fn to_string<T: serde::Serialize>(record: T) -> Result<String, io::Error> {
        let mut buf = io::BufWriter::new(Vec::new());
        {
            let mut csv_writer = csv::Writer::from_writer(&mut buf);
            csv_writer.serialize(&record)?;
            csv_writer.flush()?;
        }
        Ok(match String::from_utf8(buf.into_inner()?) {
            Ok(s) => s,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "csv serialization error -- cannot encode as utf8",
                ))
            }
        })
    }
}
