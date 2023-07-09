# [Anonymizers](https://github.com/qooba/anonymize-rs) 🎭 - ner, regex, flash text anonymizer

This project is a data anonymization library built with Rust 🦀. 

It uses three techniques to anonymize data: 
* Named Entity Recognition (NER), 
* Flash Text, 
* Regular Expressions (Regex). 

Library can be used:
* python library 🐍
* rust library 🦀
* REST API 🌐
* Docker image 🐠

# Anonymizers

## [Named Entity Recognition (NER)](https://en.wikipedia.org/wiki/Named-entity_recognition)

This method enables the library to identify and anonymize sensitive named entities in your data, like names, organizations, locations, and other personal identifiers.

Anonymizers library uses ML models in onnx format (using [tract](https://github.com/sonos/tract)).

To prepare onnx model (they are only for converting model to onnx thus you don't need them during inference) additional libraries will be required:
```bash
pip install torch onnx sacremoses transformers[onnx]
```

You can use exsting models from HuggingFace (please note that repository license is only associated with library code) eg 
* en [dslim/bert-base-NER](https://huggingface.co/dslim/bert-base-NER)
* pl [clarin-pl/FastPDN](https://huggingface.co/clarin-pl/FastPDN)
* multilanguage [xlm-roberta-large-finetuned-conll03-english](https://huggingface.co/xlm-roberta-large-finetuned-conll03-english)


```python
import os
import transformers
from transformers import AutoModelForMaskedLM, AutoTokenizer, AutoModelForTokenClassification
from transformers.onnx import FeaturesManager
from pathlib import Path
from transformers import pipeline

model_id='dslim/bert-base-NER'
tokenizer = AutoTokenizer.from_pretrained(model_id)
model = AutoModelForTokenClassification.from_pretrained(model_id)

feature='token-classification'

model_kind, model_onnx_config = FeaturesManager.check_supported_model_or_raise(model, feature=feature)
onnx_config = model_onnx_config(model.config)

output_dir = "./dslim"
os.makedirs(output_dir, exist_ok=True)

# export
onnx_inputs, onnx_outputs = transformers.onnx.export(
        preprocessor=tokenizer,
        model=model,
        config=onnx_config,
        opset=13,
        output=Path(output_dir+"/model.onnx")
)

print(onnx_inputs)
print(onnx_outputs)
tokenizer.save_pretrained(output_dir)
```

configuration file `config.yaml`:
```
pipeline:
  - kind: ner
    model_path: ./examples/dslim/model.onnx
    tokenizer_path: ./examples/dslim/tokenizer.json
    id2label:
      "0": ["O", false]
      "1": ["B-MISC", true]
      "2": ["I-MISC", true]
      "3": ["B-PER", true]
      "4": ["I-PER", true]
      "5": ["B-ORG", true]
      "6": ["I-ORG", true]
      "7": ["B-LOC", true]
      "8": ["I-LOC", true]
```


## [Flash Text](https://arxiv.org/abs/1711.00046)

A fast method for searching and replacing words in large datasets, used to anonymize predefined sensitive information.

configuration file `config.yaml`:
```yaml
pipeline:
  - kind: flashText
    name: FRUIT_FLASH
    file: ./tests/config/fruits.txt
    keywords:
    - apple
    - banana
    - plum
```

## [Regex](https://en.wikipedia.org/wiki/Regular_expression)

This method provides a flexible way to identify and anonymize data patterns like credit card numbers, social security numbers, etc.

configuration file `config.yaml`:
```yaml
pipeline:
  - kind: regex
    name: FRUIT_REGEX
    file: ./tests/config/fruits_regex.txt
    patterns:
    - \bapple\w*\b
    - \bbanana\w*\b
    - \bplum\w*\b
```


# Usage

## REST API

The library exposes a simple and user-friendly REST API, making it easy to integrate this anonymization functionality into your existing systems or applications.

```
git clone https://github.com/qooba/anonymize-rs
cd anonymize-rs

cargo run -- server --host 0.0.0.0 --port 8089 --config config.yaml
```

where `config.yaml`:
```yaml
pipeline:
  - kind: flashText
    name: FRUIT_FLASH
    file: ./tests/config/fruits.txt
  - kind: regex
    name: FRUIT_REGEX
    file: ./tests/config/fruits_regex.txt
  - kind: ner
    model_path: ./examples/dslim/model.onnx
    tokenizer_path: ./examples/dslim/tokenizer.json
    token_type_ids_included: true
    id2label:
      "0": ["O", false] # [replacement, is_to_replaced]
      "1": ["B-MISC", true]
      "2": ["I-MISC", true]
      "3": ["B-PER", true]
      "4": ["I-PER", true]
      "5": ["B-ORG", true]
      "6": ["I-ORG", true]
      "7": ["B-LOC", true]
      "8": ["I-LOC", true]
```

### Anonymization

```
curl -X POST "http://localhost:8080/api/anonymize" -H "accept: application/json" -H "Content-Type: application/json" -d '{"text":"I like to eat apples and bananas and plums"}'
```

or

```
curl -X GET "http://localhost:8080/api/anonymize?text=I like to eat apples and bananas and plums" -H "accept: application/json" -H "Content-Type: application/json"
```


Response:
```
{
    "text": "I like to eat FRUIT_FLASH0 and FRUIT_FLASH1 and FRUIT_REGEX0",
    "items": {
        "FRUIT_FLASH0": "apples",
        "FRUIT_FLASH1": "banans",
        "FRUIT_REGEX0": "plums"
    }
}
```

### Deanonymization

```
curl -X POST "http://localhost:8080/api/deanonymize" -H "accept: application/json" -H "Content-Type: application/json" -d '{
    "text": "I like to eat FRUIT_FLASH0 and FRUIT_FLASH1 and FRUIT_REGEX0",
    "items": {
        "FRUIT_FLASH0": "apples",
        "FRUIT_FLASH1": "banans",
        "FRUIT_REGEX0": "plums"
    }
}'
```

Response:
```
{
    "text": "I like to eat apples and bananas and plums"
}
```

## Docker image

You can simply run anonymization server using docker image:
```
docker run -it -v $(pwd)/config.yaml:config.yaml -p 8080:8080 qooba/anonymize-rs server --host 0.0.0.0 --port 8080 --config config.yaml
```

## Python

```
pip install anonymizers
```

```python
>>> from anonymizers import Ner, Regex, FlashText
>>> id2label={"0":("O",False),"1": ("B-MISC", True),"2": ("I-MISC", True),"3": ("B-PER", True),"4": ("I-PER", True),"5": ("B-ORG", True),"6": ("I-ORG", True),"7": ("B-LOC", True),"8": ("I-LOC", True)}
>>> ner_anonymizer = Ner("./dslim/model.onnx","./dslim/tokenizer.json", id2label)
MODEL LOADED: 3.25s
TOKENIZER LOADED: 14.10ms
>>> ner_anonymizer.anonymize("My name is Sarah and I live in London. I like London.")
('My name is B-PER0 and I live in B-LOC0. I like B-LOC0.', {'B-PER0': 'Sarah', 'B-LOC0': 'London'})
```

```python
>>> from anonymizers import Ner, Regex, FlashText
>>> flash_anonymizer = FlashText("FRUIT", None, ["apple","banana","plum"])
>>> flash_anonymizer.anonymize("I like to eat apples and bananas and plums.")
('I like to eat FRUIT0 and FRUIT1 and FRUIT2.', {'FRUIT2': 'plums', 'FRUIT1': 'bananas', 'FRUIT0': 'apples'})
```



⚠️ Note:  Anonymizers library can help identify sensitive/PII data in un/structured text. However, it uses automated detection mechanisms, and there is no guarantee that it will find all sensitive information. Consequently, additional systems and protections should be employed. This tool is meant to be a part of your privacy protection suite, not the entirety of it. Always ensure your data protection measures are comprehensive and multi-layered.
