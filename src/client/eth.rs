use jsonrpsee::{
    core::{client::ClientT, DeserializeOwned},
    http_client::HttpClientBuilder,
    types::ParamsSer,
};
use serde::Serialize;

const LOCALHOST: &'static str = "http://localhost:8545";

pub async fn request<'a, T>(
    url: Option<&String>,
    method: &'a str,
    params: Option<ParamsSer<'a>>,
) -> anyhow::Result<crate::commands::base_entity::JsonEntity<T>>
where
    T: DeserializeOwned + Serialize,
{
    let url = url.map(|w| w.to_owned()).unwrap_or(LOCALHOST.to_owned());
    let client = HttpClientBuilder::default().build(url)?;
    let response: Result<T, jsonrpsee::core::Error> = client.request(method, params).await;

    Ok(match response {
        Ok(response) => Ok(crate::commands::base_entity::JsonEntity::new(response)),
        Err(err) => Err(err),
    }?)
}
