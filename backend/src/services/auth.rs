//! Authentication helpers

use std::time::{SystemTime, UNIX_EPOCH};

use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::{rngs::OsRng, RngCore};
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

pub fn generate_sms_code() -> String {
    let mut buf = [0u8; 4];
    OsRng.fill_bytes(&mut buf);
    let value = u32::from_le_bytes(buf) % 1_000_000;
    format!("{:06}", value)
}

pub fn hash_phone(phone: &str, key: &str) -> Result<String, anyhow::Error> {
    let mut mac = <HmacSha256 as hmac::digest::KeyInit>::new_from_slice(key.as_bytes())
        .map_err(|err| anyhow::anyhow!(err))?;
    mac.update(phone.as_bytes());
    let result = mac.finalize().into_bytes();
    Ok(hex::encode(result))
}

pub fn encrypt_phone(phone: &str, key_b64: &str) -> Result<String, anyhow::Error> {
    let key_bytes = general_purpose::STANDARD.decode(key_b64)?;
    if key_bytes.len() != 32 {
        anyhow::bail!("PHONE_ENC_KEY must be 32 bytes base64");
    }
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|_| anyhow::anyhow!("invalid encryption key length"))?;
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, phone.as_bytes())
        .map_err(|_| anyhow::anyhow!("failed to encrypt phone"))?;
    let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext);
    Ok(general_purpose::STANDARD.encode(blob))
}

pub fn decrypt_phone(blob_b64: &str, key_b64: &str) -> Result<String, anyhow::Error> {
    let key_bytes = general_purpose::STANDARD.decode(key_b64)?;
    if key_bytes.len() != 32 {
        anyhow::bail!("PHONE_ENC_KEY must be 32 bytes base64");
    }
    let blob = general_purpose::STANDARD.decode(blob_b64)?;
    if blob.len() < 13 {
        anyhow::bail!("encrypted phone blob too short");
    }
    let (nonce_bytes, ciphertext) = blob.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|_| anyhow::anyhow!("invalid encryption key length"))?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("failed to decrypt phone"))?;
    Ok(String::from_utf8(plaintext)?)
}

pub fn mask_phone(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 7 {
        return "***".to_string();
    }
    let prefix = &digits[..3];
    let suffix = &digits[digits.len() - 4..];
    format!("{}****{}", prefix, suffix)
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

pub fn decode_access_token(
    config: &AuthConfig,
    token: &str,
) -> Result<AuthClaims, anyhow::Error> {
    let mut validation = Validation::default();
    validation.set_issuer(&[config.jwt_issuer.as_str()]);
    let data = jsonwebtoken::decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    )?;
    Ok(data.claims)
}
