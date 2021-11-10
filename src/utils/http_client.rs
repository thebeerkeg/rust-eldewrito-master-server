use std::fmt::Debug;
use serde::de::{DeserializeOwned};

pub async fn get<T: DeserializeOwned + Debug>(url: String) -> Result<T, reqwest::Error> {
    let result: Result<T, reqwest::Error> = reqwest::get(url).await?.json::<T>().await;
    result
}