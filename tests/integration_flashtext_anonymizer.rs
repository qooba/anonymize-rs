use anonymize_rs::anonymizer::{flashtext_anonymizer::FlashTextAnonymizer, Anonymizer};
use anyhow::Result;

#[tokio::main]
#[test]
async fn test_flashtext_replace() -> Result<()> {
    let mut flash_text = FlashTextAnonymizer::new(None);
    flash_text.add_keyword("apple")?;
    flash_text.add_keyword("banana")?;
    flash_text.add_keyword("plum")?;

    let test_cases = [
        (
            "I like to eat apples and bananas and plums",
            "I like to eat FRUIT0 and FRUIT1 and FRUIT2",
        ),
        (
            "{\"content\": \"I like to eat apples and bananas and plums\"}",
            "{\"content\": \"I like to eat FRUIT0 and FRUIT1 and FRUIT2\"}",
        ),
    ];

    for test_case in test_cases {
        let text = test_case.0;
        let keywords = flash_text.find_keywords(text);
        println!("{:?}", keywords);
        assert_eq!(keywords[0], "apple");

        let res = flash_text.replace_keywords(text, Some("FRUIT"))?;
        assert_eq!(&res.text, &test_case.1);

        let d_res = flash_text.deanonymize(res);
        assert_eq!(d_res, test_case.0);
    }
    Ok(())
}

#[test]
fn test_flashtest_replace_file1() -> Result<()> {
    let mut flash_text = FlashTextAnonymizer::new(None);
    flash_text.add_keywords_file("./tests/config/fruits.txt")?;

    let text = "I like to eat apples and bananas";
    let keywords = flash_text.find_keywords(text);
    println!("{:?}", keywords);
    assert_eq!(keywords[0], "apple");

    let res = flash_text.replace_keywords(text, Some("FRUIT"))?;
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1");
    Ok(())
}

#[test]
fn test_flashtest_replace_file2() -> Result<()> {
    let mut flash_text = FlashTextAnonymizer::new(Some("FRUIT".to_string()));
    flash_text.add_keywords_file("./tests/config/fruits.txt")?;

    let text = "I like to eat apples and bananas";
    let keywords = flash_text.find_keywords(text);
    println!("{:?}", keywords);
    assert_eq!(keywords[0], "apple");

    let res = flash_text.replace_keywords(text, None)?;
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1");
    Ok(())
}
