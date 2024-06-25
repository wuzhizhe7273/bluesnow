use serde::{Deserialize, Serialize};

#[derive(strum::EnumString, strum::Display, Clone, Deserialize, Serialize, Debug)]
pub enum Environment {
    #[strum(serialize = "dev")]
    Dev,
    #[strum(serialize = "test")]
    Test,
    #[strum(serialize = "prod")]
    Prod,
}
