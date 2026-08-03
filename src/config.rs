use std::path::Path;

#[derive(Clone)]
pub struct Config {
    pub vault_url: String,
    pub vault_role: String,
    pub vault_secret: String,
}

#[derive(Clone)]
pub struct ServiceUrl {
    pub admin_service_url: String,
    pub auth_service_url: String,
    pub contact_service_url: String,
    pub feed_service_url: String,
    pub me_service_url: String,
    pub polls_service_url: String,
    pub rooms_service_url: String,
    pub search_service_url: String,
    pub users_service_url: String,
    pub frontend_url: String,
    pub redis_url: String,
    pub garage_url: String,
    pub garage_uploads_host: String,
}

impl Config {
    fn init_dotenvy() {
        dotenvy::from_path(Path::new("../.env")).ok();
    }

    pub fn from_env() -> Self {
        Self::init_dotenvy();
        Self {
            vault_url: dotenvy::var("VAULT_ADDR")
                .unwrap_or_else(|_| "VAULT_ADDR is missing".to_string()),
            vault_role: dotenvy::var("ROLE_ID")
                .unwrap_or_else(|_| "ROLE_ID is missing".to_string()),
            vault_secret: dotenvy::var("SECRET_ID")
                .unwrap_or_else(|_| "SECRET_ID is missing".to_string())
                .to_string(),
        }
    }
}
