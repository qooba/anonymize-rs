# -*- coding: utf-8 -*-

#pip install torch transformers onnx sacremoses
#export PATH=${PATH}:/home/jovyan/.local/bin/
#python3 example.py
#pip install transformers[onnx] torch

import os

#import torch
import transformers

from transformers import AutoModelForMaskedLM, AutoTokenizer, AutoModelForTokenClassification
from transformers.onnx import FeaturesManager
from pathlib import Path


from transformers import pipeline



tokenizer = AutoTokenizer.from_pretrained("xlm-roberta-large-finetuned-conll03-english")
model = AutoModelForTokenClassification.from_pretrained("xlm-roberta-large-finetuned-conll03-english")

feature='token-classification'

model_kind, model_onnx_config = FeaturesManager.check_supported_model_or_raise(model, feature=feature)
onnx_config = model_onnx_config(model.config)

output_dir = "./xlm-ner-transformers"
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
