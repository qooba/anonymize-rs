use anonymize_rs::{
    anonymizer::{ner_anonymizer::NerAnonymizer, ReplaceResult},
    config::{AnonymizePipelineConfig, AnonymizerConfig},
};
use anyhow::Result;

async fn replace_with_ner(text: &str, model_name: &str, lang: &str) -> Result<ReplaceResult> {
    let model_path = format!("./examples/{model_name}/model.onnx").to_string();
    let tokenizer_path = format!("./examples/{model_name}/tokenizer.json").to_string();

    let path = format!("./tests/config/config_ner_{lang}.yaml").to_string();
    let config = AnonymizePipelineConfig::new(&path).await?;
    let id2label = if let AnonymizerConfig::Ner {
        model_path: _,
        tokenizer_path: _,
        id2label,
        token_type_ids_included: _,
    } = config.pipeline.last().unwrap()
    {
        id2label
    } else {
        panic!("WRONG CONFIG");
    };

    let ner_anonymizer =
        NerAnonymizer::new(model_path, tokenizer_path, id2label.clone(), Some(true))?;

    let res = ner_anonymizer.replace_matches(text, None)?;

    Ok(res)
}

#[ignore]
#[tokio::main]
#[test]
async fn test_ner_replace_pl() -> Result<()> {
    let text =
        "Jan Kowalski i Anna Kowalska mieszka w Krakowie na ulicy Warszawskiej. Jego numer telefonu to 555555555.";

    let model_name = "clarin-pl";
    let lang = "pl";
    let res = replace_with_ner(text, model_name, lang).await?;
    println!("{:?}", res);

    //assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}

#[ignore]
#[tokio::main]
#[test]
async fn test_ner_replace_en() -> Result<()> {
    let text = "My name is Sarah and I live in London";

    let model_name = "dslim";
    let lang = "en";
    let res = replace_with_ner(text, model_name, lang).await?;
    println!("{:?}", res);

    //assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}
