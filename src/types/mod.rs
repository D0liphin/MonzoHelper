use std::collections::HashMap;

use crate::types::time::Time;
use crate::*;
use crate::util::FmtCurrencyOptions;

pub mod error;
pub mod time;
pub mod user;

/// The query for redirecting the user to auth.monzo.com
#[derive(Serialize, Deserialize, Debug)]
pub struct RedirectToMonzo {
    pub redirect_uri: String,
    pub client_id: String,
    pub response_type: String,
    pub state: String,
}

impl Default for RedirectToMonzo {
    fn default() -> Self {
        Self {
            client_id: consts::CLIENT_ID.to_owned(),
            redirect_uri: consts::REDIRECT_URI.to_owned(),
            response_type: "code".to_owned(),
            state: consts::STATE_TOKEN.to_owned(),
        }
    }
}

/// Request for an access token (using an authorization code)
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: String,
}

impl AccessTokenRequest {
    /// Creates a new access token requestfrom a given authorization
    /// code
    pub fn new(authorization_code: String) -> Self {
        Self {
            grant_type: "authorization_code".to_owned(),
            client_id: consts::CLIENT_ID.to_owned(),
            client_secret: consts::CLIENT_SECRET.to_owned(),
            redirect_uri: consts::REDIRECT_URI.to_owned(),
            code: authorization_code,
        }
    }
}

/// The authorization code and state token returned after authorizing through
/// monzo
#[derive(Deserialize, Default, Clone)]
pub struct MonzoRedirectBundle {
    pub code: String,
    pub state: String,
}

/// The response to a request for an access token
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub client_id: String,
    pub expires_in: u32,
    #[serde(default)]
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub closed: bool,
    pub created: Time, // TODO: change to a time type
    pub description: String,
    pub currency: String,
    pub country_code: String,
    pub owners: Vec<Owner>,
    pub account_number: String,
    pub sort_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
    pub user_id: String,
    pub preferred_name: String,
    pub preferred_first_name: String,
}

/// Return type of a query of a user's balance
/// TODO: update with local fields
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Balance {
    pub balance: i32,
    #[serde(default)]
    pub total_balance: i32,
    #[serde(default)]
    pub balance_including_flexible_savings: i32,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub spend_today: i32,
}

impl Balance {
    /// Returns a nicely formatted string for this balance (only works for GBP)
    pub fn balance_string(&self) -> String {
        self.prettify_minor_currency_units(self.balance)
    }

    /// Returns a nicely formatted string for this total_balance (only works for GBP)
    pub fn total_balance_string(&self) -> String {
        self.prettify_minor_currency_units(self.total_balance)
    }

    /// Converts an integer of minor currency units to a string that may contain
    /// some delimeter to separate major and minor units with a currency symbol
    fn prettify_minor_currency_units(&self, amount: i32) -> String {
        util::fmt_currency(amount, &self.currency, &FmtCurrencyOptions::default())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub account_balance: Option<i32>,
    pub amount: i32,
    pub created: Time,
    pub currency: String,
    pub description: String,
    pub id: String,
    pub merchant: Option<Merchant>,
    pub counterparty: Option<Counterparty>,
    pub decline_reason: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Merchant {
    pub address: Address,
    pub created: Time,
    pub group_id: String,
    pub id: String,
    pub logo: String,
    pub emoji: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Counterparty {
    pub account_number: Option<String>,
    pub name: Option<String>,
    pub sort_code: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub postcode: String,
    pub region: String,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputType {
    Json,
    Csv,
    Display,
}

impl OutputType {
    pub fn from_str(s: &str) -> Result<Self, error::InvalidArgumentError> {
        Ok(match s {
            "json" => Self::Json,
            "csv" => Self::Csv,
            "display" => Self::Display,
            _ => {
                return Err(error::InvalidArgumentError(format!(
                    "`{}` is not a valid output format",
                    s
                )))
            }
        })
    }
}
