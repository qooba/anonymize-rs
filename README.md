# anonymize-rs - ner, regex, flash text anonymizer

## Run application

Find selected ner model on:
https://huggingface.co/models?pipeline_tag=token-classification&library=transformers&sort=trending

```
git clone https://github.com/qooba/llm-ui
cd llm-ui
curl -LO https://huggingface.co/rustformers/gpt4all-j-ggml/resolve/main/gpt4all-j-q4_0-ggjt.bin
cargo run --release -- --host 0.0.0.0 --port 8089 gptj ./gpt4all-j-q4_0-ggjt.bin

cargo run --release -- chat --host 0.0.0.0 --port 8089 gptj ../llm-ui/models/rustformers/gpt4all-j-ggml/gpt4all-j-q4_0-ggjt.bin --db-path file:///tmp/temp1.db


cargo run --release -- admin init --db-path file:///tmp/temp1.db

cargo run --release -- admin add-user --db-path file:///tmp/temp1.db --user-id test1@test.pl
```

Open: http://localhost:8089/index.html
