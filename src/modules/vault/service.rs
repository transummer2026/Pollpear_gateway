//! Reading the gateway's configuration out of Vault at boot.

use std::{error::Error, time::Duration};

use vaultrs::{
    auth::approle,
    client::{Client, VaultClient},
    kv2,
};

use crate::{clients::create_client_vault, config::{ServiceUrl, Settings}};

use super::{App, RateLimit, Redis, S3, Session, Ws};

/// Bootstrap attempts before giving up and aborting startup.
const MAX_ATTEMPTS: u32 = 3;

/// Logs into Vault, reads every secret and assembles the runtime configuration.
///
/// Runs once, before the router exists. Retried up to [`MAX_ATTEMPTS`] times
/// with a short backoff because on a cold deploy the gateway can easily come up
/// before Vault is ready to answer; the whole sequence — login included — is
/// retried, since a failure part-way through may have been an expired token
/// rather than an unreachable server.
///
/// All ten `*_service_url` fields are filled from the single `backend_url`
/// secret: the back end is one service today, and the split is kept in the type
/// so moving a domain out later is a change here and nowhere else.
///
/// # Errors
///
/// The last attempt's error, whatever it was: unreachable Vault, bad AppRole
/// credentials, a missing secret path, or a non-numeric TTL or quota. Startup
/// then fails — a gateway with no configuration could not authenticate anyone.
pub async fn bootstrap(
    vault_url: &str,
    role_id: String,
    secret_id: String,
) -> Result<(ServiceUrl, Settings), Box<dyn Error>> {
    for attempt in 1..=MAX_ATTEMPTS {
        let result: Result<(ServiceUrl, Settings ), Box<dyn Error>> = async {
            let client = create_client_vault(vault_url)?;
            let client = approle_login(client, role_id.clone(), secret_id.clone()).await?;

            let app_secret = read_app(&client).await?;
            let redis_secret = read_redis(&client).await?;
            let session_secret = read_session(&client).await?;
            let s3_secret = read_s3(&client).await?;
            let ws_secret = read_ws(&client).await?;
            let rate_limit_secret = read_rate_limit(&client).await?;

            // Parsed here so a malformed secret aborts startup instead of
            // surfacing on the first request that needs the value.
            let settings = Settings {
                session_ttl: session_secret.ttl.parse()?,
                rate_limit: rate_limit_secret.limit.parse()?,
                rate_limit_window_seconds: rate_limit_secret.window_seconds.parse()?,
            };

            let service_url = ServiceUrl {
                admin_service_url: app_secret.backend_url.clone(),
                api_service_url: app_secret.backend_url.clone(),
                auth_service_url: app_secret.backend_url.clone(),
                contact_service_url: app_secret.backend_url.clone(),
                feed_service_url: app_secret.backend_url.clone(),
                me_service_url: app_secret.backend_url.clone(),
                polls_service_url: app_secret.backend_url.clone(),
                rooms_service_url: app_secret.backend_url.clone(),
                search_service_url: app_secret.backend_url.clone(),
                users_service_url: app_secret.backend_url.clone(),
                // Normalised to exactly one trailing slash, so the query string
                // appended at handshake time lands on a well-formed URL.
                ws_url: format!(
                    "{}{}/",
                    to_ws_url(&app_secret.backend_url.clone())?,
                    ws_secret.path.trim_end_matches('/')
                ),
                frontend_url: app_secret.frontend_url,
                redis_url: format!(
                    "redis://:{}@{}:{}",
                    redis_secret.password, redis_secret.host, redis_secret.port
                ),
                garage_url: s3_secret.url,
                garage_uploads_host: s3_secret.uploads_host,
            };

            Ok((service_url, settings))
        }
        .await;

        match result {
            Ok(value) => return Ok(value),
            Err(err) if attempt < MAX_ATTEMPTS => {
                tracing::warn!(
                    "vault bootstrap failed (attempt {attempt}/{MAX_ATTEMPTS}): {err}"
                );
                // Linear backoff: 200ms, then 400ms. Enough to ride out a Vault
                // still coming up, short enough not to stall the deployment.
                tokio::time::sleep(Duration::from_millis(200 * attempt as u64)).await;
            }
            Err(err) => return Err(err),
        }
    }

    unreachable!("loop always returns on the last attempt")
}

/// Turns an `http(s)://…` backend URL into its `ws(s)://…` equivalent.
///
/// Stripping `http` and prepending `ws` preserves the `s`, so TLS carries over
/// from the backend URL without a separate secret to keep in sync.
///
/// # Errors
///
/// If the URL does not start with `http`, which would otherwise produce a
/// silently unusable WebSocket target.
fn to_ws_url(http_url: &str) -> Result<String, Box<dyn Error>> {
    let Some(url) = http_url.strip_prefix("http") else {
        return Err(format!("invalid backend_url scheme: {http_url}").into());
    };
    Ok(format!("ws{url}"))
}

/// Authenticates with AppRole and returns the client carrying the token.
///
/// AppRole rather than a static token so the credentials can be rotated without
/// rebuilding the gateway. The token is set on the client built by
/// [`create_client_vault`], which starts out empty.
pub async fn approle_login(
    mut client: VaultClient,
    role_id: String,
    secret_id: String,
) -> Result<VaultClient, Box<dyn Error>> {
    let auth_info = approle::login(&client, "approle", &role_id, &secret_id).await?;
    client.set_token(&auth_info.client_token);

    Ok(client)
}

/// Reads `secret/pollpear/app` into [`App`].
pub async fn read_app(client: &VaultClient) -> Result<App, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/app").await?;

    Ok(secret)
}

/// Reads `secret/pollpear/redis` into [`Redis`].
pub async fn read_redis(client: &VaultClient) -> Result<Redis, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/redis").await?;

    Ok(secret)
}

/// Reads `secret/pollpear/session` into [`Session`].
pub async fn read_session(client: &VaultClient) -> Result<Session, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/session").await?;

    Ok(secret)
}

/// Reads `secret/pollpear/s3` into [`S3`].
pub async fn read_s3(client: &VaultClient) -> Result<S3, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/s3").await?;

    Ok(secret)
}

/// Reads `secret/pollpear/ws` into [`Ws`].
pub async fn read_ws(client: &VaultClient) -> Result<Ws, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/ws").await?;

    Ok(secret)
}

/// Reads `secret/pollpear/rate_limit` into [`RateLimit`].
pub async fn read_rate_limit(client: &VaultClient) -> Result<RateLimit, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/rate_limit").await?;

    Ok(secret)
}
