# Transformer 快速入门：Pipeline 详解

> 本教程带你从零开始理解 Hugging Face Transformers 的核心组件 —— Pipeline，快速体验 Transformer 的强大能力。

---

## 📖 学习目标

完成本节后，你将能够：
1. 理解 Pipeline 的概念和作用
2. 掌握 Pipeline 的三种创建方式
3. 了解 Pipeline 支持的任务类型
4. 使用 Pipeline 完成实际 NLP 任务
5. 理解 Pipeline 背后的实现原理

---

## 1. 什么是 Pipeline？

### 1.1 核心概念

**Pipeline（管道）** 是 Transformers 库提供的最高级 API，它将复杂的模型推理流程封装成一个简单的接口。

想象一下，使用 Transformer 模型需要以下步骤：
```
输入文本 → Tokenizer → 模型推理 → 后处理 → 输出结果
```

而 Pipeline 将这整个流程封装成一行代码：
```python
from transformers import pipeline
pipe = pipeline("text-classification")
result = pipe("这部电影太棒了！")
```

### 1.2 Pipeline 的优势

| 优势 | 说明 |
|------|------|
| **简洁** | 一行代码完成复杂任务 |
| **高效** | 自动优化推理性能 |
| **灵活** | 支持多种任务类型和模型 |
| **易用** | 无需了解底层细节 |

---

## 2. Pipeline 支持的任务类型

### 2.1 查看支持的任务

```python
from transformers.pipelines import SUPPORTED_TASKS
from pprint import pprint

# 查看所有支持的任务类型
pprint(SUPPORTED_TASKS.keys())
```

**输出示例：**
```
dict_keys([
    'text-classification',      # 文本分类
    'token-classification',      # 命名实体识别
    'question-answering',        # 问答
    'fill-mask',                 # 完形填空
    'summarization',             # 文本摘要
    'translation',               # 翻译
    'text-generation',           # 文本生成
    'zero-shot-classification',  # 零样本分类
    'object-detection',          # 目标检测
    'image-classification',      # 图像分类
    ...
])
```

### 2.2 常见 NLP 任务说明

| 任务 | 描述 | 应用场景 |
|------|------|----------|
| `text-classification` | 文本分类 | 情感分析、垃圾邮件检测 |
| `token-classification` |  token 级分类 | 命名实体识别 |
| `question-answering` | 问答 | 阅读理解、客服问答 |
| `fill-mask` | 完形填空 | 文本补全 |
| `summarization` | 摘要 | 新闻摘要、会议纪要 |
| `translation` | 翻译 | 多语言翻译 |

---

## 3. Pipeline 的三种创建方式

### 3.1 方式一：直接创建（默认模型）

**适合场景**：快速测试、原型开发

```python
from transformers import pipeline

# 使用默认英文模型
pipe = pipeline("text-classification")
result = pipe(["very good!", "very bad!"])
print(result)
```

**输出：**
```python
[
    {'label': 'POSITIVE', 'score': 0.9998},
    {'label': 'NEGATIVE', 'score': 0.9995}
]
```

### 3.2 方式二：指定模型创建

**适合场景**：生产环境、特定任务

```python
from transformers import pipeline

# 指定中文情感分析模型
pipe = pipeline(
    "text-classification",
    model="uer/roberta-base-finetuned-dianping-chinese"
)

result = pipe("我觉得不太行！")
print(result)
```

**输出：**
```python
[{'label': 'NEGATIVE', 'score': 0.9988}]
```

### 3.3 方式三：预先加载模型和分词器

**适合场景**：需要精细控制、批量推理

```python
from transformers import (
    pipeline,
    AutoModelForSequenceClassification,
    AutoTokenizer
)

# 预先加载模型和分词器
model = AutoModelForSequenceClassification.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)
tokenizer = AutoTokenizer.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)

# 创建 Pipeline
pipe = pipeline(
    "text-classification",
    model=model,
    tokenizer=tokenizer
)

result = pipe("我觉得不太行！")
print(result)
```

---

## 4. Pipeline 进阶使用

### 4.1 GPU 加速推理

```python
# 指定使用 GPU（device=0 表示第一块 GPU）
pipe = pipeline(
    "text-classification",
    model="uer/roberta-base-finetuned-dianping-chinese",
    device=0  # 使用 GPU
)

# 验证设备
print(pipe.model.device)  # 输出：cuda:0
```

**性能对比：**
| 设备 | 平均推理时间 |
|------|-------------|
| CPU | ~50ms |
| GPU | ~5ms |

### 4.2 问答 Pipeline 示例

```python
from transformers import pipeline

# 创建问答 Pipeline
qa_pipe = pipeline(
    "question-answering",
    model="uer/roberta-base-chinese-extractive-qa"
)

# 问答任务
result = qa_pipe(
    question="中国的首都是哪里？",
    context="中国的首都是北京，上海是中国最大的城市。"
)

print(result)
```

**输出：**
```python
{
    'answer': '北京',
    'score': 0.9876,
    'start': 5,
    'end': 7
}
```

### 4.3 控制 Pipeline 参数

```python
# 限制答案长度
result = qa_pipe(
    question="中国的首都是哪里？",
    context="中国的首都是北京",
    max_answer_len=1  # 答案最多 1 个字符
)
print(result)  # 输出：{'answer': '北', ...}
```

---

## 5. 实战案例：图像目标检测

### 5.1 零样本目标检测

```python
from transformers import pipeline
from PIL import Image
import requests

# 创建目标检测 Pipeline
detector = pipeline(
    model="google/owlvit-base-patch32",
    task="zero-shot-object-detection"
)

# 加载图片
url = "https://unsplash.com/photos/oj0zeY2Ltk4/download"
image = Image.open(requests.get(url, stream=True).raw)

# 进行检测
predictions = detector(
    image,
    candidate_labels=["hat", "sunglasses", "book"],
)

print(predictions)
```

**输出：**
```python
[
    {
        'score': 0.95,
        'label': 'sunglasses',
        'box': {'xmin': 120, 'ymin': 80, 'xmax': 200, 'ymax': 120}
    },
    ...
]
```

### 5.2 可视化检测结果

```python
from PIL import ImageDraw

draw = ImageDraw.Draw(image)

for prediction in predictions:
    box = prediction["box"]
    label = prediction["label"]
    score = prediction["score"]
    xmin, ymin, xmax, ymax = box.values()

    # 绘制矩形框
    draw.rectangle((xmin, ymin, xmax, ymax), outline="red", width=2)
    # 添加标签
    draw.text((xmin, ymin), f"{label}: {round(score, 2)}", fill="red")

image.show()
```

---

## 6. Pipeline 背后的实现原理

### 6.1 手动实现 Pipeline 流程

理解 Pipeline 背后的流程对于调试和自定义非常重要：

```python
from transformers import AutoTokenizer, AutoModelForSequenceClassification
import torch

# 1. 加载分词器和模型
tokenizer = AutoTokenizer.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)
model = AutoModelForSequenceClassification.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)

# 2. 输入文本
input_text = "我觉得不太行！"

# 3. Tokenizer 处理
inputs = tokenizer(input_text, return_tensors="pt")
print(f"输入张量：{inputs}")
# 输出：{'input_ids': tensor([[101, 2769, ...]]), 'attention_mask': tensor([[1, 1, ...]])}

# 4. 模型推理
outputs = model(**inputs)
print(f"模型输出：{outputs}")

# 5. 获取 logits 并计算概率
logits = outputs.logits
probabilities = torch.softmax(logits, dim=-1)
print(f"概率：{probabilities}")

# 6. 获取预测类别
pred_class = torch.argmax(probabilities).item()
print(f"预测类别 ID: {pred_class}")

# 7. ID 映射到标签
label = model.config.id2label[pred_class]
print(f"预测结果：{label}")
```

### 6.2 Pipeline 内部流程图

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐    ┌──────────────┐
│  输入文本    │ →  │  Tokenizer   │ →  │   模型推理   │ →  │  后处理输出   │
│ "我觉得..."  │    │ 编码为张量    │    │  计算 logits │    │  标签 + 分数   │
└─────────────┘    └──────────────┘    └─────────────┘    └──────────────┘
                          │                   │
                     input_ids           AutoModel
                  attention_mask      ForSequenceClassification
```

---

## 7. 常见问题与解决方案

### 7.1 模型下载慢

**问题**：从 Hugging Face 下载模型速度慢

**解决方案**：
```python
# 方案 1：使用镜像站
import os
os.environ['HF_ENDPOINT'] = 'https://hf-mirror.com'

# 方案 2：手动下载后离线加载
# 先手动下载模型到本地
# 然后使用本地路径加载
model = AutoModel.from_pretrained("./local_model_path")
```

### 7.2 显存不足

**问题**：大模型推理时显存不足

**解决方案**：
```python
# 使用较小的模型
pipe = pipeline("text-classification", model="distilbert-base-uncased")

# 或使用 CPU
pipe = pipeline("text-classification", device=-1)
```

### 7.3 批次处理提高效率

```python
# 单次处理多条文本
texts = ["很好", "不错", "太棒了", "一般"]
results = pipe(texts)
print(results)
```

---

## 8. 实战思路：构建情感分析服务

### 8.1 问题场景

你需要为公司构建一个商品评论情感分析服务。

### 8.2 解决思路

```
1. 任务分析：文本分类（正面/负面）
2. 模型选择：中文评论领域预训练模型
3. 性能要求：批量处理 + GPU 加速
4. 输出格式：情感标签 + 置信度
```

### 8.3 代码实现

```python
from transformers import pipeline

class SentimentAnalyzer:
    def __init__(self, model_name="uer/roberta-base-finetuned-dianping-chinese"):
        # 初始化 Pipeline（GPU 加速）
        self.pipe = pipeline(
            "text-classification",
            model=model_name,
            device=0  # 使用 GPU
        )

        # 标签映射
        self.label_map = {
            "LABEL_0": "负面",
            "LABEL_1": "正面"
        }

    def analyze(self, text):
        """分析单条评论"""
        result = self.pipe(text)[0]
        return {
            "text": text,
            "sentiment": self.label_map[result["label"]],
            "confidence": round(result["score"], 4)
        }

    def batch_analyze(self, texts):
        """批量分析"""
        results = self.pipe(texts)
        return [
            {
                "text": text,
                "sentiment": self.label_map[r["label"]],
                "confidence": round(r["score"], 4)
            }
            for text, r in zip(texts, results)
        ]

# 使用示例
analyzer = SentimentAnalyzer()

# 单条分析
print(analyzer.analyze("这家酒店不错，饭很好吃！"))

# 批量分析
comments = [
    "非常好，下次还会来",
    "太差了，再也不会来了",
    "一般般吧，没什么特别的"
]
for result in analyzer.batch_analyze(comments):
    print(f"{result['text']} → {result['sentiment']} ({result['confidence']:.2%})")
```

---

## 9. 本节小结

### 9.1 核心知识点

| 知识点 | 关键内容 |
|--------|----------|
| Pipeline 概念 | 封装完整的推理流程 |
| 创建方式 | 直接创建、指定模型、预加载模型 |
| 任务类型 | 分类、问答、摘要、翻译等 |
| 性能优化 | GPU 加速、批次处理 |
| 底层原理 | Tokenizer → Model → 后处理 |

### 9.2 下一步学习

1. 深入学习 [Tokenizer](02-文本处理-Tokenzier 详解.md) 的工作原理
2. 了解 [Model](03-模型架构 -Model 详解.md) 的内部结构
3. 学习 [Datasets](04-数据处理-Datasets 详解.md) 进行数据处理

---

## 附录：完整代码清单

### A.1 最小可用示例

```python
from transformers import pipeline

# 一行代码创建情感分析器
classifier = pipeline("sentiment-analysis")

# 一行代码完成分析
result = classifier("I love Transformers!")
print(result)  # [{'label': 'POSITIVE', 'score': 0.9998}]
```

### A.2 中文情感分析完整示例

```python
from transformers import pipeline

# 创建中文情感分析 Pipeline
pipe = pipeline(
    "text-classification",
    model="uer/roberta-base-finetuned-dianping-chinese",
    device=0  # 使用 GPU
)

# 分析评论
comments = [
    "距离川沙公路较近，但是公交指示不对",
    "商务大床房，房间很大，床有 2M 宽，整体感觉经济实惠不错",
    "早餐太差，无论去多少人都不加食品"
]

results = pipe(comments)
for comment, result in zip(comments, results):
    print(f"评论：{comment}")
    print(f"情感：{result['label']}, 置信度：{result['score']:.4f}")
    print("-" * 50)
```

---

> **提示**：下一节将深入讲解 Tokenizer 的工作原理，理解文本如何被转换为模型可理解的数字表示。
