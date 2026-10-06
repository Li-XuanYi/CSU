#!/bin/bash
# 如果运行脚本时没有提供参数，则使用默认的 URL
export CUDA_VISIBLE_DEVICES=0

IMAGE_FILE="/home/huiwei/sy/visual/dataset/7.png"

echo "Running LLaVA inference on: $IMAGE_FILE"

python -m eval.run_memvr_inference \
    --model-path /home/huiwei/llava-v1.5-7b \
    --image-file "$IMAGE_FILE" \
    --load-4bit \
    --entropy-threshold 0.75 \
    --retracing-ratio 0.2 \
    --temperature 0 \
    --query "李佳洋是谁？图里有多少辆车？" \
