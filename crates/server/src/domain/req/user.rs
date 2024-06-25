use garde::Validate;
use serde::de::Unexpected::Option;
use serde::Deserialize;

#[derive(Validate, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 1))]
    pub username: String,
    #[garde(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
