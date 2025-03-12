use std::collections::HashMap;

use serde::Deserialize;

const USER_AGENT_URL: &str = "https://deviceandbrowserinfo.com/api/user_agents/all";

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Human {
    pub user_agent: String,
    pub browser: String,
    pub browser_version: Option<String>,
    pub os: String,
    pub os_version: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub name: String,
    pub url: Option<String>,
    pub user_agents: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentSet {
    pub humans: HashMap<String, Human>,
    pub bots: Vec<Bot>,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("unable to fetch user agents: {0}")]
    Fetch(reqwest::Error),
    #[error("unable to finish reading body: {0}")]
    BodyRead(reqwest::Error),
    #[error("unable to deserialise payload: {0}")]
    Deserialise(#[from] serde_json::Error),
}

pub async fn get_user_agents() -> Result<UserAgentSet, FetchError> {
    let resp = reqwest::get(USER_AGENT_URL).await.map_err(FetchError::Fetch)?;
    let data = resp.bytes().await.map_err(FetchError::BodyRead)?;

    let user_agent_set = serde_json::from_slice(&data)?;
    Ok(user_agent_set)
}
