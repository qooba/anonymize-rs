use crate::config::{AnonymizePipelineConfig, AnonymizerConfig};
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceResult {
    pub text: String,
    pub items: HashMap<String, String>,
}

pub trait Anonymizer {
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult>;
}

pub struct AnonymizePipeline {
    pub anonymizers: Vec<Box<dyn Anonymizer>>,
}

impl AnonymizePipeline {
    pub async fn new(config: &str) -> Result<Self> {
        let anonymize_config = AnonymizePipelineConfig::new(&config.to_string()).await?;
        let mut anonymizers: Vec<Box<dyn Anonymizer>> = vec![];
        for c in anonymize_config.pipeline {
            match c {
                AnonymizerConfig::FlashText { name, file } => {
                    let mut anonymizer = FlashTextAnonymizer::new(Some(name));
                    anonymizer.add_keywords_file(&file);
                    anonymizers.push(Box::new(anonymizer));
                }
                AnonymizerConfig::Regex { name, file } => {
                    let mut anonymizer = RegexAnonymizer::new(Some(name));
                    anonymizer.add_regex_patterns_file(&file);
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

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word_end: bool,
}

pub struct FlashTextAnonymizer {
    root: TrieNode,
    replacement: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word_end: false,
        }
    }
}

impl FlashTextAnonymizer {
    pub fn new(replacement: Option<String>) -> Self {
        FlashTextAnonymizer {
            root: TrieNode::new(),
            replacement,
        }
    }

    pub fn add_keywords_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        io::BufReader::new(file)
            .lines()
            .try_for_each(|word| -> Result<()> {
                self.add_keyword(&word?);
                Ok(())
            })?;

        Ok(())
    }

    pub fn add_keyword(&mut self, word: &str) -> Result<()> {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_word_end = true;
        Ok(())
    }

    pub fn replace_keywords(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult> {
        let mut internal_text = text.to_string();
        internal_text.push_str("  ");
        let mut result = String::new();
        let mut ch_indices = internal_text.char_indices();
        let mut start = 0;

        let mut items = HashMap::new();
        let mut idx = 0;

        let base_replacement = if replacement.is_some() {
            replacement.ok_or(anyhow!("SET REPLACEMENT"))?.to_string()
        } else {
            self.replacement.clone().ok_or(anyhow!("SET REPLACEMENT"))?
        };

        while let Some((match_start, ch)) = ch_indices.next() {
            if let Some(_word) = self.traverse_trie(match_start, ch, &mut ch_indices) {
                result.push_str(&internal_text[start..match_start]);
                let mut rep = base_replacement.to_string();
                rep.push_str(&idx.to_string());
                result.push_str(&rep);
                start = self.skip_to_word_boundary(
                    &internal_text,
                    match_start + ch.len_utf8(),
                    &mut ch_indices,
                );
                items.insert(rep, internal_text[match_start..start].to_string());
                idx += 1;
            }
        }
        result.push_str(&internal_text[start..]);
        result.pop();

        Ok(ReplaceResult {
            text: result,
            items,
        })
    }

    pub fn find_keywords(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut ch_indices = text.char_indices();
        while let Some((start, ch)) = ch_indices.next() {
            if let Some(word) = self.traverse_trie(start, ch, &mut ch_indices) {
                result.push(word);
            }
        }
        result
    }

    fn traverse_trie<'a, I>(&self, start: usize, ch: char, ch_indices: &mut I) -> Option<String>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut node = self.root.children.get(&ch)?;
        let mut end = start + ch.len_utf8();
        let mut chars = vec![ch];

        for (next_start, next_ch) in ch_indices.by_ref() {
            if let Some(next_node) = node.children.get(&next_ch) {
                chars.push(next_ch);
                end = next_start + next_ch.len_utf8();
                node = next_node;
            } else {
                break;
            }
        }

        if node.is_word_end {
            Some(chars.into_iter().collect())
        } else {
            None
        }
    }

    fn skip_to_word_boundary<'a, I>(&self, text: &str, start: usize, ch_indices: &mut I) -> usize
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut end = start;
        for (next_start, _) in ch_indices.by_ref() {
            if self.is_word_boundary(text, next_start) {
                end = next_start;
                break;
            }
        }
        end
    }

    fn is_word_boundary(&self, text: &str, index: usize) -> bool {
        if index >= text.len() {
            return true;
        }

        let ch = text.chars().nth(index).unwrap();
        !ch.is_alphabetic()
    }
}

pub struct RegexAnonymizer {
    regex_patterns: Vec<Regex>,
    replacement: Option<String>,
}

impl RegexAnonymizer {
    pub fn new(replacement: Option<String>) -> Self {
        RegexAnonymizer {
            regex_patterns: Vec::new(),
            replacement,
        }
    }

    pub fn add_regex_patterns_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        io::BufReader::new(file)
            .lines()
            .try_for_each(|word| -> Result<()> {
                self.add_regex_pattern(&word?)?;
                Ok(())
            })?;

        Ok(())
    }

    pub fn add_regex_pattern(&mut self, pattern: &str) -> Result<()> {
        let regex = Regex::new(pattern)?;
        self.regex_patterns.push(regex);
        Ok(())
    }

    pub fn replace_regex_matches(
        &self,
        text: &str,
        replacement: Option<&str>,
    ) -> Result<ReplaceResult> {
        let mut result = text.to_string();
        let mut items = HashMap::new();
        let mut idx = 0;

        let base_replacement = if replacement.is_some() {
            replacement.ok_or(anyhow!("SET REPLACEMENT"))?.to_string()
        } else {
            self.replacement.clone().ok_or(anyhow!("SET REPLACEMENT"))?
        };

        for pattern in &self.regex_patterns {
            let mut it = pattern.find_iter(&result).enumerate().peekable();
            if it.peek().is_some() {
                let mut rep = base_replacement.to_string();
                rep.push_str(&idx.to_string());
                let mut new = String::with_capacity(result.len());
                let mut last_match = 0;
                for (_i, m) in it {
                    let start = m.start();
                    new.push_str(&result[last_match..start]);
                    new.push_str(&rep.to_string());
                    last_match = m.end();
                    items.insert(rep.to_string(), result[start..last_match].to_string());
                    idx += 1;
                }
                new.push_str(&result[last_match..]);

                result = new;
            }
        }

        Ok(ReplaceResult {
            text: result,
            items,
        })
    }
}

impl Anonymizer for FlashTextAnonymizer {
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult> {
        self.replace_keywords(text, replacement)
    }
}

impl Anonymizer for RegexAnonymizer {
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult> {
        self.replace_regex_matches(text, replacement)
    }
}
