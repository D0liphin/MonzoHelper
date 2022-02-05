use crate::types::*;
use crate::*;

/// A list of acccounts associated with a given user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub accounts: Vec<Account>,
    pub access_token: String,
    pub user_id: String,
}

impl User {
    pub fn new(access_token_response: AccessTokenResponse, accounts: Vec<Account>) -> Self {
        Self {
            accounts,
            user_id: access_token_response.user_id,
            access_token: access_token_response.access_token,
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
        client::new_client_with_authorization_header(&self.access_token)
    }   
}
