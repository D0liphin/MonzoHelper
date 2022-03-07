use crate::types::*;

/// A list of acccounts associated with a given user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub accounts: Vec<Account>,
    pub access_token: AccessToken,
    pub user_id: String,
}

impl User {
    pub fn new(access_token_response: AccessTokenResponse, accounts: Vec<Account>) -> Self {
        Self {
            accounts,
            user_id: access_token_response.user_id,
            access_token: AccessToken {
                token: access_token_response.access_token,
                expires: Time::now().add(&chrono::Duration::seconds(
                    access_token_response.expires_in as _,
                )),
            },
        }
    }

    /// Create a new `User` from a specified `AccessToken`
    pub async fn new_from_access_token(
        client: &reqwest::Client,
        access_token_response: AccessTokenResponse,
    ) -> Result<Self, types::error::AuthorizationError> {
        let accounts = commands::get_accounts(client).await?;
        Ok(Self::new(access_token_response, accounts))
    }

    /// Creates an authorized `Client` from this `User` object
    pub fn create_authorized_client(&self) -> reqwest::Client {
        client::new_client_with_authorization_header(&self.access_token.token)
    }
}

/// Contains an access token as well as the timestamp at which the access token expires
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub expires: Time,
}
