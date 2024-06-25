use crate::utils::hash;
use anyhow::anyhow;
use bluesnow_result::{invalid_input_error, Error, Result};
use tokio::task::spawn_blocking;
use tracing::debug;

pub async fn hash(password: &str) -> Result<String> {
    let password = password.to_string();
    let jh = spawn_blocking(move || hash::argon_hash(password));
    let password = jh.await??;
    Ok(password)
}

pub async fn verify(password: String, hashed_pass: String) -> Result<()> {
    let jh = spawn_blocking(move || hash::argon_verify(password, hashed_pass));
    if let Err(e) = jh.await? {
        debug!("The password is not correct: {e}");
        Err(invalid_input_error(
            "password",
            "The password is not correct.",
        ))
    } else {
        Ok(())
    }
}
