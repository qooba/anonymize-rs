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

impl Anonymizer for RegexAnonymizer {
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult> {
        self.replace_regex_matches(text, replacement)
    }
}
