use std::sync::Arc;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use server_config::JwtConfig;
use server_global::global;
use tokio::sync::{Mutex, OnceCell};

use crate::web::auth::Claims;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: OnceCell<Arc<Mutex<Keys>>> = OnceCell::const_new();
pub static VALIDATION: OnceCell<Arc<Mutex<Validation>>> = OnceCell::const_new();

pub async fn initialize_keys_and_validation() {
    let jwt_config = match global::get_config::<JwtConfig>().await {
        Some(cfg) => cfg,
        None => {
            eprintln!("Failed to load JWT config");
            return;
        }
    };

    let keys = Keys::new(jwt_config.jwt_secret.as_bytes());
    KEYS.set(Arc::new(Mutex::new(keys)))
        .unwrap_or_else(|_| eprintln!("Failed to set KEYS"));

    let mut validation = jsonwebtoken::Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[&jwt_config.issuer]);
    VALIDATION
        .set(Arc::new(Mutex::new(validation)))
        .unwrap_or_else(|_| eprintln!("Failed to set VALIDATION"));
}

// pub static KEYS: Lazy<Arc<Mutex<Keys>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT
// config");     Arc::new(Mutex::new(Keys::new(config.jwt_secret.as_bytes())))
// });
//
// pub static VALIDATION: Lazy<Arc<Mutex<Validation>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT
// config");     let mut validation = Validation::default();
//     validation.leeway = 60;
//     validation.set_issuer(&[config.issuer.clone()]);
//     Arc::new(Mutex::new(validation))
// });

pub struct JwtUtils;

impl JwtUtils {
    pub async fn generate_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let keys = KEYS.get().expect("Keys not initialized").lock().await;
        encode(&Header::default(), claims, &keys.encoding)
    }

    pub async fn validate_token(
        token: &str,
        audience: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let keys = KEYS.get().expect("Keys not initialized").lock().await;
        let validation = VALIDATION.get().expect("Validation not initialized").lock().await;

        let mut validation_clone = validation.clone();
        validation_clone.set_audience(&[audience.to_string()]);
        decode::<Claims>(token, &keys.decoding, &validation_clone)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use server_initialize::initialize_config;

    use super::*;

    fn create_claims(issuer: &str, audience: &str, exp_offset: i64) -> Claims {
        let now = Utc::now();
        Claims::new(
            "user123".to_string(),
            (now + Duration::seconds(exp_offset)).timestamp() as usize,
            issuer.to_string(),
            audience.to_string(),
            now.timestamp() as usize,
            now.timestamp() as usize,
            "unique_token_id".to_string(),
            "account".to_string(),
            "admin".to_string(),
            "example_domain".to_string(),
        )
    }

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            initialize_keys_and_validation().await;
            *initialized = Some(Arc::new(()));
        }
    }

    #[tokio::test]
    async fn test_validate_token_success() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience", 3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_audience() {
        init().await;

        let claims = create_claims(
            "https://github.com/ByteByteBrew/soybean-admin-rust",
            "invalid_audience",
            3600,
        );
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_issuer() {
        init().await;

        let claims = create_claims("invalid_issuer", "audience", 3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_expired() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience", -3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_signature() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience", 3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let mut invalid_token = token.clone();
        let len = invalid_token.len();
        invalid_token.replace_range((len - 1)..len, "X");

        let result = JwtUtils::validate_token(&invalid_token, "audience").await;
        assert!(result.is_err());
    }
}
