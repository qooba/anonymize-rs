use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use anonymize_rs::anonymizer::flashtext_anonymizer::FlashTextAnonymizer;
use anonymize_rs::anonymizer::ner_anonymizer::NerAnonymizer;
use anonymize_rs::anonymizer::regex_anonymizer::RegexAnonymizer;
use anonymize_rs::anonymizer::{Anonymizer, ReplaceResult};
use anonymize_rs::config::{AnonymizePipelineConfig, AnonymizerConfig};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::wrap_pyfunction;

#[pymodule]
#[pyo3(name = "anonymizers")]
fn anonymizerrs(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyclass]
    pub struct Ner {
        anonymizer_instance: NerAnonymizer,
    }

    #[pymethods]
    impl Ner {
        #[new]
        pub fn new(
            model_path: String,
            tokenizer_path: String,
            id2label: HashMap<String, (String, bool)>,
            token_type_ids_included: Option<bool>,
        ) -> Self {
            let anonymizer = NerAnonymizer::new(
                model_path,
                tokenizer_path,
                id2label,
                token_type_ids_included,
            )
            .unwrap();

            Ner {
                anonymizer_instance: anonymizer,
            }
        }

        pub fn anonymize(
            &self,
            text: &str,
            replacement: Option<&str>,
            items: Option<HashMap<String, String>>,
        ) -> PyResult<(String, HashMap<String, String>)> {
            let result = self
                .anonymizer_instance
                .anonymize(text, replacement, items)
                .unwrap();
            Ok((result.text, result.items))
        }

        pub fn deanonymize(
            &self,
            text: String,
            items: HashMap<String, String>,
        ) -> PyResult<String> {
            let text = self
                .anonymizer_instance
                .deanonymize(ReplaceResult { text, items });
            Ok(text)
        }
    }

    #[pyclass]
    pub struct Regex {
        anonymizer_instance: RegexAnonymizer,
    }

    #[pymethods]
    impl Regex {
        #[new]
        pub fn new(name: String, file: Option<String>, patterns: Option<Vec<String>>) -> Self {
            let mut anonymizer = RegexAnonymizer::new(Some(name));
            if let Some(f) = file {
                anonymizer.add_regex_patterns_file(&f).unwrap();
            };
            if let Some(p) = patterns {
                p.iter()
                    .for_each(|v| anonymizer.add_regex_pattern(v).unwrap());
            };
            Regex {
                anonymizer_instance: anonymizer,
            }
        }

        pub fn anonymize(
            &self,
            text: &str,
            replacement: Option<&str>,
            items: Option<HashMap<String, String>>,
        ) -> PyResult<(String, HashMap<String, String>)> {
            let result = self
                .anonymizer_instance
                .anonymize(text, replacement, items)
                .unwrap();
            Ok((result.text, result.items))
        }

        pub fn deanonymize(
            &self,
            text: String,
            items: HashMap<String, String>,
        ) -> PyResult<String> {
            let text = self
                .anonymizer_instance
                .deanonymize(ReplaceResult { text, items });
            Ok(text)
        }
    }

    #[pyclass]
    pub struct FlashText {
        anonymizer_instance: FlashTextAnonymizer,
    }

    #[pymethods]
    impl FlashText {
        #[new]
        pub fn new(name: String, file: Option<String>, keywords: Option<Vec<String>>) -> Self {
            let mut anonymizer = FlashTextAnonymizer::new(Some(name));
            if let Some(f) = file {
                anonymizer.add_keywords_file(&f).unwrap();
            };
            if let Some(p) = keywords {
                p.iter().for_each(|v| anonymizer.add_keyword(v).unwrap());
            };
            FlashText {
                anonymizer_instance: anonymizer,
            }
        }

        pub fn anonymize(
            &self,
            text: &str,
            replacement: Option<&str>,
            items: Option<HashMap<String, String>>,
        ) -> PyResult<(String, HashMap<String, String>)> {
            let result = self
                .anonymizer_instance
                .anonymize(text, replacement, items)
                .unwrap();
            Ok((result.text, result.items))
        }

        pub fn deanonymize(
            &self,
            text: String,
            items: HashMap<String, String>,
        ) -> PyResult<String> {
            let text = self
                .anonymizer_instance
                .deanonymize(ReplaceResult { text, items });
            Ok(text)
        }
    }

    m.add_class::<Ner>()?;
    m.add_class::<Regex>()?;
    m.add_class::<FlashText>()?;
    Ok(())
}
