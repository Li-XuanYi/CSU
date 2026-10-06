# Transformer 训练实战：Evaluate + Trainer

> 本教程深入讲解模型评估和训练完整流程，从手动训练到 Trainer 封装，掌握 Transformer 模型训练的完整技能。

---

## 📖 学习目标

完成本节后，你将能够：
1. 掌握 Evaluate 库的使用方法和评估指标计算
2. 理解手动训练的完整流程（循环、优化、验证）
3. 掌握 Trainer 和 TrainingArguments 的配置
4. 能够根据实际需求选择训练方式
5. 完成端到端的文本分类实战项目

---

## 1. Evaluate 库详解

### 1.1 什么是 Evaluate？

**Evaluate** 是 Hugging Face 提供的评估库，提供了统一的评估接口。

```
传统评估方式：
from sklearn.metrics import accuracy_score, f1_score, ...
acc = accuracy_score(y_true, y_pred)
f1 = f1_score(y_true, y_pred)
...

Evaluate 方式：
import evaluate
accuracy = evaluate.load("accuracy")
f1 = evaluate.load("f1")
results = accuracy.compute(predictions=preds, references=labels)
```

### 1.2 查看支持的评估指标

```python
import evaluate

# 查看所有支持的评估指标
metrics = evaluate.list_evaluation_modules()
print(f"支持的评估指标数量：{len(metrics)}")
print(f"部分指标：{metrics[:20]}")
```

**输出：**
```
支持的评估指标数量：200+
部分指标：['accuracy', 'f1', 'precision', 'recall', 'bleu', 'rouge', ...]
```

### 1.3 常见评估指标

| 指标 | 用途 | 加载方式 |
|------|------|----------|
| `accuracy` | 准确率 | `evaluate.load("accuracy")` |
| `f1` | F1 分数 | `evaluate.load("f1")` |
| `precision` | 精确率 | `evaluate.load("precision")` |
| `recall` | 召回率 | `evaluate.load("recall")` |
| `bleu` | 翻译质量 | `evaluate.load("bleu")` |
| `rouge` | 摘要质量 | `evaluate.load("rouge")` |
| `squad` | 问答评估 | `evaluate.load("squad")` |
| `pearsonr` | 相关性 | `evaluate.load("pearsonr")` |

---

## 2. 评估指标计算

### 2.1 加载评估指标

```python
import evaluate

# 加载准确率指标
accuracy = evaluate.load("accuracy")

# 查看指标信息
print(accuracy.description)
print(accuracy.inputs_description)
```

**输出：**
```
Accuracy is the proportion of correct predictions among the total number of cases processed.
It can be computed with:
Accuracy = (TP + TN) / (TP + TN + FP + FN)

Args:
    predictions (`list` of `int`): Predicted labels.
    references (`list` of `int`): Ground truth labels.
    normalize (`boolean`): If set to False, returns the number of correctly classified samples.
    ...
```

### 2.2 全局计算方式

```python
import evaluate

# 加载指标
accuracy = evaluate.load("accuracy")

# 准备数据
references = [0, 1, 2, 0, 1, 2]  # 真实标签
predictions = [0, 1, 1, 2, 1, 0]  # 预测标签

# 一次性计算
results = accuracy.compute(
    references=references,
    predictions=predictions
)
print(results)
# {'accuracy': 0.5}
```

**计算过程：**
```
真实：[0, 1, 2, 0, 1, 2]
预测：[0, 1, 1, 2, 1, 0]
      ✓  ✓  ✗  ✗  ✓  ✗

正确数：3
准确率：3/6 = 0.5
```

### 2.3 迭代计算方式

```python
import evaluate

accuracy = evaluate.load("accuracy")

# 逐条添加（适合流式评估）
for ref, pred in zip([0, 1, 0, 1], [1, 0, 0, 1]):
    accuracy.add(references=ref, predictions=pred)

# 计算最终结果
results = accuracy.compute()
print(results)  # {'accuracy': 0.5}
```

### 2.4 批量添加方式

```python
import evaluate

accuracy = evaluate.load("accuracy")

# 批量添加
for refs, preds in zip(
    [[0, 1], [0, 1]],  # 两批数据
    [[1, 0], [0, 1]]
):
    accuracy.add_batch(references=refs, predictions=preds)

results = accuracy.compute()
print(results)  # {'accuracy': 0.5}
```

### 2.5 组合多个指标

```python
import evaluate

# 组合多个分类指标
clf_metrics = evaluate.combine([
    "accuracy",
    "f1",
    "recall",
    "precision"
])

# 一次性计算所有指标
results = clf_metrics.compute(
    predictions=[0, 1, 0],
    references=[0, 1, 1]
)
print(results)
```

**输出：**
```python
{
    'accuracy': 0.6667,
    'f1': 0.6667,
    'recall': 0.5,
    'precision': 1.0
}
```

**指标解释：**
```
真实：[0, 1, 1]
预测：[0, 1, 0]

混淆矩阵：
              预测 0   预测 1
真实 0    1(TP)    0(FN)
真实 1    1(FP)    1(TN)

准确率 = (1+1)/3 = 0.6667
精确率 = 1/1 = 1.0      (预测为 0 的中，有多少是真的 0)
召回率 = 1/2 = 0.5      (真实为 0 的中，有多少被预测对)
F1 = 2 * (精确率 * 召回率) / (精确率 + 召回率) = 0.6667
```

---

## 3. 手动训练完整流程

### 3.1 训练流程概览

```
┌─────────────────────────────────────────────────────────────┐
│                     训练流程                                 │
│                                                             │
│  加载数据 → 预处理 → 创建 DataLoader → 创建模型 → 训练循环   │
│     ↓         ↓           ↓            ↓          ↓         │
│  Dataset   Tokenize   DataLoader   Model    for epoch      │
│                                                             │
│  训练循环：                                                  │
│  for batch in trainloader:                                  │
│      outputs = model(**batch)                               │
│      loss = outputs.loss                                    │
│      loss.backward()                                        │
│      optimizer.step()                                       │
│                                                             │
│  验证循环：                                                  │
│  for batch in validloader:                                  │
│      outputs = model(**batch)                               │
│      计算评估指标                                            │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 完整代码实现

```python
import torch
from torch.utils.data import DataLoader
from torch.optim import Adam
from transformers import (
    AutoTokenizer,
    AutoModelForSequenceClassification,
    DataCollatorWithPadding
)
from datasets import load_dataset
import evaluate

# ============ Step 1: 加载数据 ============
dataset = load_dataset(
    "csv",
    data_files="./ChnSentiCorp_htl_all.csv",
    split="train"
)
dataset = dataset.filter(lambda x: x["review"] is not None)

# ============ Step 2: 划分数据集 ============
datasets = dataset.train_test_split(test_size=0.1)

# ============ Step 3: 创建 Tokenizer ============
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# ============ Step 4: 预处理 ============
def process_function(examples):
    tokenized = tokenizer(
        examples["review"],
        max_length=128,
        truncation=True
    )
    tokenized["labels"] = examples["label"]
    return tokenized

tokenized_datasets = datasets.map(
    process_function,
    batched=True,
    remove_columns=datasets["train"].column_names
)

# ============ Step 5: 创建 DataLoader ============
collator = DataCollatorWithPadding(tokenizer=tokenizer)

trainloader = DataLoader(
    tokenized_datasets["train"],
    batch_size=32,
    shuffle=True,
    collate_fn=collator
)

validloader = DataLoader(
    tokenized_datasets["test"],
    batch_size=64,
    shuffle=False,
    collate_fn=collator
)

# ============ Step 6: 创建模型 ============
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=2
)

# 移动到 GPU
if torch.cuda.is_available():
    model = model.cuda()

# ============ Step 7: 创建优化器 ============
optimizer = Adam(model.parameters(), lr=2e-5)

# ============ Step 8: 创建评估指标 ============
clf_metrics = evaluate.combine(["accuracy", "f1"])

# ============ Step 9: 训练与验证 ============
def evaluate():
    """验证函数"""
    model.eval()
    with torch.inference_mode():
        for batch in validloader:
            if torch.cuda.is_available():
                batch = {k: v.cuda() for k, v in batch.items()}
            outputs = model(**batch)
            preds = torch.argmax(outputs.logits, dim=-1)
            clf_metrics.add_batch(
                predictions=preds.long(),
                references=batch["labels"].long()
            )
    return clf_metrics.compute()

def train(epoch=3, log_step=100):
    """训练函数"""
    global_step = 0
    for ep in range(epoch):
        model.train()
        for batch in trainloader:
            if torch.cuda.is_available():
                batch = {k: v.cuda() for k, v in batch.items()}

            # 前向传播
            optimizer.zero_grad()
            outputs = model(**batch)
            loss = outputs.loss

            # 反向传播
            loss.backward()
            optimizer.step()

            # 打印日志
            if global_step % log_step == 0:
                print(f"Epoch: {ep}, Step: {global_step}, Loss: {loss.item():.4f}")
            global_step += 1

        # 每个 epoch 结束后评估
        metrics = evaluate()
        print(f"Epoch: {ep}, Metrics: {metrics}")

# 开始训练
train(epoch=3)
```

**训练输出示例：**
```
Epoch: 0, Step: 0, Loss: 0.6931
Epoch: 0, Step: 100, Loss: 0.3421
Epoch: 0, Step: 200, Loss: 0.2156
Epoch: 0, Metrics: {'accuracy': 0.8912, 'f1': 0.8901}
Epoch: 1, Step: 300, Loss: 0.1823
...
Epoch: 2, Metrics: {'accuracy': 0.9234, 'f1': 0.9228}
```

### 3.3 模型预测

```python
# 模型预测
sen = "我觉得这家酒店不错，饭很好吃！"
id2label = {0: "差评！", 1: "好评！"}

model.eval()
with torch.inference_mode():
    inputs = tokenizer(sen, return_tensors="pt")
    if torch.cuda.is_available():
        inputs = {k: v.cuda() for k, v in inputs.items()}
    logits = model(**inputs).logits
    pred = torch.argmax(logits, dim=-1)
    print(f"输入：{sen}")
    print(f"预测结果：{id2label.get(pred.item())}")
```

### 3.4 使用 Pipeline 进行预测

```python
from transformers import pipeline

# 设置标签映射
model.config.id2label = id2label

# 创建 Pipeline
pipe = pipeline(
    "text-classification",
    model=model,
    tokenizer=tokenizer,
    device=0 if torch.cuda.is_available() else -1
)

# 预测
result = pipe(sen)
print(result)
# [{'label': '好评！', 'score': 0.9876}]
```

---

## 4. Trainer 训练方式

### 4.1 为什么使用 Trainer？

**手动训练的问题：**
- 需要写大量样板代码
- 梯度累积、混合精度等需要自己实现
- 检查点保存、日志记录需要自己处理
- 分布式训练配置复杂

**Trainer 的优势：**
- 简洁的 API
- 内置最佳实践（梯度累积、混合精度等）
- 自动检查点保存
- 支持分布式训练
- 与 Hugging Face 生态无缝集成

### 4.2 TrainingArguments 配置

```python
from transformers import TrainingArguments

train_args = TrainingArguments(
    # 输出设置
    output_dir="./checkpoints",          # 输出文件夹
    logging_dir="./logs",                # 日志文件夹

    # 批次大小
    per_device_train_batch_size=64,      # 训练 batch_size
    per_device_eval_batch_size=128,      # 验证 batch_size

    # 学习率
    learning_rate=2e-5,                  # 学习率
    weight_decay=0.01,                   # 权重衰减

    # 训练轮数
    num_train_epochs=3,                  # 训练轮数

    # 日志与保存
    logging_steps=10,                    # log 打印频率
    evaluation_strategy="epoch",         # 评估策略
    save_strategy="epoch",               # 保存策略
    save_total_limit=3,                  # 最大保存检查点数

    # 其他
    load_best_model_at_end=True,         # 训练完成后加载最优模型
    metric_for_best_model="f1",          # 用于选择最优模型的指标
    greater_is_better=True,              # 指标越大越好
    seed=42,                             # 随机种子

    # GPU 相关
    fp16=False,                          # 是否使用混合精度
    gradient_accumulation_steps=1,       # 梯度累积步数
)

print(train_args)
```

### 4.3 评估函数定义

```python
import evaluate

# 加载评估指标
acc_metric = evaluate.load("accuracy")
f1_metric = evaluate.load("f1")

def compute_metrics(eval_pred):
    """
    Trainer 的评估函数

    Args:
        eval_pred: EvalPrediction 对象，包含 predictions 和 label_ids

    Returns:
        评估指标字典
    """
    predictions, labels = eval_pred

    # 获取预测类别（取概率最大的）
    preds = predictions.argmax(axis=-1)

    # 计算指标
    acc = acc_metric.compute(predictions=preds, references=labels)
    f1 = f1_metric.compute(predictions=preds, references=labels)

    # 合并结果
    acc.update(f1)
    return acc
```

### 4.4 创建 Trainer

```python
from transformers import (
    Trainer,
    AutoTokenizer,
    AutoModelForSequenceClassification,
    DataCollatorWithPadding
)
from datasets import load_dataset

# ============ Step 1-4: 数据加载和预处理（同上） ============
dataset = load_dataset("csv", data_files="./ChnSentiCorp_htl_all.csv", split="train")
dataset = dataset.filter(lambda x: x["review"] is not None)
datasets = dataset.train_test_split(test_size=0.1)

tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

def process_function(examples):
    tokenized = tokenizer(examples["review"], max_length=128, truncation=True)
    tokenized["labels"] = examples["label"]
    return tokenized

tokenized_datasets = datasets.map(
    process_function,
    batched=True,
    remove_columns=datasets["train"].column_names
)

# ============ Step 5: 创建模型 ============
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=2
)

# ============ Step 6: 创建 TrainingArguments ============
train_args = TrainingArguments(
    output_dir="./checkpoints",
    per_device_train_batch_size=64,
    per_device_eval_batch_size=128,
    logging_steps=10,
    evaluation_strategy="epoch",
    save_strategy="epoch",
    save_total_limit=3,
    learning_rate=2e-5,
    weight_decay=0.01,
    metric_for_best_model="f1",
    load_best_model_at_end=True,
)

# ============ Step 7: 创建 Trainer ============
trainer = Trainer(
    model=model,                              # 模型
    args=train_args,                          # 训练参数
    train_dataset=tokenized_datasets["train"], # 训练集
    eval_dataset=tokenized_datasets["test"],   # 验证集
    data_collator=DataCollatorWithPadding(     # 数据整理器
        tokenizer=tokenizer
    ),
    compute_metrics=compute_metrics,          # 评估函数
)
```

### 4.5 训练与评估

```python
# 开始训练
trainer.train()
```

**训练输出示例：**
```
***** Running training *****
  Num examples = 6989
  Num Epochs = 3
  Instantaneous batch size per device = 64
  Total train batch size (w. parallel, distributed & accumulation) = 64
  Total optimization steps = 327

  Epoch    Training Loss    Validation Loss    Accuracy    F1
  0        0.234567         0.198765           0.891234    0.890123
  1        0.156789         0.167890           0.912345    0.911234
  2        0.123456         0.156789           0.923456    0.922890
```

### 4.6 模型评估

```python
# 在测试集上评估
results = trainer.evaluate(tokenized_datasets["test"])
print(results)
```

**输出：**
```python
{
    'eval_loss': 0.156789,
    'eval_accuracy': 0.923456,
    'eval_f1': 0.922890,
    'eval_runtime': 5.4321,
    'eval_samples_per_second': 128.345,
    'eval_steps_per_second': 2.012
}
```

### 4.7 模型预测

```python
# 使用 Trainer 进行预测
predictions = trainer.predict(tokenized_datasets["test"])

print(predictions)
# PredictionOutput(
#     predictions=array([[...], [...]]),  # logits
#     label_ids=array([0, 1, 1, ...]),
#     metrics={'test_loss': 0.156, 'test_accuracy': 0.923, ...}
# )

# 获取预测结果
preds = predictions.predictions.argmax(axis=-1)
print(f"预测标签：{preds[:10]}")
```

---

## 5. 三种训练方式对比

### 5.1 方式对比表

| 特性 | 手动训练 | Trainer | 自定义 TrainingLoop |
|------|----------|---------|---------------------|
| 代码量 | 多 | 少 | 中等 |
| 灵活性 | 高 | 中 | 高 |
| 内置功能 | 无 | 完整 | 自选 |
| 学习曲线 | 陡 | 平缓 | 中等 |
| 推荐场景 | 学习原理 | 生产环境 | 特殊需求 |

### 5.2 代码量对比

**手动训练（~100 行）：**
```python
# 需要手动实现：
# - 训练循环
# - 验证循环
# - 优化器步进
# - 梯度清零
# - 日志记录
# - 检查点保存
# - 评估指标计算
```

**Trainer（~30 行）：**
```python
trainer = Trainer(
    model=model,
    args=train_args,
    train_dataset=train_dataset,
    eval_dataset=eval_dataset,
    compute_metrics=compute_metrics,
)
trainer.train()
```

---

## 6. 实战：完整项目流程

### 6.1 项目结构

```
sentiment_analysis/
├── data/
│   └── ChnSentiCorp_htl_all.csv
├── checkpoints/              # 模型检查点
├── logs/                     # 训练日志
├── train.py                  # 训练脚本
├── predict.py                # 预测脚本
└── requirements.txt          # 依赖
```

### 6.2 完整训练脚本

```python
# train.py
import torch
from transformers import (
    AutoTokenizer,
    AutoModelForSequenceClassification,
    TrainingArguments,
    Trainer,
    DataCollatorWithPadding
)
from datasets import load_dataset
import evaluate
import numpy as np

# ============ 配置 ============
MODEL_NAME = "hfl/rbt3"
DATA_FILE = "./data/ChnSentiCorp_htl_all.csv"
OUTPUT_DIR = "./checkpoints"
NUM_LABELS = 2
EPOCHS = 3
BATCH_SIZE = 64
LEARNING_RATE = 2e-5

# ============ 加载数据 ============
print("Loading data...")
dataset = load_dataset("csv", data_files=DATA_FILE, split="train")
dataset = dataset.filter(lambda x: x["review"] is not None)
datasets = dataset.train_test_split(test_size=0.1, stratify_by_column="label")
print(f"Train: {len(datasets['train'])}, Test: {len(datasets['test'])}")

# ============ 创建 Tokenizer ============
print("Loading tokenizer...")
tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME)

# ============ 预处理 ============
def process_function(examples):
    tokenized = tokenizer(
        examples["review"],
        max_length=128,
        truncation=True,
        padding=False
    )
    tokenized["labels"] = examples["label"]
    return tokenized

print("Preprocessing...")
tokenized_datasets = datasets.map(
    process_function,
    batched=True,
    remove_columns=datasets["train"].column_names
)

# ============ 创建模型 ============
print("Loading model...")
model = AutoModelForSequenceClassification.from_pretrained(
    MODEL_NAME,
    num_labels=NUM_LABELS
)

# ============ 评估函数 ============
acc_metric = evaluate.load("accuracy")
f1_metric = evaluate.load("f1")

def compute_metrics(eval_pred):
    predictions, labels = eval_pred
    preds = predictions.argmax(axis=-1)
    acc = acc_metric.compute(predictions=preds, references=labels)
    f1 = f1_metric.compute(predictions=preds, references=labels)
    acc.update(f1)
    return acc

# ============ 训练参数 ============
train_args = TrainingArguments(
    output_dir=OUTPUT_DIR,
    per_device_train_batch_size=BATCH_SIZE,
    per_device_eval_batch_size=BATCH_SIZE * 2,
    learning_rate=LEARNING_RATE,
    num_train_epochs=EPOCHS,
    weight_decay=0.01,
    logging_steps=10,
    evaluation_strategy="epoch",
    save_strategy="epoch",
    save_total_limit=2,
    load_best_model_at_end=True,
    metric_for_best_model="f1",
    seed=42,
)

# ============ 创建 Trainer ============
trainer = Trainer(
    model=model,
    args=train_args,
    train_dataset=tokenized_datasets["train"],
    eval_dataset=tokenized_datasets["test"],
    data_collator=DataCollatorWithPadding(tokenizer=tokenizer),
    compute_metrics=compute_metrics,
)

# ============ 开始训练 ============
print("Starting training...")
trainer.train()

# ============ 保存模型 ============
print("Saving model...")
trainer.save_model(OUTPUT_DIR)
tokenizer.save_pretrained(OUTPUT_DIR)

# ============ 最终评估 ============
print("Final evaluation...")
results = trainer.evaluate()
print(results)

print("Training completed!")
```

### 6.3 预测脚本

```python
# predict.py
from transformers import AutoModelForSequenceClassification, AutoTokenizer
import torch

# 配置
MODEL_DIR = "./checkpoints"
ID2LABEL = {0: "负面", 1: "正面"}

# 加载模型和分词器
print("Loading model...")
tokenizer = AutoTokenizer.from_pretrained(MODEL_DIR)
model = AutoModelForSequenceClassification.from_pretrained(MODEL_DIR)

if torch.cuda.is_available():
    model = model.cuda()

model.eval()

# 预测函数
def predict(text):
    inputs = tokenizer(text, return_tensors="pt")
    if torch.cuda.is_available():
        inputs = {k: v.cuda() for k, v in inputs.items()}

    with torch.inference_mode():
        outputs = model(**inputs)
        probs = torch.softmax(outputs.logits, dim=-1)
        pred = torch.argmax(probs, dim=-1)

    return {
        "text": text,
        "label": ID2LABEL[pred.item()],
        "confidence": probs[0, pred].item()
    }

# 批量预测
def batch_predict(texts):
    inputs = tokenizer(
        texts,
        return_tensors="pt",
        padding=True,
        truncation=True,
        max_length=128
    )
    if torch.cuda.is_available():
        inputs = {k: v.cuda() for k, v in inputs.items()}

    with torch.inference_mode():
        outputs = model(**inputs)
        probs = torch.softmax(outputs.logits, dim=-1)
        preds = torch.argmax(probs, dim=-1)

    results = []
    for text, pred, prob in zip(texts, preds, probs):
        results.append({
            "text": text,
            "label": ID2LABEL[pred.item()],
            "confidence": prob[pred].item()
        })
    return results

# 使用示例
if __name__ == "__main__":
    # 单条预测
    text = "我觉得这家酒店不错，饭很好吃！"
    result = predict(text)
    print(f"文本：{result['text']}")
    print(f"情感：{result['label']} ({result['confidence']:.2%})")

    # 批量预测
    texts = [
        "非常好，下次还会来",
        "太差了，再也不会来了",
        "一般般吧，没什么特别的"
    ]
    results = batch_predict(texts)
    for r in results:
        print(f"{r['text']} → {r['label']} ({r['confidence']:.2%})")
```

---

## 7. 高级技巧

### 7.1 早停（Early Stopping）

```python
from transformers import Trainer, TrainingArguments
from transformers.trainer_callback import TrainerCallback, EarlyStoppingCallback

# 方式 1：使用内置回调
train_args = TrainingArguments(
    ...
    load_best_model_at_end=True,
    metric_for_best_model="f1",
    greater_is_better=True,
)

trainer = Trainer(
    model=model,
    args=train_args,
    callbacks=[EarlyStoppingCallback(early_stopping_patience=3)]
)

# 方式 2：自定义回调
class CustomCallback(TrainerCallback):
    def on_epoch_end(self, args, state, control, metrics=None, **kwargs):
        if metrics.get("eval_f1", 0) > 0.95:
            control.should_training_stop = True
            print("F1 > 0.95, stop training!")

trainer.add_callback(CustomCallback())
```

### 7.2 学习率调度

```python
from transformers import TrainingArguments

train_args = TrainingArguments(
    ...
    lr_scheduler_type="cosine",  # 余弦退火
    # 其他选项：linear, polynomial, constant_with_warmup
    warmup_ratio=0.1,            # 10% 步数用于 warmup
)
```

### 7.3 梯度累积

```python
train_args = TrainingArguments(
    ...
    per_device_train_batch_size=16,     # 实际 batch_size
    gradient_accumulation_steps=4,      # 累积 4 步
    # 等效 batch_size = 16 * 4 = 64
)
```

### 7.4 混合精度训练

```python
train_args = TrainingArguments(
    ...
    fp16=True,                          # 启用混合精度
    fp16_opt_level="O1",                # 优化级别
)
```

---

## 8. 常见问题与解决方案

### 8.1 过拟合

**症状**：训练集准确率高，验证集准确率低

**解决方案：**
```python
train_args = TrainingArguments(
    ...
    weight_decay=0.01,          # 权重衰减
    dropout=0.1,                # Dropout
    warmup_ratio=0.1,           # Warmup
)

# 或者在模型中设置
model.config.hidden_dropout_prob = 0.2
model.config.attention_probs_dropout_prob = 0.2
```

### 8.2 训练慢

**解决方案：**
```python
train_args = TrainingArguments(
    ...
    fp16=True,                          # 混合精度
    dataloader_num_workers=4,           # 多进程数据加载
    gradient_accumulation_steps=4,      # 梯度累积（增大 batch_size）
)
```

### 8.3 显存不足

**解决方案：**
```python
train_args = TrainingArguments(
    ...
    per_device_train_batch_size=8,      # 减小 batch_size
    gradient_accumulation_steps=8,      # 增加累积步数
    fp16=True,                          # 混合精度
    gradient_checkpointing=True,        # 梯度检查点
)
```

### 8.4 类别不平衡

```python
# 使用加权损失
from transformers import AutoModelForSequenceClassification
import torch.nn as nn

model = AutoModelForSequenceClassification.from_pretrained("hfl/rbt3", num_labels=2)

# 计算类别权重
class_weights = torch.tensor([1.0, 3.0])  # 少数类权重更高
criterion = nn.CrossEntropyLoss(weight=class_weights)

# 自定义 Trainer 重写 compute_loss
class CustomTrainer(Trainer):
    def compute_loss(self, model, inputs, return_outputs=False):
        labels = inputs.pop("labels")
        outputs = model(**inputs)
        logits = outputs.logits
        loss = criterion(logits, labels)
        return (loss, outputs) if return_outputs else loss
```

---

## 9. 本节小结

### 9.1 核心知识点

| 知识点 | 关键内容 |
|--------|----------|
| Evaluate | 加载指标、计算、组合 |
| 手动训练 | 训练循环、验证循环、优化器 |
| Trainer | TrainingArguments、compute_metrics |
| 训练技巧 | 早停、学习率调度、梯度累积、混合精度 |

### 9.2 训练方式选择

```
学习原理 → 手动训练
生产环境 → Trainer
特殊需求 → 自定义 TrainingLoop
```

### 9.3 完整流程图

```
数据准备 → 预处理 → 模型加载 → 训练配置 → 训练 → 评估 → 预测
   ↓          ↓          ↓          ↓          ↓       ↓       ↓
Dataset   Tokenize   AutoModel  Arguments  Trainer  evaluate  Pipeline
```

---

## 附录：完整代码清单

### A.1 最小可用 Trainer 示例

```python
from transformers import (
    AutoTokenizer,
    AutoModelForSequenceClassification,
    TrainingArguments,
    Trainer,
    DataCollatorWithPadding
)
from datasets import load_dataset
import evaluate

# 数据
dataset = load_dataset("csv", data_files="./data.csv", split="train")
datasets = dataset.train_test_split(test_size=0.1)

# Tokenizer
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 预处理
def preprocess(examples):
    tokenized = tokenizer(examples["text"], truncation=True, max_length=128)
    tokenized["labels"] = examples["label"]
    return tokenized

tokenized_datasets = datasets.map(preprocess, batched=True, remove_columns=dataset.column_names)

# 模型
model = AutoModelForSequenceClassification.from_pretrained("hfl/rbt3", num_labels=2)

# 评估
metric = evaluate.load("accuracy")
def compute_metrics(eval_pred):
    preds = eval_pred.predictions.argmax(-1)
    return metric.compute(predictions=preds, references=eval_pred.label_ids)

# 训练
args = TrainingArguments(
    output_dir="./output",
    per_device_train_batch_size=32,
    num_train_epochs=3,
    evaluation_strategy="epoch",
    save_strategy="epoch",
    load_best_model_at_end=True,
)

trainer = Trainer(
    model=model,
    args=args,
    train_dataset=tokenized_datasets["train"],
    eval_dataset=tokenized_datasets["test"],
    data_collator=DataCollatorWithPadding(tokenizer),
    compute_metrics=compute_metrics,
)

trainer.train()
```

---

> **恭喜！** 你已经完成了 Transformer 基础架构的完整学习。现在你可以：
> 1. 使用 Pipeline 快速推理
> 2. 使用 Tokenizer 处理文本
> 3. 使用 Model 进行模型调用
> 4. 使用 Datasets 处理数据
> 5. 使用 Trainer 训练模型
>
> 接下来可以尝试更多实际项目，如文本分类、命名实体识别、问答系统等！
