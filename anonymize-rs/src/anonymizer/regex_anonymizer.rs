use crate::anonymizer::{Anonymizer, ReplaceResult};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
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
            .try_for_each(|word| -> Result<()> { self.add_regex_pattern(&word?) })?;

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
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        let mut result = text.to_string();
        let mut items: HashMap<String, String> = match items {
            Some(it) => it,
            None => HashMap::new(),
        };
        let mut idx = 0;

        let base_replacement = if replacement.is_some() {
            replacement.ok_or(anyhow!("SET REPLACEMENT"))?.to_string()
        } else {
            self.replacement.clone().ok_or(anyhow!("SET REPLACEMENT"))?
        };

        for pattern in &self.regex_patterns {
            let mut it = pattern.find_iter(&result).enumerate().peekable();
            if it.peek().is_some() {
                let mut rep = String::new();
                let mut new = String::with_capacity(result.len());
                let mut last_match = 0;
                for (_i, m) in it {
                    let start = m.start();
                    new.push_str(&result[last_match..start]);
                    last_match = m.end();

                    let item_value = result[start..last_match].to_string();

                    let existing_item = items.iter().find(|(_, v)| *v == &item_value);
                    match existing_item {
                        Some((k, _v)) => {
                            rep = k.to_string();
                        }
                        None => {
                            rep.push_str(&base_replacement);
                            rep.push_str(&idx.to_string());
                            items.insert(rep.to_string(), item_value);
                            idx += 1;
                        }
                    }

                    new.push_str(&rep.to_string());
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

impl Anonymizer for RegexAnonymizer {
    fn anonymize(
        &self,
        text: &str,
        replacement: Option<&str>,
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        self.replace_regex_matches(text, replacement, items)
    }
}
