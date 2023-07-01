use anyhow::{anyhow, Result};
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use url::Url;

#[derive(thiserror::Error, Debug)]
enum ReplaceTokensError {
    #[error("Environment variable: {0} not set")]
    NoEnv(String),
}

pub struct ReplaceTokens {}

impl ReplaceTokens {
    pub fn replace(template: &String) -> Result<String> {
        let mut text = template.clone();
        let tokens = Self::find_tokens(template)?;
        for token in tokens {
            let from = format!("${{{}}}", &token);
            let to = match env::var(token) {
                Ok(v) => v,
                Err(_error) => return Err(anyhow!(ReplaceTokensError::NoEnv(token.to_string()))),
            };
            text = text.replace(&from, &to);
        }
        Ok(text)
    }

    fn find_tokens(text: &String) -> Result<Vec<&str>> {
        let re = Regex::new(r"\$\{(?P<token>[a-zA-Z0-9_\-]+)\}").unwrap();
        let tokens: Vec<&str> = re
            .captures_iter(text)
            .map(|x| x.name("token").unwrap().as_str())
            .collect();
        Ok(tokens)
    }
}

pub async fn read_config_str(path: &String, replace_env: Option<bool>) -> Result<String> {
    let configuration_str = if Url::parse(path).is_ok() {
        reqwest::get(path).await?.text().await?
    } else {
        fs::read_to_string(path)?
    };

    if let Some(true) = replace_env {
        ReplaceTokens::replace(&configuration_str)
    } else {
        Ok(configuration_str)
    }
}

pub async fn read_config_bytes(path: &String) -> Result<Vec<u8>> {
    let res = if Url::parse(path).is_ok() {
        reqwest::get(path).await?.bytes().await?.to_vec()
    } else {
        fs::read(path)?
    };
    Ok(res)
}

pub async fn read_config<T>(path: &String, replace_env: Option<bool>) -> Result<T>
where
    T: DeserializeOwned,
{
    let config = read_config_str(path, replace_env).await?;
    let o: T = serde_yaml::from_str(&config)?;

    Ok(o)
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnonymizePipelineConfig {
    pub pipeline: Vec<AnonymizerConfig>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AnonymizerConfig {
    FlashText { name: String, file: String },
    Regex { name: String, file: String },
    Ner { model_path: String },
}

impl AnonymizePipelineConfig {
    pub async fn new(path: &String) -> Result<AnonymizePipelineConfig> {
        let s = read_config_str(path, Some(true)).await?;
        let config: AnonymizePipelineConfig = serde_yaml::from_str(&s)?;
        Ok(config)
    }
}

#[tokio::main]
#[test]
async fn test_replace_tokens() -> Result<()> {
    env::set_var("Q_1", "TOKEN1");
    env::set_var("Q_2", "TOKEN2");
    let mut text = "${Q_1} ${Q_2}".to_string();
    text = ReplaceTokens::replace(&text)?;
    env::remove_var("Q_1");
    env::remove_var("Q_2");

    assert_eq!(text, "TOKEN1 TOKEN2");

    Ok(())
}
