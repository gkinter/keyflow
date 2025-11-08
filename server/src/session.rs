use crate::error::AppError;
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

const SESSION_LIFETIME_HOURS: i64 = 12;

#[derive(Clone)]
pub struct SessionSigner {
    encoding: EncodingKey,
    decoding_keys: Vec<DecodingKey<'static>>,
    validation: Validation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub sub: Uuid,
    pub exp: i64,
}

impl SessionSigner {
    pub fn new(secrets: &[String]) -> Result<Self, AppError> {
        if secrets.is_empty() {
            return Err(AppError::Config("no session signing keys configured".into()));
        }

        let encoding = EncodingKey::from_secret(secrets[0].as_bytes());
        let decoding_keys = secrets
            .iter()
            .map(|secret| DecodingKey::from_secret(secret.as_bytes()).into_static())
            .collect();

        Ok(Self {
            encoding,
            decoding_keys,
            validation: Validation::new(Algorithm::HS256),
        })
    }

    pub fn issue(&self, user_id: Uuid) -> Result<String, AppError> {
        let expiration = OffsetDateTime::now_utc()
            .checked_add(Duration::hours(SESSION_LIFETIME_HOURS))
            .ok_or_else(|| AppError::Internal)?;

        let claims = SessionClaims {
            sub: user_id,
            exp: expiration.unix_timestamp(),
        };

        jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &self.encoding).map_err(AppError::from)
    }

    pub fn verify(&self, token: &str) -> Result<SessionClaims, AppError> {
        let mut validation = self.validation.clone();
        validation.validate_exp = true;
        let mut last_error = None;

        for key in &self.decoding_keys {
            match jsonwebtoken::decode::<SessionClaims>(token, key, &validation) {
                Ok(token_data) => return Ok(token_data.claims),
                Err(err) => match err.kind() {
                    ErrorKind::InvalidSignature => {
                        last_error = Some(err);
                        continue;
                    }
                    _ => return Err(AppError::from(err)),
                },
            }
        }

        if let Some(err) = last_error {
            return Err(AppError::from(err));
        }

        Err(AppError::Unauthorized)
    }
}

