use std::{collections::HashMap, fs::File, io::BufReader};

use anonymize_rs::{
    anonymizer::ner_anonymizer::NerAnonymizer,
    config::{AnonymizePipelineConfig, AnonymizerConfig},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    id2label: HashMap<String, String>,
}

#[tokio::main]
#[test]
async fn test_ner_replace() -> Result<()> {
    let model_path = "./examples/clarin-pl/model.onnx".to_string();
    let tokenizer_path = "./examples/clarin-pl/tokenizer.json".to_string();

    let path = "./tests/config/config_ner.yaml".to_string();
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

    let text =
        "Jan Kowalski mieszka w Krakowie na ulicy Warszawskiej. Jego numer telefonu to 555555555.";

    let res = ner_anonymizer.replace_matches(text, None)?;
    println!("{:?}", res);

    //assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}
