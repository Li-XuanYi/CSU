import argparse
import os
import random
import json
import csv
import torch
import numpy as np
from tqdm import tqdm
from PIL import Image
import matplotlib.pyplot as plt
import sys

# Add the current directory to sys.path to import cal_chair
sys.path.append(os.path.dirname(os.path.abspath(__file__)))
from cal_chair import CHAIR

from llava.constants import IMAGE_TOKEN_INDEX, DEFAULT_IMAGE_TOKEN
from llava.conversation import conv_templates
from llava.model.builder import load_pretrained_model
from llava.utils import disable_torch_init
from llava.mm_utils import tokenizer_image_token, get_model_name_from_path

def parse_args():
    parser = argparse.ArgumentParser(description="Layer-wise MLP Zero-Ablation Experiment")
    parser.add_argument("--model-path", type=str, default="llava-v1.5-7b")
    parser.add_argument("--model-base", type=str, default=None)
    parser.add_argument("--image-folder", type=str, default="/home/huiwei/sy/LLaVA-main/playground/data/eval/chair/val2014")
    parser.add_argument("--question-file", type=str, default="/home/huiwei/sy/LLaVA-main/playground/data/eval/chair/annotations/instances_val2014.json")
    parser.add_argument("--coco-path", type=str, default="/home/huiwei/sy/LLaVA-main/playground/data/eval/chair/annotations")
    parser.add_argument("--output-dir", type=str, default="./ablation_results")
    parser.add_argument("--num-samples", type=int, default=75)
    parser.add_argument("--conv-mode", type=str, default="vicuna_v1")
    parser.add_argument("--temperature", type=float, default=0)
    parser.add_argument("--top_p", type=float, default=None)
    parser.add_argument("--num_beams", type=int, default=1)
    parser.add_argument("--max-new-tokens", type=int, default=64) # Reduced for speed
    parser.add_argument("--layer-start", type=int, default=0, help="Start layer index for ablation")
    parser.add_argument("--layer-end", type=int, default=32, help="End layer index for ablation (exclusive)")
    parser.add_argument("--run-baseline", action="store_true", help="Run baseline inference")
    parser.add_argument("--baseline-score", type=float, default=None, help="Pre-computed baseline CHAIRs score")
    return parser.parse_args()

def get_mlp_modules(model):
    # Assuming Llama structure: model.model.layers[i].mlp
    return [layer.mlp for layer in model.model.layers]

def register_mlp_hook(mlp_module):
    def hook(module, input, output):
        return torch.zeros_like(output)
    return mlp_module.register_forward_hook(hook)

def run_inference(model, tokenizer, image_processor, img_files, img_dict, args, output_file):
    # Check if file exists to resume
    finished_ids = set()
    if os.path.exists(output_file):
        print(f"Output file {output_file} exists. Checking for resume...")
        with open(output_file, 'r') as f:
            for line in f:
                try:
                    data = json.loads(line)
                    finished_ids.add(data['image_id'])
                except json.JSONDecodeError:
                    continue
        print(f"Found {len(finished_ids)} processed images.")
        
        if len(finished_ids) == len(img_files):
            print("All images processed. Skipping inference.")
            return output_file

    # Open in append mode
    with open(output_file, 'a') as f_out:
        for i, img_file in tqdm(enumerate(img_files), total=len(img_files), desc="Inference"):
            img_id = int(img_file.split(".jpg")[0][-6:])
            
            if img_id in finished_ids:
                continue
            
            image_path = os.path.join(args.image_folder, img_file)
            if not os.path.exists(image_path):
                print(f"Image not found: {image_path}")
                continue
                
            raw_image = Image.open(image_path).convert("RGB")
            image_tensor = image_processor.preprocess(raw_image, return_tensors='pt')['pixel_values'][0].unsqueeze(0).to(model.device, dtype=torch.float16)
            
            qs = "Please describe this image in detail."
            qs = DEFAULT_IMAGE_TOKEN + '\n' + qs
            conv = conv_templates[args.conv_mode].copy()
            conv.append_message(conv.roles[0], qs)
            conv.append_message(conv.roles[1], None)
            prompt = conv.get_prompt()
            
            input_ids = tokenizer_image_token(prompt, tokenizer, IMAGE_TOKEN_INDEX, return_tensors='pt').unsqueeze(0).to(model.device)
            
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
            
            result = {
                "image_id": img_id,
                "caption": outputs
            }
            
            # Write immediately
            f_out.write(json.dumps(result) + "\n")
            f_out.flush()
    
    return output_file

def calculate_chair(chair_evaluator, cap_file):
    # cap_file is jsonl
    cap_dict = chair_evaluator.compute_chair(cap_file, "image_id", "caption")
    # Return CHAIRs and CHAIRi
    return cap_dict['overall_metrics']['CHAIRs'], cap_dict['overall_metrics']['CHAIRi']

def main():
    args = parse_args()
    disable_torch_init()
    
    if not os.path.exists(args.output_dir):
        os.makedirs(args.output_dir)
        
    # Load Model
    print("Loading model...")
    model_name = get_model_name_from_path(args.model_path)
    tokenizer, model, image_processor, context_len = load_pretrained_model(args.model_path, args.model_base, model_name)
    
    # Prepare Data
    print("Preparing data...")
    with open(args.question_file, 'r') as f:
        coco_anns = json.load(f)
    
    img_dict = {}
    all_img_files = []
    for img_info in coco_anns["images"]:
        img_dict[img_info["id"]] = img_info
        all_img_files.append(img_info["file_name"])
        
    # Random sample
    random.seed(42)
    selected_img_files = random.sample(all_img_files, args.num_samples)
    print(f"Selected {len(selected_img_files)} images for evaluation.")
    
    # Initialize CHAIR evaluator
    print("Initializing CHAIR evaluator...")
    chair_evaluator = CHAIR(args.coco_path)
    
    # Initialize CSV logging for this part
    csv_file = os.path.join(args.output_dir, f"summary_scores_part_{args.layer_start}_{args.layer_end}.csv")
    # Always write header for partial files as they are new
    with open(csv_file, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['layer_idx', 'CHAIRs', 'CHAIRi', 'delta_CHAIRs'])

    baseline_chairs = args.baseline_score

    # Baseline
    if args.run_baseline:
        print("Running Baseline...")
        baseline_file = os.path.join(args.output_dir, "baseline.jsonl")
        run_inference(model, tokenizer, image_processor, selected_img_files, img_dict, args, baseline_file)
        baseline_chairs, baseline_chairi = calculate_chair(chair_evaluator, baseline_file)
        print(f"Baseline CHAIRs: {baseline_chairs}, CHAIRi: {baseline_chairi}")
        
        # Save baseline score to a file for other processes to read if needed (though we pass via args)
        with open(os.path.join(args.output_dir, "baseline_score.txt"), "w") as f:
            f.write(str(baseline_chairs))
    
    if baseline_chairs is None:
        # Try to read from file if not passed and not running baseline
        baseline_score_path = os.path.join(args.output_dir, "baseline_score.txt")
        if os.path.exists(baseline_score_path):
             with open(baseline_score_path, "r") as f:
                baseline_chairs = float(f.read().strip())
        else:
            print("Warning: No baseline score provided or found. Delta will be calculated against 0 or fail.")
            # For safety, if we can't get baseline, we might just log raw scores.
            # But let's assume the user follows instructions.
            baseline_chairs = 0.0

    # Layer-wise Ablation
    num_layers = len(model.model.layers)
    # Validate range
    start_idx = max(0, args.layer_start)
    end_idx = min(num_layers, args.layer_end)
    
    print(f"Running Ablation on layers {start_idx} to {end_idx}...")
    
    for layer_idx in range(start_idx, end_idx):
        print(f"Ablating Layer {layer_idx}...")
        
        output_file = os.path.join(args.output_dir, f"ablation_layer_{layer_idx}.jsonl")
        
        # Register Hook
        mlp_module = model.model.layers[layer_idx].mlp
        hook_handle = register_mlp_hook(mlp_module)
        
        try:
            # Run Inference
            run_inference(model, tokenizer, image_processor, selected_img_files, img_dict, args, output_file)
        finally:
            # Remove Hook
            hook_handle.remove()
            torch.cuda.empty_cache()
        
        # Calculate Score
        try:
            chairs, chairi = calculate_chair(chair_evaluator, output_file)
            print(f"Layer {layer_idx} CHAIRs: {chairs}, CHAIRi: {chairi}")
            score = chairs - baseline_chairs
        except ZeroDivisionError:
            # Language collapse, not hallucination. Assign NaN.
            chairs = np.nan
            chairi = np.nan
            score = np.nan
            print(f"Layer {layer_idx} | ZeroDivisionError caught. Assigned NaN.")
        
        # Real-time logging
        with open(csv_file, 'a', newline='') as f:
            writer = csv.writer(f)
            writer.writerow([layer_idx, chairs, chairi, score])
            
    print(f"Finished ablation for layers {start_idx} to {end_idx}.")
    # Removed visualization and top-5 logic from here as it will be handled by a separate script


if __name__ == "__main__":
    main()
