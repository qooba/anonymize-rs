use crate::anonymizer::Anonymizer;
use crate::anonymizer::ReplaceResult;
use crate::anonymizer::TrieNode;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use tract_onnx::prelude::tract_itertools::Itertools;

#[derive(Debug, Clone)]
pub struct FlashTextAnonymizer {
    root: TrieNode,
    replacement: Option<String>,
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
            .try_for_each(|word| -> Result<()> { self.add_keyword(&word?) })?;

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

    pub fn replace_keywords(
        &self,
        text: &str,
        replacement: Option<&str>,
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        let mut internal_text = text.to_string();
        internal_text.push_str("  ");
        let mut result = String::new();
        let mut ch_indices = internal_text.char_indices();
        let mut start = 0;

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

        while let Some((match_start, ch)) = ch_indices.next() {
            if let Some(_word) = self.traverse_trie(ch, &mut ch_indices) {
                result.push_str(&internal_text[start..match_start]);
                let mut rep = base_replacement.to_string();
                rep.push_str(&idx.to_string());
                start = self.skip_to_word_boundary(
                    &internal_text,
                    match_start + ch.len_utf8(),
                    &mut ch_indices,
                );
                let (item_value, addition) =
                    self.process_item_value(&internal_text[match_start..start]);
                let existing_item = items.iter().find(|(_, v)| *v == &item_value);
                match existing_item {
                    Some((k, _v)) => {
                        rep = k.to_string();
                    }
                    None => {
                        items.insert(rep.to_string(), item_value);
                        idx += 1;
                    }
                }

                result.push_str(&rep);
                result.push_str(&addition);
            }
        }
        result.push_str(&internal_text[start..]);
        result = result.trim_end().to_string();

        Ok(ReplaceResult {
            text: result,
            items,
        })
    }

    fn process_item_value(&self, item_value: &str) -> (String, String) {
        let final_value: String = item_value
            .trim_end_matches(|c: char| !c.is_alphabetic())
            .to_string();
        let addition = item_value[final_value.len()..].to_string();
        (final_value, addition)
    }

    pub fn find_keywords(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut ch_indices = text.char_indices();
        while let Some((_start, ch)) = ch_indices.next() {
            if let Some(word) = self.traverse_trie(ch, &mut ch_indices) {
                result.push(word);
            }
        }
        result
    }

    fn traverse_trie<I>(&self, ch: char, ch_indices: &mut I) -> Option<String>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut node = self.root.children.get(&ch)?;
        let mut chars = vec![ch];

        for (_next_start, next_ch) in ch_indices.by_ref() {
            if let Some(next_node) = node.children.get(&next_ch) {
                chars.push(next_ch);
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

    fn skip_to_word_boundary<I>(&self, text: &str, start: usize, ch_indices: &mut I) -> usize
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

impl Anonymizer for FlashTextAnonymizer {
    fn anonymize(
        &self,
        text: &str,
        replacement: Option<&str>,
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        self.replace_keywords(text, replacement, items)
    }
}
