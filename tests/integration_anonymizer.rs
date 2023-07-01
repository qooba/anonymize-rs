use anonymize_rs::{anonymizer::{FlashTextAnonymizer, RegexAnonymizer}, config::AnonymizePipelineConfig};
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
async fn test_flashtext_replace() -> Result<()> {
    let mut flash_text = FlashTextAnonymizer::new();
    flash_text.add_keyword("apple");
    flash_text.add_keyword("banana");
    flash_text.add_keyword("plum");

    let text = "I like to eat apples and bananas and plums";
    let keywords = flash_text.find_keywords(text);
    println!("{:?}", keywords);
    assert_eq!(keywords[0], "apple");

    let res = flash_text.replace_keywords(text, "FRUIT");
    println!("{:?}", res);
    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2 ");
    Ok(())
}

#[test]
fn test_flashtest_replace_file() -> Result<()> {
    let mut flash_text = FlashTextAnonymizer::new();
    flash_text.add_keywords_file("./tests/config/fruits.txt")?;

    let text = "I like to eat apples and bananas";
    let keywords = flash_text.find_keywords(text);
    println!("{:?}", keywords);
    assert_eq!(keywords[0], "apple");

    let res = flash_text.replace_keywords(text, "FRUIT");
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 ");
    Ok(())
}

#[test]
fn test_regex_replace() -> Result<()> {
    let mut regex_anonymizer = RegexAnonymizer::new();
    regex_anonymizer.add_regex_pattern(r"\bapple\w*\b")?;
    regex_anonymizer.add_regex_pattern(r"\bbanana\w*\b")?;
    regex_anonymizer.add_regex_pattern(r"\bplum\w*\b")?;

    let text = "I like to eat apples and bananas and plums";

    let res = regex_anonymizer.replace_regex_matches(text, "FRUIT");
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}

#[test]
fn test_regex_replace_file() -> Result<()> {
    let mut regex_anonymizer = RegexAnonymizer::new();
    regex_anonymizer.add_regex_patterns_file("./tests/config/fruits_regex.txt")?;

    let text = "I like to eat apples and bananas and plums";

    let res = regex_anonymizer.replace_regex_matches(text, "FRUIT");
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}
