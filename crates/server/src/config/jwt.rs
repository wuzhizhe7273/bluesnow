use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;
use std::sync::OnceLock;
use std::time::Duration;
pub static ENCODING_KEY: OnceLock<EncodingKey> = OnceLock::new();
pub static DECODING_KEY: OnceLock<DecodingKey> = OnceLock::new();
// pub static DECODE_HEADER: OnceLock<Validation> = OnceLock::new();
// pub static ENCODE_HEADER: OnceLock<Header> = OnceLock::new();

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    secret: String,
    exp: u64,
    // iss:String
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "default-key".to_string(),
            exp: 60 * 60 * 24,
        }
    }
}

impl JwtConfig {
    pub fn get_encoding_key(&self) -> &EncodingKey {
        ENCODING_KEY.get_or_init(|| EncodingKey::from_secret(self.secret.clone().as_bytes()))
    }
    pub fn get_decoding_key(&self) -> &DecodingKey {
        DECODING_KEY.get_or_init(|| DecodingKey::from_secret(self.secret.clone().as_bytes()))
    }
    // pub fn get_encode_header(&self)->&Header{
    //     ENCODE_HEADER.get_or_init(||{
    //         Header::new(Algorithm::HS256)
    //     })
    // }
    // pub fn get_decode_header(&self)->&Validation{
    //     DECODE_HEADER.get_or_init(||{
    //         let mut validation=Validation::new(Algorithm::HS256);
    //         validation.set_issuer(&[self.iss.clone()]);
    //         validation.set_required_spec_claims(&["exp","iss"]);
    //         validation
    //     })
    // }
    pub fn get_exp(&self) -> Duration {
        Duration::from_secs(self.exp)
    }
}
