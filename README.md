# anonymize-rs - ner, regex, flash text anonymizer

## Run application

Find selected ner model on:
https://huggingface.co/models?pipeline_tag=token-classification&library=transformers&sort=trending

```
git clone https://github.com/qooba/anonymize-rs
cd anonymize-rs

cargo run -- server --host 0.0.0.0 --port 8089 --config tests/config/config.yaml
```

Open: http://localhost:8089/index.html
