use anonymize_rs::anonymizer::regex_anonymizer::RegexAnonymizer;
use anyhow::Result;

#[test]
fn test_regex_replace() -> Result<()> {
    let mut regex_anonymizer = RegexAnonymizer::new(None);
    regex_anonymizer.add_regex_pattern(r"\bapple\w*\b")?;
    regex_anonymizer.add_regex_pattern(r"\bbanana\w*\b")?;
    regex_anonymizer.add_regex_pattern(r"\bplum\w*\b")?;

    let text = "I like to eat apples and bananas and plums";

    let res = regex_anonymizer.replace_regex_matches(text, Some("FRUIT"))?;
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}

#[test]
fn test_regex_replace_file1() -> Result<()> {
    let mut regex_anonymizer = RegexAnonymizer::new(None);
    regex_anonymizer.add_regex_patterns_file("./tests/config/fruits_regex.txt")?;

    let text = "I like to eat apples and bananas and plums";

    let res = regex_anonymizer.replace_regex_matches(text, Some("FRUIT"))?;
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}

#[test]
fn test_regex_replace_file2() -> Result<()> {
    let mut regex_anonymizer = RegexAnonymizer::new(Some("FRUIT".to_string()));
    regex_anonymizer.add_regex_patterns_file("./tests/config/fruits_regex.txt")?;

    let text = "I like to eat apples and bananas and plums";

    let res = regex_anonymizer.replace_regex_matches(text, None)?;
    println!("{:?}", res);

    assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}
