import argparse
import torch
from PIL import Image
import requests
from io import BytesIO
import os

# LLaVA 核心组件
from llava.constants import IMAGE_TOKEN_INDEX, DEFAULT_IMAGE_TOKEN, DEFAULT_IM_START_TOKEN, DEFAULT_IM_END_TOKEN
from llava.conversation import conv_templates
from llava.model.builder import load_pretrained_model
from llava.utils import disable_torch_init
from llava.mm_utils import tokenizer_image_token, process_images, get_model_name_from_path

import transformers

# MemVR 核心组件 (确保 memvr.py 在当前目录)
try:
    from memvr import apply_memvr_llama, LlamaMLP
except ImportError:
    print("【错误】找不到 memvr.py，请确保该文件在当前目录下。")
    exit()

def load_image(image_file):
    if image_file.startswith('http://') or image_file.startswith('https://'):
        response = requests.get(image_file)
        image = Image.open(BytesIO(response.content)).convert('RGB')
    else:
        image = Image.open(image_file).convert('RGB')
    return image

def main(args):
    # 1. 初始化
    disable_torch_init()
    model_path = os.path.expanduser(args.model_path)
    model_name = get_model_name_from_path(model_path)

    # =======================================================
    # MemVR 关键步骤 1: 替换 LlamaMLP 类定义
    # 必须在 load_pretrained_model 之前执行
    # =======================================================
    print(">>> [MemVR] Patching LlamaMLP...")
    transformers.models.llama.modeling_llama.LlamaMLP = LlamaMLP

    # 2. 加载模型
    print(f">>> Loading model from {model_path}...")
    tokenizer, model, image_processor, context_len = load_pretrained_model(
        model_path, 
        args.model_base, 
        model_name, 
        load_4bit=args.load_4bit, 
        load_8bit=args.load_8bit, 
        device=args.device
    )

    # =======================================================
    # MemVR 关键步骤 2: 应用运行时配置 (Hook & Config)
    # =======================================================
    print(f">>> [MemVR] Activating hooks (Threshold: {args.entropy_threshold}, Ratio: {args.retracing_ratio})...")
    apply_memvr_llama(
        self=model,
        starting_layer=args.starting_layer,
        ending_layer=args.ending_layer,
        entropy_threshold=args.entropy_threshold,
        retracing_ratio=args.retracing_ratio
    )

    # 3. 处理图片
    print(f">>> Processing image: {args.image_file}")
    image = load_image(args.image_file)
    image_tensor = process_images([image], image_processor, model.config)
    
    if type(image_tensor) is list:
        image_tensor = [image.to(model.device, dtype=torch.float16) for image in image_tensor]
    else:
        image_tensor = image_tensor.to(model.device, dtype=torch.float16)

    # 4. 构建 Prompt
    qs = args.query
    if model.config.mm_use_im_start_end:
        qs = DEFAULT_IM_START_TOKEN + DEFAULT_IMAGE_TOKEN + DEFAULT_IM_END_TOKEN + '\n' + qs
    else:
        qs = DEFAULT_IMAGE_TOKEN + '\n' + qs

    conv = conv_templates[args.conv_mode].copy()
    conv.append_message(conv.roles[0], qs)
    conv.append_message(conv.roles[1], None)
    prompt = conv.get_prompt()

    input_ids = tokenizer_image_token(prompt, tokenizer, IMAGE_TOKEN_INDEX, return_tensors='pt').unsqueeze(0).to(model.device)

    # 5. 生成 (Inference)
    print(">>> Generating...")
    with torch.inference_mode():
        output_ids = model.generate(
            input_ids,
            images=image_tensor,
            do_sample=True if args.temperature > 0 else False,
            temperature=args.temperature,
            top_p=args.top_p,
            num_beams=args.num_beams,
            max_new_tokens=args.max_new_tokens,
            use_cache=True
        )

    outputs = tokenizer.batch_decode(output_ids, skip_special_tokens=True)[0].strip()

    print("\n==================================")
    print(f"Prompt: {args.query}")
    print("-" * 30)
    print("Outputs:", outputs)
    print("==================================\n")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    # 基础参数
    parser.add_argument("--model-path", type=str, default="liuhaotian/llava-v1.5-7b")
    parser.add_argument("--model-base", type=str, default=None)
    parser.add_argument("--image-file", type=str, required=True, help="Path to the image file")
    parser.add_argument("--query", type=str, default="Describe the image.", help="User question")
    parser.add_argument("--conv-mode", type=str, default="vicuna_v1")
    parser.add_argument("--device", type=str, default="cuda")
    
    # 量化参数
    parser.add_argument("--load-8bit", action="store_true")
    parser.add_argument("--load-4bit", action="store_true")

    # 生成参数
    parser.add_argument("--temperature", type=float, default=0)
    parser.add_argument("--top_p", type=float, default=None)
    parser.add_argument("--num_beams", type=int, default=1)
    parser.add_argument("--max-new-tokens", type=int, default=1024)

    # MemVR 参数
    parser.add_argument("--retracing-ratio", type=float, default=0.1, help="MemVR retracing ratio (default: 0.1)")
    parser.add_argument("--entropy-threshold", type=float, default=0.75, help="Entropy threshold (default: 0.75)")
    parser.add_argument("--starting-layer", type=int, default=5)
    parser.add_argument("--ending-layer", type=int, default=16)

    args = parser.parse_args()
    main(args)