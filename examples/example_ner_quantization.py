# -*- coding: utf-8 -*-

#pip install torch transformers onnx sacremoses
#export PATH=${PATH}:/home/jovyan/.local/bin/
#python3 example.py

import os

import torch
from transformers import AutoModelForMaskedLM, AutoTokenizer, AutoModelForTokenClassification
from torch.quantization import quantize_dynamic


from transformers import pipeline
tokenizer = AutoTokenizer.from_pretrained("xlm-roberta-large-finetuned-conll03-english")
model = AutoModelForTokenClassification.from_pretrained("xlm-roberta-large-finetuned-conll03-english")

qconfig = {"dtype": torch.qint8}
quantized_model = quantize_dynamic(model, qconfig_spec=qconfig)

classifier = pipeline("ner", model=quantized_model, tokenizer=tokenizer)
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

output_dir = "./xlm-ner-quantized"
os.makedirs(output_dir, exist_ok=True)
torch.onnx.export(
    quantized_model,
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
    export_params=True,
)

tokenizer.save_pretrained(output_dir)
