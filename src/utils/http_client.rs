use std::fmt::Debug;
use serde::de::{DeserializeOwned};
use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: reqwest::Client = {
        let timeout_duration = std::time::Duration::from_secs(10);

        reqwest::Client::builder()
            .timeout(timeout_duration)
            .build()
            .expect("Failed to build reqwest client")
    };
}

pub async fn get<T: DeserializeOwned + Debug>(url: String) -> Result<T, reqwest::Error> {
    let response = CLIENT.get(&url).send().await?;
    let result = response.json::<T>().await?;

    Ok(result)
}

