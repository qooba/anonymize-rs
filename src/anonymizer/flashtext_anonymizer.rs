use crate::anonymizer::TrieNode;
use crate::anonymizer::ReplaceResult;
use crate::anonymizer::Anonymizer;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

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
            .try_for_each(|word| -> Result<()> {
                self.add_keyword(&word?)
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
            if let Some(_word) = self.traverse_trie(ch, &mut ch_indices) {
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
    fn anonymize(&self, text: &str, replacement: Option<&str>) -> Result<ReplaceResult> {
        self.replace_keywords(text, replacement)
    }
}
