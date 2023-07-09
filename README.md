# anonymize-rs - ner, regex, flash text anonymizer

This project is a data anonymization library built with Rust. 
It uses three techniques to anonymize data: 
* Named Entity Recognition (NER), 
* Flash Text, 
* Regular Expressions (Regex). 


# Features

## Named Entity Recognition (NER) 

This method enables the library to identify and anonymize sensitive named entities in your data, like names, organizations, locations, and other personal identifiers.

## Flash Text 

A lightning-fast method for searching and replacing words in large datasets, used to anonymize predefined sensitive information.

## Regex

This method provides a flexible way to identify and anonymize data patterns like credit card numbers, social security numbers, etc.

## REST API

The library exposes a simple and user-friendly REST API, making it easy to integrate this anonymization functionality into your existing systems or applications.

## Command Line Interface (CLI)

It also offers a command-line interface for direct and easy use, perfect for ad-hoc data anonymization tasks or integrating into scripts.

## Run application

Find selected ner model on:
https://huggingface.co/models?pipeline_tag=token-classification&library=transformers&sort=trending

```
git clone https://github.com/qooba/anonymize-rs
cd anonymize-rs

cargo run -- server --host 0.0.0.0 --port 8089 --config tests/config/config.yaml
```

Open: http://localhost:8089/index.html

```
docker build -t qooba/anonymize-rs -f docker/Dockerfile .
```

```
curl -X POST "http://localhost:8080/post" -H "accept: application/json" -H "Content-Type: application/json" -d '{"content":"I like to eat apples and bananas and plums"}'
```

⚠️ Note on Data Protection: library can help identify sensitive/PII data in un/structured text. However, it uses automated detection mechanisms, and there is no guarantee that it will find all sensitive information. Consequently, additional systems and protections should be employed. This tool is meant to be a part of your privacy protection suite, not the entirety of it. Always ensure your data protection measures are comprehensive and multi-layered.
