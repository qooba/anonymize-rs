use std::{collections::HashMap, fs::File, io::BufReader};

use anonymize_rs::anonymizer::ner_anonymizer::NerAnonymizer;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    id2label: HashMap<String, String>,
}

#[test]
fn test_ner_replace() -> Result<()> {
    let model_path = "./examples/clarin-pl/model.onnx".to_string();
    let tokenizer_path = "./examples/clarin-pl/tokenizer.json".to_string();
    let file = File::open("./examples/clarin-pl/config.json")?;
    let reader = BufReader::new(file);
    let u: Config = serde_json::from_reader(reader)?;
    let id2label = u.id2label;

    let ner_anonymizer = NerAnonymizer::new(model_path, tokenizer_path, id2label, Some(true))?;

    let text = "Jan Kowalski mieszka w Krakowie na ulicy Warszawskiej. Jego numer telefonu to 555555555.";

    let res = ner_anonymizer.replace_matches(text, None)?;
    println!("{:?}", res);

    //assert_eq!(res.text, "I like to eat FRUIT0 and FRUIT1 and FRUIT2");
    Ok(())
}
