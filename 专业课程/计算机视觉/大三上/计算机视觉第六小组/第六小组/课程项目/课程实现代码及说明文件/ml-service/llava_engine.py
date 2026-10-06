#!/usr/bin/env python3
"""
LLaVA engine wrapper with MemVR enabled by default.
"""

from __future__ import annotations

import os
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import torch
from PIL import Image


def _get_env_bool(key: str, default: bool = False) -> bool:
    raw = os.getenv(key)
    if raw is None:
        return default
    return raw.strip() not in {"0", "false", "False", "no", "NO"}


def _get_env_float(key: str, default: float) -> float:
    raw = os.getenv(key)
    if raw is None:
        return default
    try:
        return float(raw)
    except ValueError:
        return default


def _get_env_int(key: str, default: int) -> int:
    raw = os.getenv(key)
    if raw is None:
        return default
    try:
        return int(raw)
    except ValueError:
        return default


@dataclass
class LlavaEngine:
    tokenizer: Any
    model: Any
    image_processor: Any
    conv_mode: str
    default_temperature: float
    default_top_p: Optional[float]
    default_num_beams: int
    default_max_new_tokens: int

    def build_prompt(self, history: List[Dict[str, Any]]) -> str:
        if not history:
            raise ValueError("Conversation history is empty")

        conv = self._conv_templates[self.conv_mode].copy()
        for idx, msg in enumerate(history):
            role = msg.get("role")
            content = msg.get("content", "")
            if role == "user" and idx == 0:
                if getattr(self.model.config, "mm_use_im_start_end", False):
                    content = (
                        self._DEFAULT_IM_START_TOKEN
                        + self._DEFAULT_IMAGE_TOKEN
                        + self._DEFAULT_IM_END_TOKEN
                        + "\n"
                        + content
                    )
                else:
                    content = self._DEFAULT_IMAGE_TOKEN + "\n" + content
            if role == "assistant":
                conv.append_message(conv.roles[1], content)
            else:
                conv.append_message(conv.roles[0], content)
        conv.append_message(conv.roles[1], None)
        return conv.get_prompt()

    def generate(
        self,
        image: Image.Image,
        history: List[Dict[str, Any]],
        params: Optional[Dict[str, Any]] = None,
    ) -> str:
        from llava.constants import IMAGE_TOKEN_INDEX
        from llava.mm_utils import process_images, tokenizer_image_token

        params = params or {}
        image_tensor = process_images([image], self.image_processor, self.model.config)
        if isinstance(image_tensor, list):
            image_tensor = [
                t.to(dtype=torch.float16, device=self.model.device) for t in image_tensor
            ]
        else:
            image_tensor = image_tensor.to(dtype=torch.float16, device=self.model.device)
        image_sizes = [image.size]

        prompt = self.build_prompt(history)
        input_ids = (
            tokenizer_image_token(prompt, self.tokenizer, IMAGE_TOKEN_INDEX, return_tensors="pt")
            .unsqueeze(0)
            .to(self.model.device)
        )

        temperature = float(params.get("temperature", self.default_temperature))
        top_p = params.get("top_p", self.default_top_p)
        num_beams = int(params.get("num_beams", self.default_num_beams))
        max_new_tokens = int(params.get("max_new_tokens", self.default_max_new_tokens))

        generate_kwargs = {
            "do_sample": temperature > 0,
            "temperature": temperature,
            "num_beams": num_beams,
            "max_new_tokens": max_new_tokens,
            "use_cache": False,
        }
        if top_p is not None:
            generate_kwargs["top_p"] = float(top_p)

        attention_mask = torch.ones_like(input_ids, dtype=torch.long, device=self.model.device)

        with torch.inference_mode():
            output_ids = self.model.generate(
                input_ids,
                images=image_tensor,
                image_sizes=image_sizes,
                attention_mask=attention_mask,
                **generate_kwargs,
            )

        outputs = self.tokenizer.batch_decode(output_ids, skip_special_tokens=True)[0].strip()
        return outputs


def load_llava_engine(
    model_name: str,
    device: str,
    conv_mode: str,
) -> LlavaEngine:
    sys.path.append(str(Path(__file__).parent / "llava_test"))
    sys.path.append(str(Path(__file__).parent / "llava"))

    from llava.constants import (
        DEFAULT_IMAGE_TOKEN,
        DEFAULT_IM_END_TOKEN,
        DEFAULT_IM_START_TOKEN,
    )
    from llava.conversation import conv_templates
    from llava.mm_utils import get_model_name_from_path
    from llava.model.builder import load_pretrained_model

    from memvr import apply_memvr_llama, LlamaMLP
    import transformers

    transformers.models.llama.modeling_llama.LlamaMLP = LlamaMLP

    model_path = os.path.expanduser(model_name)
    model_label = get_model_name_from_path(model_path)

    load_4bit = _get_env_bool("LLAVA_LOAD_4BIT", True)
    load_8bit = _get_env_bool("LLAVA_LOAD_8BIT", False)

    kwargs: Dict[str, Any] = {}
    if load_4bit:
        from transformers import BitsAndBytesConfig

        kwargs["quantization_config"] = BitsAndBytesConfig(
            load_in_4bit=True,
            bnb_4bit_compute_dtype=torch.float16,
            bnb_4bit_use_double_quant=True,
            bnb_4bit_quant_type="nf4",
        )
        load_4bit = False
    elif load_8bit:
        from transformers import BitsAndBytesConfig

        kwargs["quantization_config"] = BitsAndBytesConfig(load_in_8bit=True)
        load_8bit = False

    tokenizer, model, image_processor, _ = load_pretrained_model(
        model_path=model_path,
        model_base=None,
        model_name=model_label,
        load_4bit=load_4bit,
        load_8bit=load_8bit,
        device=device,
        **kwargs,
    )

    memvr_start = _get_env_int("LLAVA_MEMVR_START_LAYER", 5)
    memvr_end = _get_env_int("LLAVA_MEMVR_END_LAYER", 16)
    memvr_entropy = _get_env_float("LLAVA_MEMVR_ENTROPY_THRESHOLD", 0.75)
    memvr_ratio = _get_env_float("LLAVA_MEMVR_RETRACING_RATIO", 0.1)

    apply_memvr_llama(
        self=model,
        starting_layer=memvr_start,
        ending_layer=memvr_end,
        entropy_threshold=memvr_entropy,
        retracing_ratio=memvr_ratio,
    )

    model.config.use_cache = False
    model.eval()

    engine = LlavaEngine(
        tokenizer=tokenizer,
        model=model,
        image_processor=image_processor,
        conv_mode=conv_mode,
        default_temperature=_get_env_float("LLAVA_TEMPERATURE", 0.0),
        default_top_p=os.getenv("LLAVA_TOP_P") and float(os.getenv("LLAVA_TOP_P")),
        default_num_beams=_get_env_int("LLAVA_NUM_BEAMS", 1),
        default_max_new_tokens=_get_env_int("LLAVA_MAX_NEW_TOKENS", 512),
    )
    engine._DEFAULT_IMAGE_TOKEN = DEFAULT_IMAGE_TOKEN
    engine._DEFAULT_IM_START_TOKEN = DEFAULT_IM_START_TOKEN
    engine._DEFAULT_IM_END_TOKEN = DEFAULT_IM_END_TOKEN
    engine._conv_templates = conv_templates
    return engine
