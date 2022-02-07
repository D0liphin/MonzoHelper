use crate::*;
use std::io::Write;
use std::process::Command;
use tokio::sync::mpsc;
use warp::Filter;

/// The `auth` command, aksi returns the `User` object and an authorized client
pub async fn auth() -> Result<(types::user::User, reqwest::Client), types::error::AuthorizationError>
{
    let access_token_response = get_access_token(&reqwest::Client::new()).await?;
    let client = client::new_client_with_authorization_header(&access_token_response.access_token);

    print!("\nAllow access to your data on the monzo app, then press enter...");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut String::new());
    println!();

    let user = types::user::User::new_from_access_token(&client, access_token_response).await?;
    let _ = user_file::update_user_file(&user);

    Ok((user, client))
}

/// Authorizes this application to connect to a monzo account, returns the access token
/// for this session
async fn get_access_token(
    client: &reqwest::Client,
) -> Result<types::AccessTokenResponse, types::error::AuthorizationError> {
    redirect_user_to_monzo()?;
    let authorization_code = acquire_authorization_code().await?;
    acquire_access_token(client, authorization_code).await
}

/// Opens a webpage redirecting the user to auth.monzo.com and prints the link in case
/// the user does not have xdg-open
fn redirect_user_to_monzo() -> Result<(), types::error::AuthorizationError> {
    let redirect_link = format!(
        "https://auth.monzo.com/?{}",
        serde_qs::to_string(&types::RedirectToMonzo::default())?,
    );
    let _ = Command::new("xdg-open").args([&redirect_link]).output();
    println!(
        "If the page does not open, use this link manually \n[{}]",
        redirect_link
    );

    Ok(())
}

/// Acquire the authorization code
async fn acquire_authorization_code() -> Result<String, tokio::task::JoinError> {
    /// The message displayed when the user enters a bad URI
    const BAD_URI_MESSAGE: &'static str = "There's nothing to see here (-_-)\n\nERROR: Bad URI";
    /// The message displayed when the user is successfully redirected to a well-formed URI
    const SUCCESS_MESSAGE: &'static str = "Successfully updated authorization code \\(^-^)/";

    // We're using mpsc instead of oneshot because with oneshot, we need to drop
    // the sender, whcih means we can't guarantee (at compile time) that it will live long
    // enough );
    let (kill_signal_tx, mut kill_signal_rx) = mpsc::unbounded_channel::<()>();
    let (authorization_code_tx, mut authorization_code_rx) = mpsc::unbounded_channel();
    let router  // br
        = warp::query()
        .map(move |query: types::MonzoRedirectBundle| {
            let _ = authorization_code_tx.send(query.code);
            let _ = kill_signal_tx.send(());
            SUCCESS_MESSAGE
        })
        .or(warp::any().map(|| BAD_URI_MESSAGE));

    // Start server with graceful shutdown and await it's completion
    let (_, server) = warp::serve(router) // br
        .bind_with_graceful_shutdown(consts::ADDR, async move {
            kill_signal_rx.recv().await;
        });

    // Start a task that waits to receive the authorization code
    let authorization_code_join_handle = tokio::task::spawn(async move {
        let authorization_code = authorization_code_rx
            .recv()
            .await
            .expect("impossible error");
        authorization_code
    });
    tokio::task::spawn(server).await?;

    // return the authorization code
    authorization_code_join_handle.await
}

/// Acquire a new access token
async fn acquire_access_token(
    client: &reqwest::Client,
    authorization_code: String,
) -> Result<types::AccessTokenResponse, types::error::AuthorizationError> {
    let access_token_request = types::AccessTokenRequest::new(authorization_code);
    let response = client
        .post(concatcp!(consts::MONZO_API, "/oauth2/token"))
        .form(&access_token_request)
        .send()
        .await?;

    let access_token_response =
        serde_json::from_slice::<types::AccessTokenResponse>(&response.bytes().await?)?;

    Ok(access_token_response)
}
