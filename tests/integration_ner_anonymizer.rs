use anonymize_rs::{
    anonymizer::ner_anonymizer::NerAnonymizer,
    config::{AnonymizePipelineConfig, AnonymizerConfig},
};
use anyhow::Result;

async fn create_anonymizer(model_name: &str, lang: &str) -> Result<NerAnonymizer> {
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

    Ok(ner_anonymizer)
}

#[tokio::main]
#[test]
#[ignore]
async fn test_ner_replace_pl() -> Result<()> {
    let test_cases = [
        ("Jan Kowalski i Anna Kowalska mieszka w Krakowie na ulicy Warszawskiej.",
         "B-nam_liv_person0 I-nam_liv_person0 i B-nam_liv_person1 I-nam_liv_person1 mieszka w B-nam_loc_gpe_city0 na ulicy B-nam_fac_road0."),
        ("{\"content\": \"Jan Kowalski i Anna Kowalska mieszka w Krakowie na ulicy Warszawskiej.\"}",
         "{\"content\": \"B-nam_liv_person0 I-nam_liv_person0 i B-nam_liv_person1 I-nam_liv_person1 mieszka w B-nam_loc_gpe_city0 na ulicy B-nam_fac_road0.\"}")
    ];

    let model_name = "clarin-pl";
    let lang = "pl";
    let ner_anonymizer = create_anonymizer(model_name, lang).await?;

    for test_case in test_cases {
        let text = test_case.0;
        let res = ner_anonymizer.replace_matches(text, None)?;
        assert_eq!(res.text, test_case.1);
    }
    Ok(())
}

#[tokio::main]
#[test]
#[ignore]
async fn test_ner_replace_en() -> Result<()> {
    let test_cases = [
        (
            "My name is Sarah and I live in London",
            "My name is B-PER0 and I live in B-LOC0",
        ),
        (
            "{\"content\":\"My name is Sarah and I live in London\"}",
            "{\"content\":\"My name is B-PER0 and I live in B-LOC0\"}",
        ),
    ];
    let model_name = "dslim";
    let lang = "en";
    let ner_anonymizer = create_anonymizer(model_name, lang).await?;
    for test_case in test_cases {
        let text = test_case.0;
        let res = ner_anonymizer.replace_matches(text, None)?;
        assert_eq!(res.text, test_case.1);
    }
    Ok(())
}
