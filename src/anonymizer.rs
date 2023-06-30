use anyhow::Result;
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

pub struct AnonymizerPipeline {}

pub trait Anonymizer {
    fn anonymize(&self, text: &str, replacement: &str) -> ReplaceResult;
}

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word_end: bool,
}

pub struct FlashTextAnonymizer {
    root: TrieNode,
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
    pub fn new() -> Self {
        FlashTextAnonymizer {
            root: TrieNode::new(),
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

    pub fn add_keyword(&mut self, word: &str) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_word_end = true;
    }

    pub fn replace_keywords(&self, text: &str, replacement: &str) -> ReplaceResult {
        let mut internal_text = text.to_string();
        internal_text.push_str("  ");
        let mut result = String::new();
        let mut ch_indices = internal_text.char_indices();
        let mut start = 0;

        let mut items = HashMap::new();
        let mut idx = 0;

        while let Some((match_start, ch)) = ch_indices.next() {
            if let Some(_word) = self.traverse_trie(match_start, ch, &mut ch_indices) {
                result.push_str(&internal_text[start..match_start]);
                let mut rep = replacement.to_string();
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

        ReplaceResult {
            text: result,
            items,
        }
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

        while let Some((next_start, next_ch)) = ch_indices.next() {
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
        while let Some((next_start, _)) = ch_indices.next() {
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

impl Default for FlashTextAnonymizer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RegexAnonymizer {
    regex_patterns: Vec<Regex>,
}

impl RegexAnonymizer {
    pub fn new() -> Self {
        RegexAnonymizer {
            regex_patterns: Vec::new(),
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

    pub fn add_regex_pattern(&mut self, pattern: &str) -> Result<(), regex::Error> {
        let regex = Regex::new(pattern)?;
        self.regex_patterns.push(regex);
        Ok(())
    }

    pub fn replace_regex_matches(&self, text: &str, replacement: &str) -> ReplaceResult {
        let mut result = text.to_string();
        let mut items = HashMap::new();
        let mut idx = 0;

        for pattern in &self.regex_patterns {
            let mut it = pattern.find_iter(&result).enumerate().peekable();
            if !it.peek().is_none() {
                let mut rep = replacement.to_string();
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

        ReplaceResult {
            text: result,
            items,
        }
    }
}

impl Default for RegexAnonymizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Anonymizer for FlashTextAnonymizer {
    fn anonymize(&self, text: &str, replacement: &str) -> ReplaceResult {
        self.replace_keywords(text, replacement)
    }
}

impl Anonymizer for RegexAnonymizer {
    fn anonymize(&self, text: &str, replacement: &str) -> ReplaceResult {
        self.replace_regex_matches(text, replacement)
    }
}
