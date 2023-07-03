use crate::anonymizer::flashtext_anonymizer::FlashTextAnonymizer;
use crate::anonymizer::regex_anonymizer::RegexAnonymizer;
use crate::config::{AnonymizePipelineConfig, AnonymizerConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod flashtext_anonymizer;
pub mod regex_anonymizer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceResult {
    pub text: String,
    pub items: HashMap<String, String>,
}

pub trait Anonymizer: AnonymizerClone {
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult>;
}

pub trait AnonymizerClone {
    fn clone_box(&self) -> Box<dyn Anonymizer>;
}

impl<T> AnonymizerClone for T
where
    T: 'static + Anonymizer + Clone,
{
    fn clone_box(&self) -> Box<dyn Anonymizer> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Anonymizer> {
    fn clone(&self) -> Box<dyn Anonymizer> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct AnonymizePipeline {
    pub anonymizers: Vec<Box<dyn Anonymizer>>,
}

impl AnonymizePipeline {
    pub fn new(anonymize_config: AnonymizePipelineConfig) -> Result<Self> {
        let mut anonymizers: Vec<Box<dyn Anonymizer>> = vec![];
        for c in anonymize_config.pipeline {
            match c {
                AnonymizerConfig::FlashText {
                    name,
                    file,
                    keywords,
                } => {
                    let mut anonymizer = FlashTextAnonymizer::new(Some(name));
                    if let Some(f) = file {
                        anonymizer.add_keywords_file(&f)?;
                    };
                    if let Some(k) = keywords {
                        k.iter()
                            .try_for_each(|v| -> Result<()> { anonymizer.add_keyword(v) })?;
                    };
                    anonymizers.push(Box::new(anonymizer));
                }
                AnonymizerConfig::Regex {
                    name,
                    file,
                    patterns,
                } => {
                    let mut anonymizer = RegexAnonymizer::new(Some(name));
                    if let Some(f) = file {
                        anonymizer.add_regex_patterns_file(&f)?;
                    };
                    if let Some(p) = patterns {
                        p.iter()
                            .try_for_each(|v| -> Result<()> { anonymizer.add_regex_pattern(v) })?;
                    };
                    anonymizers.push(Box::new(anonymizer));
                }
                AnonymizerConfig::Ner { model_path } => {}
            };
        }
        Ok(AnonymizePipeline { anonymizers })
    }
}

impl Anonymizer for AnonymizePipeline {
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult> {
        let mut replace_result = ReplaceResult {
            text: text.to_string(),
            items: HashMap::new(),
        };

        self.anonymizers
            .iter()
            .try_for_each(|anonymizer| -> Result<()> {
                let result = anonymizer.anonymize(&replace_result.text, replacement)?;
                replace_result.text = result.text;
                replace_result.items.extend(result.items);
                Ok(())
            });

        Ok(replace_result)
    }
}

#[derive(Default, Debug, Clone)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word_end: false,
        }
    }
}
