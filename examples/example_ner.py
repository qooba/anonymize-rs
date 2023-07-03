# -*- coding: utf-8 -*-

#pip install torch transformers onnx sacremoses
#export PATH=${PATH}:/home/jovyan/.local/bin/
#python3 example.py

import os

import torch
from transformers import AutoModelForMaskedLM, AutoTokenizer, AutoModelForTokenClassification


from transformers import pipeline
tokenizer = AutoTokenizer.from_pretrained("xlm-roberta-large-finetuned-conll03-english")
model = AutoModelForTokenClassification.from_pretrained("xlm-roberta-large-finetuned-conll03-english")
classifier = pipeline("ner", model=model, tokenizer=tokenizer)
print(classifier("Jan Kowalski mieszka w Krakowie na ulice Warszawskiej."))

tokenizer_output = tokenizer.batch_encode_plus(
        [
            (
                "Jan Kowalski mieszka w Krakowie na ulice Warszawskiej."
            )
        ],
    padding='longest',
    add_special_tokens=True,
    return_tensors='pt'
    )

print(tokenizer_output)



input_ids = tokenizer_output["input_ids"]
attention_mask = tokenizer_output["attention_mask"]
#token_type_ids = tokenizer_output["token_type_ids"]


#print(output)
#print(model)

dynamic_axes = {
    0: "batch",
    1: "seq",
}

output_dir = "./xlm-ner"
os.makedirs(output_dir, exist_ok=True)
torch.onnx.export(
    model,
    (input_ids, attention_mask),
    os.path.join(output_dir, "model.onnx"),
    input_names=["input_ids", "attention_mask"],
    output_names=["logits"],
    dynamic_axes={
        "input_ids": dynamic_axes,
        "attention_mask": dynamic_axes,
        "logits": dynamic_axes,
    },
    opset_version=13,
)

tokenizer.save_pretrained(output_dir)
