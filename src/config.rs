use std::path::Path;

#[derive(Clone)]
pub struct Config {
    pub port: String,
    pub service_url: ServiceUrl,
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
}

impl Config {
    fn init_dotenvy() {
        dotenvy::from_path(Path::new("../.env")).ok();
    }

    pub fn from_env() -> Self {
        Self::init_dotenvy();
        Self {
            port: dotenvy::var("GATEWAY_PORT").unwrap_or_else(|_| "8080".to_string()),
            service_url: ServiceUrl {
                admin_service_url: dotenvy::var("ADMIN_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("ADMIN_SERVICE_URL or BACKEND_URL is missing"),
                auth_service_url: dotenvy::var("AUTH_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("AUTH_SERVICE_URL or BACKEND_URL is missing"),
                contact_service_url: dotenvy::var("CONTACT_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("CONTACT_SERVICE_URL or BACKEND_URL is missing"),
                feed_service_url: dotenvy::var("FEED_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("FEED_SERVICE_URL or BACKEND_URL is missing"),
                me_service_url: dotenvy::var("ME_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("ME_SERVICE_URL or BACKEND_URL is missing"),
                polls_service_url: dotenvy::var("POLLS_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("POLLS_SERVICE_URL or BACKEND_URL is missing"),
                rooms_service_url: dotenvy::var("ROOMS_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("ROOMS_SERVICE_URL or BACKEND_URL is missing"),
                search_service_url: dotenvy::var("SEARCH_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("SEARCH_SERVICE_URL or BACKEND_URL is missing"),
                users_service_url: dotenvy::var("USERS_SERVICE_URL")
                    .or_else(|_| dotenvy::var("BACKEND_URL"))
                    .expect("USERS_SERVICE_URL or BACKEND_URL is missing"),
                frontend_url: dotenvy::var("FRONTEND_URL")
                    .unwrap_or_else(|_| "http://localhost:5173".to_string()),
                redis_url: dotenvy::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://redis:6379".to_string()),
            },
        }
    }
}
