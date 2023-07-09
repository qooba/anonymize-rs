use crate::anonymizer::{Anonymizer, ReplaceResult};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use tokenizers::tokenizer::Tokenizer;
use tract_ndarray::Axis;
use tract_onnx::prelude::*;

type NerModel = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

#[derive(Debug, Clone)]
pub struct NerAnonymizer {
    model: NerModel,
    tokenizer: Tokenizer,
    id2label: HashMap<String, (String, bool)>,
    token_type_ids_included: Option<bool>,
}

impl NerAnonymizer {
    pub fn new(
        model_path: String,
        tokenizer_path: String,
        id2label: HashMap<String, (String, bool)>,
        token_type_ids_included: Option<bool>,
    ) -> Result<Self> {
        let now = Instant::now();
        let model = tract_onnx::onnx()
            .model_for_path(Path::new(&model_path))?
            .into_optimized()?
            .into_runnable()?;
        let elapsed = now.elapsed();
        println!("MODEL LOADED: {:.2?}", elapsed);

        let now = Instant::now();
        let tokenizer = Tokenizer::from_file(Path::new(&tokenizer_path)).unwrap();

        let elapsed = now.elapsed();
        println!("TOKENIZER LOADED: {:.2?}", elapsed);

        Ok(NerAnonymizer {
            model,
            tokenizer,
            id2label,
            token_type_ids_included,
        })
    }

    pub fn replace_matches(
        &self,
        text: &str,
        replacement: Option<&str>,
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        if let Some(_rep) = replacement {
            todo!("Functionality not implemented");
        }

        let tokenizer_output = self.tokenizer.encode(text, true).unwrap();
        let input_ids = tokenizer_output.get_ids();
        let attention_mask = tokenizer_output.get_attention_mask();
        let length = input_ids.len();
        let offsets = tokenizer_output.get_offsets();

        let input_ids: Tensor = tract_ndarray::Array2::from_shape_vec(
            (1, length),
            input_ids.iter().map(|&x| x as i64).collect(),
        )?
        .into();
        let attention_mask: Tensor = tract_ndarray::Array2::from_shape_vec(
            (1, length),
            attention_mask.iter().map(|&x| x as i64).collect(),
        )?
        .into();

        let outputs = if let Some(true) = self.token_type_ids_included {
            let token_type_ids = tokenizer_output.get_type_ids();
            let token_type_ids: Tensor = tract_ndarray::Array2::from_shape_vec(
                (1, length),
                token_type_ids.iter().map(|&x| x as i64).collect(),
            )?
            .into();

            self.model.run(tvec!(
                input_ids.into(),
                attention_mask.into(),
                token_type_ids.into()
            ))?
        } else {
            self.model
                .run(tvec!(input_ids.into(), attention_mask.into(),))?
        };

        let id2label = self.id2label.clone();

        let mut replacements = Vec::new();
        outputs[0]
            .to_array_view::<f32>()?
            .axis_iter(Axis(0))
            .last()
            .unwrap()
            .axis_iter(Axis(0))
            .enumerate()
            .for_each(|(i, x)| {
                let result_exp = x.mapv(f32::exp);
                let results_exp_sum = result_exp.sum();
                let softmax = result_exp.mapv(|v| v / results_exp_sum);
                let label_indices = softmax
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, _)| index)
                    .unwrap();
                let label = id2label[&label_indices.to_string()].clone();
                if label.1 {
                    let offset = offsets[i];
                    if offset.0 != offset.1 {
                        replacements.push((offset.0, offset.1, label.0));
                    }
                }
                //dbg!(r4);
            });

        self.replace_words(text, replacements, items)
    }

    fn replace_words(
        &self,
        text_in: &str,
        replacements: Vec<(usize, usize, String)>,
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        let mut text = text_in.to_string();
        let mut offset: isize = 0;
        let mut replaced_words: HashMap<String, String> = match items {
            Some(it) => it,
            None => HashMap::new(),
        };
        let mut replaced_words_counter = HashMap::new();

        for replacement in replacements {
            let (mut start, mut end, word) = replacement;
            start = (start as isize + offset) as usize;
            end = (end as isize + offset) as usize;

            if end > text.len() || start > end {
                return Err(anyhow!("Invalid range"));
            }

            let old_word = text[start..end].to_string();

            let existing_item = replaced_words.iter().find(|(_, v)| *v == &old_word);
            let mut word_rep = String::new();
            match existing_item {
                Some((k, _v)) => {
                    word_rep.push_str(k);
                }
                None => {
                    replaced_words_counter.insert(
                        word.to_string(),
                        if replaced_words_counter.contains_key(&word) {
                            replaced_words_counter[&word] + 1
                        } else {
                            0
                        },
                    );
                    let idx = replaced_words_counter[&word];
                    word_rep.push_str(&word);
                    word_rep.push_str(&idx.to_string());
                }
            }

            text.replace_range(start..end, &word_rep);

            replaced_words.insert(word_rep.to_string(), old_word);
            let len = end - start;
            offset += word_rep.len() as isize - len as isize;
        }

        Ok(ReplaceResult {
            text,
            items: replaced_words,
        })
    }
}

impl Anonymizer for NerAnonymizer {
    fn anonymize(
        &self,
        text: &str,
        replacement: Option<&str>,
        items: Option<HashMap<String, String>>,
    ) -> Result<ReplaceResult> {
        self.replace_matches(text, replacement, items)
    }
}
