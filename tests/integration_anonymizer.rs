use anonymize_rs::{
    anonymizer::{AnonymizePipeline, Anonymizer},
    config::AnonymizePipelineConfig,
};
use anyhow::Result;

#[tokio::main]
#[test]
async fn test_config() -> Result<()> {
    let path = "./tests/config/config.yaml".to_string();
    let config = AnonymizePipelineConfig::new(&path).await?;
    println!("{config:?}");

    assert_eq!(config.pipeline.len(), 2);
    Ok(())
}

#[tokio::main]
#[test]
#[ignore]
async fn test_config_ner() -> Result<()> {
    let path = "./tests/config/config_ner_en.yaml".to_string();
    let config = AnonymizePipelineConfig::new(&path).await?;
    println!("{config:?}");

    assert_eq!(config.pipeline.len(), 1);
    Ok(())
}

#[tokio::main]
#[test]
async fn test_replace_config() -> Result<()> {
    let path = "./tests/config/config.yaml".to_string();

    let text = "I like to eat apples and bananas and plums";

    let config = AnonymizePipelineConfig::new(&path).await?;
    let anonymize_pipeline = AnonymizePipeline::new(config)?;
    let res = anonymize_pipeline.anonymize(text, None)?;
    println!("{:?}", res);
    assert_eq!(
        res.text,
        "I like to eat FRUIT_FLASH0 and FRUIT_FLASH1 and FRUIT_REGEX0 "
    );

    Ok(())
}
