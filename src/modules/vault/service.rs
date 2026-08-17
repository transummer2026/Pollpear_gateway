use std::{error::Error, time::Duration};

use vaultrs::{
    auth::approle,
    client::{Client, VaultClient},
    kv2,
};

use crate::{clients::create_client_vault, config::ServiceUrl};

use super::{App, Redis, S3, Session};

const MAX_ATTEMPTS: u32 = 3;

pub async fn bootstrap(
    vault_url: &str,
    role_id: String,
    secret_id: String,
) -> Result<(ServiceUrl, i64), Box<dyn Error>> {
    for attempt in 1..=MAX_ATTEMPTS {
        let result: Result<(ServiceUrl, i64), Box<dyn Error>> = async {
            let client = create_client_vault(vault_url)?;
            let client = approle_login(client, role_id.clone(), secret_id.clone()).await?;

            let app_secret = read_app(&client).await?;
            let redis_secret = read_redis(&client).await?;
            let session_secret = read_session(&client).await?;
            let s3_secret = read_s3(&client).await?;

            let ttl: i64 = session_secret.ttl.parse()?;

            let service_url = ServiceUrl {
                admin_service_url: app_secret.backend_url.clone(),
                auth_service_url: app_secret.backend_url.clone(),
                contact_service_url: app_secret.backend_url.clone(),
                feed_service_url: app_secret.backend_url.clone(),
                me_service_url: app_secret.backend_url.clone(),
                polls_service_url: app_secret.backend_url.clone(),
                rooms_service_url: app_secret.backend_url.clone(),
                search_service_url: app_secret.backend_url.clone(),
                users_service_url: app_secret.backend_url.clone(),
                ws_url: to_ws_url(&app_secret.backend_url.clone())?,
                frontend_url: app_secret.frontend_url,
                redis_url: format!(
                    "redis://:{}@{}:{}",
                    redis_secret.password, redis_secret.host, redis_secret.port
                ),
                garage_url: s3_secret.url,
                garage_uploads_host: s3_secret.uploads_host,
            };

            Ok((service_url, ttl))
        }
        .await;

        match result {
            Ok(value) => return Ok(value),
            Err(err) if attempt < MAX_ATTEMPTS => {
                tracing::warn!(
                    "vault bootstrap failed (attempt {attempt}/{MAX_ATTEMPTS}): {err}"
                );
                tokio::time::sleep(Duration::from_millis(200 * attempt as u64)).await;
            }
            Err(err) => return Err(err),
        }
    }

    unreachable!("loop always returns on the last attempt")
}

fn to_ws_url(http_url: &str) -> Result<String, Box<dyn Error>> {
    let Some(url) = http_url.strip_prefix("http") else {
        return Err(format!("invalid backend_url scheme: {http_url}").into());
    };
    Ok(format!("ws{url}"))
}

pub async fn approle_login(
    mut client: VaultClient,
    role_id: String,
    secret_id: String,
) -> Result<VaultClient, Box<dyn Error>> {
    let auth_info = approle::login(&client, "approle", &role_id, &secret_id).await?;
    client.set_token(&auth_info.client_token);

    Ok(client)
}

pub async fn read_app(client: &VaultClient) -> Result<App, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/app").await?;

    Ok(secret)
}

pub async fn read_redis(client: &VaultClient) -> Result<Redis, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/redis").await?;

    Ok(secret)
}

pub async fn read_session(client: &VaultClient) -> Result<Session, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/session").await?;

    Ok(secret)
}

pub async fn read_s3(client: &VaultClient) -> Result<S3, Box<dyn Error>> {
    let secret = kv2::read(client, "secret", "pollpear/s3").await?;

    Ok(secret)
}
