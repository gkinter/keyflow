use crate::error::AppError;
use axum::http::HeaderMap;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use subtle::ConstantTimeEq;
use time::{Duration, OffsetDateTime};

pub const CSRF_COOKIE: &str = "kf_csrf";
pub const CSRF_HEADER: &str = "x-csrf-token";
const CSRF_TOKEN_LEN: usize = 32;

pub fn issue_csrf_cookie(jar: CookieJar, secure: bool) -> (CookieJar, String) {
    let token = generate_csrf_token();
    let jar = jar.add(build_csrf_cookie(&token, secure));
    (jar, token)
}

pub fn ensure_csrf_cookie(jar: CookieJar, secure: bool) -> (CookieJar, String) {
    if let Some(cookie) = jar.get(CSRF_COOKIE) {
        return (jar, cookie.value().to_string());
    }

    issue_csrf_cookie(jar, secure)
}

pub fn verify_csrf(jar: &CookieJar, headers: &HeaderMap) -> Result<(), AppError> {
    let header_value = headers
        .get(CSRF_HEADER)
        .ok_or_else(|| AppError::Forbidden("missing csrf header".into()))?
        .to_str()
        .map_err(|_| AppError::Forbidden("invalid csrf header".into()))?;

    let cookie = jar
        .get(CSRF_COOKIE)
        .ok_or_else(|| AppError::Forbidden("missing csrf cookie".into()))?;

    if cookie.value().as_bytes().ct_eq(header_value.as_bytes()).into() {
        Ok(())
    } else {
        Err(AppError::Forbidden("csrf token mismatch".into()))
    }
}

pub fn build_csrf_cookie(value: &str, secure: bool) -> Cookie<'static> {
    let mut builder = Cookie::build(CSRF_COOKIE, value.to_string())
        .path("/")
        .same_site(same_site_policy(secure))
        .secure(secure);

    // CSRF cookie must be readable by the client
    builder = builder.http_only(false);

    builder.finish()
}

pub fn expire_csrf_cookie(secure: bool) -> Cookie<'static> {
    Cookie::build(CSRF_COOKIE, "")
        .path("/")
        .same_site(same_site_policy(secure))
        .secure(secure)
        .http_only(false)
        .max_age(Duration::seconds(0))
        .expires(OffsetDateTime::now_utc() - Duration::hours(1))
        .finish()
}

fn generate_csrf_token() -> String {
    let mut bytes = [0u8; CSRF_TOKEN_LEN];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn same_site_policy(secure: bool) -> SameSite {
    if secure {
        SameSite::None
    } else {
        SameSite::Lax
    }
}

