//! Authentication helpers

use std::time::{SystemTime, UNIX_EPOCH};

use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use hmac::{Hmac, Mac};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::config::AuthConfig;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub iss: String,
    pub iat: usize,
    pub exp: usize,
    pub jti: String,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub fn hash_login_id(login_id: &str, key: &str) -> Result<String, anyhow::Error> {
    let mut mac = <HmacSha256 as hmac::digest::KeyInit>::new_from_slice(key.as_bytes())
        .map_err(|err| anyhow::anyhow!(err))?;
    mac.update(login_id.as_bytes());
    let result = mac.finalize().into_bytes();
    Ok(hex::encode(result))
}

pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| anyhow::anyhow!(err))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, anyhow::Error> {
    let parsed = PasswordHash::new(hash).map_err(|err| anyhow::anyhow!(err))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn issue_tokens(config: &AuthConfig, user_id: Uuid) -> Result<TokenPair, anyhow::Error> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let access_exp = now + (config.access_ttl_days * 24 * 3600) as usize;
    let refresh_token = Uuid::new_v4().to_string();

    let claims = AuthClaims {
        sub: user_id.to_string(),
        iss: config.jwt_issuer.clone(),
        iat: now,
        exp: access_exp,
        jti: refresh_token.clone(),
    };

    let access_token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_in: (config.access_ttl_days * 24 * 3600),
    })
}

pub fn decode_access_token(config: &AuthConfig, token: &str) -> Result<AuthClaims, anyhow::Error> {
    let mut validation = Validation::default();
    validation.set_issuer(&[config.jwt_issuer.as_str()]);
    let data = jsonwebtoken::decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    )?;
    Ok(data.claims)
}
