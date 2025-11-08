use crate::error::AppError;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::env;
use tracing::warn;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub session_signing_keys: Vec<String>,
    pub app_base_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub frontend_origin: String,
    pub cookie_secure: bool,
    pub port: u16,
    pub auth_rate_limit_per_minute: u32,
    pub auth_rate_limit_burst: u32,
    pub allow_insecure_cookies: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|_| AppError::Config("DATABASE_URL missing".into()))?;
        let session_signing_keys = load_session_keys()?;
        let app_base_url = env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let frontend_origin =
            env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
        let github_client_id =
            env::var("GITHUB_CLIENT_ID").map_err(|_| AppError::Config("GITHUB_CLIENT_ID missing".into()))?;
        let github_client_secret =
            env::var("GITHUB_CLIENT_SECRET").map_err(|_| AppError::Config("GITHUB_CLIENT_SECRET missing".into()))?;
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8080);
        let auth_rate_limit_per_minute = env::var("AUTH_RATE_LIMIT_PER_MINUTE")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(60);
        let auth_rate_limit_burst = env::var("AUTH_RATE_LIMIT_BURST")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(10);

        let cookie_secure = app_base_url.starts_with("https://");
        let allow_insecure_cookies = env::var("ALLOW_INSECURE_COOKIES")
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);

        if !cookie_secure && !allow_insecure_cookies {
            return Err(AppError::Config(
                "APP_BASE_URL must use https when issuing SameSite=None cookies; set ALLOW_INSECURE_COOKIES=true only for local development"
                    .into(),
            ));
        }

        Ok(Self {
            database_url,
            session_signing_keys,
            app_base_url,
            github_client_id,
            github_client_secret,
            frontend_origin,
            cookie_secure,
            port,
            auth_rate_limit_per_minute,
            auth_rate_limit_burst,
            allow_insecure_cookies,
        })
    }

    pub fn oauth_client(&self) -> Result<BasicClient, AppError> {
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".into())
            .map_err(|e| AppError::Config(format!("invalid auth url: {}", e)))?;
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".into())
            .map_err(|e| AppError::Config(format!("invalid token url: {}", e)))?;
        let redirect = RedirectUrl::new(format!("{}/auth/github/callback", self.app_base_url))
            .map_err(|e| AppError::Config(format!("invalid redirect url: {}", e)))?;

        Ok(
            BasicClient::new(
                ClientId::new(self.github_client_id.clone()),
                Some(ClientSecret::new(self.github_client_secret.clone())),
                auth_url,
                Some(token_url),
            )
            .set_redirect_uri(redirect),
        )
    }
}

fn load_session_keys() -> Result<Vec<String>, AppError> {
    let keys_env = env::var("SESSION_SIGNING_KEYS").ok();
    let used_legacy = keys_env.is_none();
    let mut keys: Vec<String> = if let Some(raw) = keys_env {
        raw.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![env::var("SESSION_SIGNING_KEY")
            .map_err(|_| AppError::Config("SESSION_SIGNING_KEY or SESSION_SIGNING_KEYS missing".into()))?]
    };

    if keys.is_empty() {
        return Err(AppError::Config("no session signing keys provided".into()));
    }

    for key in &keys {
        if key.len() < 32 {
            return Err(AppError::Config(
                "all session signing keys must be at least 32 characters for HMAC signing".into(),
            ));
        }
    }

    if used_legacy {
        warn!("SESSION_SIGNING_KEY is deprecated; prefer SESSION_SIGNING_KEYS for key rotation");
    }

    if keys.len() == 1 {
        warn!("only one session signing key configured; provide multiple values in SESSION_SIGNING_KEYS to enable seamless rotation");
    }

    Ok(keys)
}

