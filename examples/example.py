# -*- coding: utf-8 -*-

#pip install torch transformers onnx sacremoses
#export PATH=${PATH}:/home/jovyan/.local/bin/
#python3 example.py

import os

import torch
from transformers import AutoModelForMaskedLM, AutoTokenizer




from transformers import AutoTokenizer, AutoModel

tokenizer = AutoTokenizer.from_pretrained("allegro/herbert-base-cased")
model = AutoModel.from_pretrained("allegro/herbert-base-cased")

tokenizer_output = tokenizer.batch_encode_plus(
        [
            (
                "A potem szedł środkiem drogi w kurzawie, bo zamiatał nogami, ślepy dziad prowadzony przez tłustego kundla na sznurku.",
                "A potem leciał od lasu chłopak z butelką, ale ten ujrzawszy księdza przy drodze okrążył go z dala i biegł na przełaj pól do karczmy."
            )
        ],
    padding='longest',
    add_special_tokens=True,
    return_tensors='pt'
    )

input_ids = tokenizer_output["input_ids"]
attention_mask = tokenizer_output["attention_mask"]
token_type_ids = tokenizer_output["token_type_ids"]

output = model(
    **tokenizer.batch_encode_plus(
        [
            (
                "A potem szedł środkiem drogi w kurzawie, bo zamiatał nogami, ślepy dziad prowadzony przez tłustego kundla na sznurku.",
                "A potem leciał od lasu chłopak z butelką, ale ten ujrzawszy księdza przy drodze okrążył go z dala i biegł na przełaj pól do karczmy."
            )
        ],
    padding='longest',
    add_special_tokens=True,
    return_tensors='pt'
    )
)

#print(output)
#print(model)

dynamic_axes = {
    0: "batch",
    1: "seq",
}

output_dir = "./herbert"
os.makedirs(output_dir, exist_ok=True)
torch.onnx.export(
    model,
    (input_ids, attention_mask, token_type_ids),
    os.path.join(output_dir, "model.onnx"),
    input_names=["input_ids", "attention_mask", "token_type_ids"],
    output_names=["logits"],
    dynamic_axes={
        "input_ids": dynamic_axes,
        "attention_mask": dynamic_axes,
        "token_type_ids": dynamic_axes,
        "logits": dynamic_axes,
    },
    opset_version=13,
)

tokenizer.save_pretrained(output_dir)
