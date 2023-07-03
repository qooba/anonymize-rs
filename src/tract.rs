use ndarray::s;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tokenizers::tokenizer::{Result, Tokenizer};
use tract_onnx::prelude::*;
use std::time::Instant;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Config {
    id2label: HashMap<String,String>,
}


fn main() -> Result<()> {
    let now = Instant::now();


    
    //let model_dir = PathBuf::from_str("./xlm-ner")?;
    //let model_dir = PathBuf::from_str("./xlm-ner")?;
    let model_dir = PathBuf::from_str("./clarin-pl")?;
    let file = File::open("./clarin-pl/config.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Config = serde_json::from_reader(reader)?;
    let tt = u.id2label;
    dbg!(&tt);

    let model = tract_onnx::onnx()
        .model_for_path(Path::join(&model_dir, "model.onnx"))?
        .into_optimized()?
        .into_runnable()?;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("MODEL LOADED");

    let now = Instant::now();
    let tokenizer = Tokenizer::from_file(Path::join(&model_dir, "tokenizer.json"))?;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);



    let now = Instant::now();
    let text = "Jan Kowalski mieszka w Krakowie na ulice Warszawskiej.";

    let tokenizer_output = tokenizer.encode(text, true)?;
    let input_ids = tokenizer_output.get_ids();
    let attention_mask = tokenizer_output.get_attention_mask();
    let token_type_ids = tokenizer_output.get_type_ids();
    let length = input_ids.len();
    println!("TOKNE LEN: {length:?}");
    //let mask_pos =
    //    input_ids.iter().position(|&x| x == tokenizer.token_to_id("<mask>").unwrap()).unwrap();


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
    let token_type_ids: Tensor = tract_ndarray::Array2::from_shape_vec(
        (1, length),
        token_type_ids.iter().map(|&x| x as i64).collect(),
    )?
    .into();

    //let mask_pos = 2;
    let outputs =
        model.run(tvec!(input_ids.into(), attention_mask.into(), token_type_ids.into()))?;
    let logits = outputs[0].to_array_view::<f32>()?;
    //let logits = logits.slice(s![0, mask_pos, ..]);
    //let word_id = logits.iter().zip(0..).max_by(|a, b| a.0.partial_cmp(b.0).unwrap()).unwrap().1;
    //let word = tokenizer.id_to_token(word_id);
    //println!("Result: {word:?}");

    //use ndarray::{array, Axis};
// let mut arr = array![
//     [[1, 2], [3, 4], [5, 6]],
//     [[7, 8], [9, 10], [11, 12]],
// ];

    
    //println!("Outputs: {outputs:?}");
    //println!("Logits: {logits:?}");

   let r1 = logits.axis_iter(Axis(0)).last().unwrap();

   r1.axis_iter(Axis(0)).for_each(|x| {
        let r3 =x.mapv(f32::exp);
        let r2 = r3.sum();
        let result = r3.mapv(|v| v/r2);
        let label_indices = result.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index).unwrap();
        let ll = tt[&label_indices.to_string()].to_string();
        dbg!(ll);
        //dbg!(r4);
   });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    Ok(())
}

use ndarray::{array, Axis};
use ndarray::prelude::*;
use ndarray::Array;


#[test]
fn test() {

let mut arr = array![[[-3.6059537, -4.285506, -4.3517995, -0.8845141, -0.482811, -0.6700195, 5.7227736, 7.6015415],
  [-2.8753364, -1.9642653, -3.0453959, -2.0741284, -1.4212604, -0.63029677, 12.5857525, -2.2296216],
  [-3.030313, -2.060628, -2.8657293, -2.4988143, -1.2686712, -0.5070703, 12.651835, -2.0611305],
  [-3.466222, -1.8563933, -2.6468396, -2.3124971, -0.85851383, -0.9410387, 11.706388, -1.5634444],
  [-2.980543, -1.8932556, -2.7204459, -1.7516459, -1.7691402, -0.5211005, 11.6239, -1.1230569],
  [-2.2075796, -2.6743174, -1.9935789, -0.9150337, -3.268633, -1.9148248, -0.90881, 14.040742],
  [-2.3726985, -2.5027752, -2.112869, 0.27194718, -3.661555, -2.1983712, -0.88319564, 13.940954],
  [0.23710823, -1.9415132, -2.626972, 12.424337, -4.100643, -2.1019156, 0.41036326, -1.5266646],
  [-2.3229377, -2.94542, -2.4421241, -0.46673942, -2.7306125, -2.2402189, -0.8568196, 14.053959],
  [-2.001008, -3.5847833, -2.6422377, 0.1636454, -2.6339934, -1.7409405, -1.3879201, 13.759484],
  [-0.63455474, -1.8817496, -4.1846004, 4.62758, 5.9834676, -2.5047069, -0.5978431, -0.90519494],
  [-1.3101119, -1.5176871, -2.9629064, 3.980482, 3.713103, -3.1840684, -0.8670796, 1.9839412],
  [-3.645009, -4.3375435, -4.612144, 0.6659852, -1.2479825, -0.4924805, 4.603184, 7.519336],
  [-2.6562033, -2.4132438, -3.301104, 1.1489949, -1.6373342, -1.310869, 5.4605193, 5.816]]];

   dbg!(arr.axes());

   let r1 = arr.axis_iter(Axis(0)).last().unwrap();

   let labels = ["B-LOC", "B-MISC", "B-ORG", "I-LOC", "I-MISC", "I-ORG", "I-PER", "O"];
   r1.axis_iter(Axis(0)).for_each(|x| {
        let r3 =x.mapv(f32::exp);
        let r2 = r3.sum();
        let result = r3.mapv(|v| v/r2);
        let label_indices = result.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index).unwrap();
        dbg!(labels.get(label_indices));
        //dbg!(r4);
   });




}

/* 
[{'entity': 'I-PER', 'score': 0.9999957, 'index': 1, 'word': '▁Jan', 'start': 0, 'end': 3}, 
{'entity': 'I-PER', 'score': 0.9999958, 'index': 2, 'word': '▁Ko', 'start': 4, 'end': 6}, 
{'entity': 'I-PER', 'score': 0.9999887, 'index': 3, 'word': 'wal', 'start': 6, 'end': 9}, 
{'entity': 'I-PER', 'score': 0.9999864, 'index': 4, 'word': 'ski', 'start': 9, 'end': 12}, 
{'entity': 'I-LOC', 'score': 0.9999865, 'index': 7, 'word': '▁Krakowie', 'start': 23, 'end': 31}, 
{'entity': 'I-MISC', 'score': 0.79235494, 'index': 10, 'word': '▁Warszawski', 'start': 41, 'end': 51}, 
{'entity': 'I-LOC', 'score': 0.52084136, 'index': 11, 'word': 'ej', 'start': 51, 'end': 53}]
{'input_ids': tensor([[     0,   3342,   1204,   8202,   1336,  39089,    148, 139801,     24,
          97499, 212284,   1334,      5,      2]]), 'attention_mask': tensor([[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]])}
*/

#[test]
fn tok() {

    let model_dir = PathBuf::from_str("./xlm-ner").unwrap();
    let tokenizer = Tokenizer::from_file(Path::join(&model_dir, "tokenizer.json")).unwrap();
    let text = "Jan Kowalski mieszka w Krakowie na ulice Warszawskiej.";

    let tokenizer_output = tokenizer.encode(text, true).unwrap();
    let input_ids = tokenizer_output.get_ids();
    let attention_mask = tokenizer_output.get_attention_mask();
    //let token_type_ids = tokenizer_output.get_type_ids();
    let length = input_ids.len();
    println!("TOKNE LEN: {length:?}");

    dbg!(input_ids);

    for word_id in input_ids {
        let word = tokenizer.id_to_token(word_id.to_owned());
        dbg!(word);
    }

 

}



