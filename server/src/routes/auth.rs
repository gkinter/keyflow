use crate::{
    error::AppError,
    models::User,
    security::csrf::{ensure_csrf_cookie, expire_csrf_cookie, issue_csrf_cookie, verify_csrf},
    session::SessionClaims,
    state::{AppState, SharedState},
};
use axum::{
    async_trait,
    extract::{ConnectInfo, FromRef, FromRequestParts, Query, State},
    http::{request::Parts, HeaderMap},
    response::{IntoResponse, Redirect},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse, Url,
};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use tracing::{info, warn};
use std::net::{IpAddr, SocketAddr};

pub const SESSION_COOKIE: &str = "kf_session";
const OAUTH_STATE_COOKIE: &str = "kf_oauth_state";
const OAUTH_PKCE_COOKIE: &str = "kf_oauth_pkce";

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

#[derive(Serialize)]
pub struct SessionResponse {
    user: PublicUser,
    #[serde(rename = "csrfToken")]
    csrf_token: String,
}

#[derive(Serialize)]
pub struct PublicUser {
    id: Uuid,
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
    email: Option<String>,
    role: String,
}

pub async fn github_login(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let client_ip = resolve_client_ip(addr, &headers);
    let limiter = state.auth_rate_limiter.clone();
    limiter.check(client_ip).await?;

    let (auth_url, csrf_token, pkce_verifier) = build_auth_url(&state)?;
    let jar = jar
        .add(build_temp_cookie(OAUTH_STATE_COOKIE, csrf_token.secret(), state.config.cookie_secure))
        .add(build_temp_cookie(OAUTH_PKCE_COOKIE, pkce_verifier.secret(), state.config.cookie_secure));

    info!(client_ip = %client_ip, "redirecting user to GitHub OAuth");

    Ok((jar, Redirect::temporary(auth_url.as_ref())))
}

pub async fn github_callback(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
    Query(query): Query<OAuthCallback>,
) -> Result<impl IntoResponse, AppError> {
    let client_ip = resolve_client_ip(addr, &headers);
    let limiter = state.auth_rate_limiter.clone();
    limiter.check(client_ip).await?;

    let state_cookie = jar
        .get(OAUTH_STATE_COOKIE)
        .ok_or_else(|| AppError::BadRequest("missing oauth state".into()))?;
    let pkce_cookie = jar
        .get(OAUTH_PKCE_COOKIE)
        .ok_or_else(|| AppError::BadRequest("missing pkce verifier".into()))?;

    if state_cookie.value() != query.state {
        warn!("oauth state mismatch");
        return Err(AppError::BadRequest("invalid oauth state".into()));
    }

    let verifier = PkceCodeVerifier::new(pkce_cookie.value().to_string());

    let token = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(verifier)
        .request_async(&state.http_client)
        .await?;

    let jar = jar
        .remove(expire_cookie(OAUTH_STATE_COOKIE, state.config.cookie_secure, true))
        .remove(expire_cookie(OAUTH_PKCE_COOKIE, state.config.cookie_secure, true));

    let github_user = fetch_github_user(&state, token.access_token().secret()).await?;
    let user = upsert_user(&state, &github_user).await?;

    let session_token = state.session_signer.issue(user.id)?;
    let jar = jar.add(build_session_cookie(SESSION_COOKIE, &session_token, state.config.cookie_secure));
    let (jar, _csrf_token) = issue_csrf_cookie(jar, state.config.cookie_secure);

    info!(user_id = %user.id, login = %user.login, client_ip = %client_ip, "user authenticated via GitHub");

    Ok((jar, Redirect::temporary(&state.config.frontend_origin)))
}

pub async fn logout(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let client_ip = resolve_client_ip(addr, &headers);
    let limiter = state.auth_rate_limiter.clone();
    limiter.check(client_ip).await?;

    verify_csrf(&jar, &headers)?;
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        if let Ok(claims) = state.session_signer.verify(cookie.value()) {
            info!(user_id = %claims.sub, client_ip = %client_ip, "user logged out");
        }
    }

    let jar = jar
        .remove(expire_cookie(SESSION_COOKIE, state.config.cookie_secure, true))
        .remove(expire_csrf_cookie(state.config.cookie_secure));
    Ok((jar, axum::http::StatusCode::NO_CONTENT))
}

pub async fn current_session(
    current_user: CurrentUser,
    jar: CookieJar,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    let (user, _) = current_user.into_parts();
    let (jar, csrf_token) = ensure_csrf_cookie(jar, state.config.cookie_secure);
    Ok((
        jar,
        Json(SessionResponse {
            user: user.into(),
            csrf_token,
        }),
    ))
}

pub async fn me(current_user: CurrentUser) -> Json<PublicUser> {
    let (user, _) = current_user.into_parts();
    Json(user.into())
}

pub struct CurrentUser {
    user: User,
    client_ip: Option<std::net::IpAddr>,
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            login: value.login,
            name: value.name,
            avatar_url: value.avatar_url,
            email: value.email,
            role: value.role,
        }
    }
}

impl CurrentUser {
    pub fn require_role(&self, role: &str) -> Result<(), AppError> {
        if self.user.role == role {
            Ok(())
        } else {
            Err(AppError::Unauthorized)
        }
    }

    pub fn require_admin(&self) -> Result<(), AppError> {
        self.require_role("admin")
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn into_parts(self) -> (User, Option<IpAddr>) {
        (self.user, self.client_ip)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    SharedState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Unauthorized)?;
        let State(app_state) = State::<SharedState>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Internal)?;
        let socket_addr = parts
            .extensions
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(addr)| *addr);
        let client_ip = socket_addr.map(|addr| resolve_client_ip(addr, &parts.headers));
        authenticate_from_cookies(jar, &app_state, client_ip).await
    }
}

async fn authenticate_from_cookies(
    jar: CookieJar,
    state: &AppState,
    client_ip: Option<IpAddr>,
) -> Result<CurrentUser, AppError> {
    let cookie = jar.get(SESSION_COOKIE).ok_or(AppError::Unauthorized)?;
    let SessionClaims { sub, .. } = state.session_signer.verify(cookie.value())?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(sub)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if let Some(ip) = client_ip {
        info!(user_id = %user.id, login = %user.login, client_ip = %ip, "session validated");
    } else {
        info!(user_id = %user.id, login = %user.login, "session validated");
    }

    Ok(CurrentUser { user, client_ip })
}

fn resolve_client_ip(addr: SocketAddr, headers: &HeaderMap) -> IpAddr {
    if let Some(forwarded_for) = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
    {
        if let Some(ip_str) = forwarded_for.split(',').next().map(|s| s.trim()) {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }

    if let Some(forwarded) = headers
        .get("forwarded")
        .and_then(|value| value.to_str().ok())
    {
        for part in forwarded.split(';').flat_map(|segment| segment.split(',')) {
            let trimmed = part.trim();
            if let Some(rest) = trimmed.strip_prefix("for=") {
                let candidate = rest.trim_matches(|c| c == '"' || c == '[' || c == ']');
                if let Ok(ip) = candidate.parse::<IpAddr>() {
                    return ip;
                }
            }
        }
    }

    addr.ip()
}

fn build_temp_cookie(name: &'static str, value: &str, secure: bool) -> Cookie<'static> {
    let expires = OffsetDateTime::now_utc() + Duration::minutes(10);
    Cookie::build(name, value.to_string())
        .http_only(true)
        .same_site(if secure { SameSite::None } else { SameSite::Lax })
        .secure(secure)
        .path("/")
        .expires(expires)
        .finish()
}

fn expire_cookie(name: &'static str, secure: bool, http_only: bool) -> Cookie<'static> {
    let past = OffsetDateTime::now_utc() - Duration::hours(1);
    Cookie::build(name, "")
        .path("/")
        .http_only(http_only)
        .same_site(if secure { SameSite::None } else { SameSite::Lax })
        .secure(secure)
        .max_age(Duration::seconds(0))
        .expires(past)
        .finish()
}

fn build_session_cookie(name: &'static str, value: &str, secure: bool) -> Cookie<'static> {
    let expires = OffsetDateTime::now_utc() + Duration::hours(12);
    Cookie::build(name, value.to_string())
        .http_only(true)
        .same_site(if secure { SameSite::None } else { SameSite::Lax })
        .secure(secure)
        .path("/")
        .max_age(Duration::hours(12))
        .expires(expires)
        .finish()
}

fn build_auth_url(state: &SharedState) -> Result<(Url, CsrfToken, PkceCodeVerifier), AppError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    Ok((auth_url, csrf_token, pkce_verifier))
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: i64,
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
    email: Option<String>,
}

async fn fetch_github_user(state: &AppState, access_token: &str) -> Result<GitHubUser, AppError> {
    let response = state
        .http_client
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .header(USER_AGENT, "keyflow-server")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::BadRequest(format!(
            "github api error: {}",
            response.status()
        )));
    }

    Ok(response.json::<GitHubUser>().await?)
}

async fn upsert_user(state: &AppState, github: &GitHubUser) -> Result<User, AppError> {
    let normalized_email = github.email.as_ref().map(|email| email.to_lowercase());

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (github_id, login, name, avatar_url, email)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (github_id) DO UPDATE
        SET login = EXCLUDED.login,
            name = EXCLUDED.name,
            avatar_url = EXCLUDED.avatar_url,
            email = COALESCE(EXCLUDED.email, users.email),
            updated_at = now()
        RETURNING *
        "#,
    )
    .bind(github.id)
    .bind(&github.login)
    .bind(&github.name)
    .bind(&github.avatar_url)
    .bind(normalized_email.as_deref())
    .fetch_one(&state.pool)
    .await?;

    Ok(user)
}

