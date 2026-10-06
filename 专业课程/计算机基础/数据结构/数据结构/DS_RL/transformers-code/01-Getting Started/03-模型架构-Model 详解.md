# Transformer 模型架构：Model 详解

> 本教程深入讲解 Transformer 模型的加载、配置和调用，理解 Base Model 与 Head Model 的区别。

---

## 📖 学习目标

完成本节后，你将能够：
1. 掌握模型的在线/离线加载方式
2. 理解 Model Config 的配置项含义
3. 区分 Base Model 和 Head Model 的调用方式
4. 理解模型输出的各个组成部分
5. 根据任务选择合适的模型类

---

## 1. 模型加载基础

### 1.1 AutoModel 自动加载

**核心思想**：根据预训练模型名称自动加载对应的模型类

```python
from transformers import AutoModel, AutoConfig, AutoTokenizer

# 一行代码加载模型
model = AutoModel.from_pretrained("hfl/rbt3")

print(model)
```

**输出：**
```python
BertModel(
  (embeddings): BertEmbeddings(...)
  (encoder): BertEncoder(
    (layer): ModuleList(
      (0-11): 12 x BertLayer(...)
    )
  )
  (pooler): BertPooler(...)
)
```

### 1.2 在线加载 vs 离线加载

#### 方式一：在线加载（推荐）

```python
# 直接从 HuggingFace Hub 加载
model = AutoModel.from_pretrained("hfl/rbt3")
```

**工作原理：**
```
1. 检查本地缓存 ~/.cache/huggingface/hub/
2. 如果缓存中没有，从 HuggingFace 下载
3. 下载后自动缓存，下次使用无需重复下载
```

#### 方式二：手动下载后离线加载

```bash
# 命令行下载模型
git lfs install
git clone https://huggingface.co/hfl/rbt3
```

```python
# 从本地路径加载
model = AutoModel.from_pretrained("./rbt3")
```

#### 方式三：强制重新下载

```python
# 强制重新下载（忽略缓存）
model = AutoModel.from_pretrained(
    "hfl/rbt3",
    force_download=True
)
```

### 1.3 模型下载参数

```python
model = AutoModel.from_pretrained(
    "hfl/rbt3",

    # 下载相关
    force_download=False,      # 强制重新下载
    resume_download=None,      # 是否允许断点续传
    local_files_only=False,    # 只使用本地文件
    cache_dir=None,            # 自定义缓存目录

    # 代理设置
    proxies=None,              # 代理服务器

    # 权限
    use_auth_token=None,       # API Token
    revision="main",           # 模型版本

    # 其他
    torch_dtype="auto",        # 自动选择数据类型
    low_cpu_mem_usage=False,   # 低 CPU 内存模式
)
```

---

## 2. 模型配置（Config）

### 2.1 查看模型配置

```python
from transformers import AutoModel

model = AutoModel.from_pretrained("hfl/rbt3")

# 查看配置
print(model.config)
```

**输出：**
```python
BertConfig {
  "architectures": ["BertForMaskedLM"],
  "attention_probs_dropout_prob": 0.1,
  "hidden_act": "gelu",
  "hidden_dropout_prob": 0.1,
  "hidden_size": 768,
  "initializer_range": 0.02,
  "intermediate_size": 3072,
  "max_position_embeddings": 512,
  "num_attention_heads": 12,
  "num_hidden_layers": 12,
  "type_vocab_size": 2,
  "vocab_size": 21128
}
```

### 2.2 加载并修改配置

```python
from transformers import AutoConfig, AutoModel

# 只加载配置（不加载模型权重）
config = AutoConfig.from_pretrained("hfl/rbt3")

# 修改配置
config.output_attentions = True      # 输出注意力权重
config.output_hidden_states = True   # 输出所有层隐藏状态

# 使用修改后的配置加载模型
model = AutoModel.from_pretrained(
    "hfl/rbt3",
    config=config
)
```

### 2.3 从配置文件加载

```python
from transformers import BertConfig

# 从本地配置文件加载
config = BertConfig.from_json_file("./rbt3/config.json")

# 或者从字典创建
config_dict = {
    "vocab_size": 21128,
    "hidden_size": 768,
    "num_hidden_layers": 12,
    "num_attention_heads": 12,
    "max_position_embeddings": 512,
}
config = BertConfig(**config_dict)
```

### 2.4 关键配置项说明

| 配置项 | 说明 | 典型值 |
|--------|------|--------|
| `vocab_size` | 词表大小 | 21128 (中文), 30522 (英文) |
| `hidden_size` | 隐藏层维度 | 768 (Base), 1024 (Large) |
| `num_hidden_layers` | Transformer 层数 | 12 (Base), 24 (Large) |
| `num_attention_heads` | 注意力头数 | 12 (Base), 16 (Large) |
| `intermediate_size` | FFN 中间层维度 | 3072 (Base), 4096 (Large) |
| `max_position_embeddings` | 最大位置编码 | 512 |
| `type_vocab_size` | 句子类型词表大小 | 2 (单句/句对) |
| `hidden_dropout_prob` | Dropout 概率 | 0.1 |
| `output_attentions` | 是否输出注意力 | True/False |
| `output_hidden_states` | 是否输出隐藏状态 | True/False |

---

## 3. Base Model vs Head Model

### 3.1 核心区别

```
┌─────────────────────────────────────────────────────────┐
│                    Base Model                            │
│  ┌─────────────────────────────────────────────────┐    │
│  │  Embeddings → Transformer Encoder → Pooler      │    │
│  └─────────────────────────────────────────────────┘    │
│  输出：last_hidden_state, pooler_output                 │
│  用途：特征提取、微调                                    │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                    Head Model                            │
│  ┌─────────────────────────────────────────────────┐    │
│  │  Base Model + Task-specific Head                │    │
│  └─────────────────────────────────────────────────┘    │
│  输出：logits, loss (如果有 labels)                      │
│  用途：特定任务推理、训练                                 │
└─────────────────────────────────────────────────────────┘
```

### 3.2 模型类对照表

| 任务 | Base Model | Head Model |
|------|-----------|------------|
| 通用 | `AutoModel` | - |
| 文本分类 | - | `AutoModelForSequenceClassification` |
| Token 分类 | - | `AutoModelForTokenClassification` |
| 问答 | - | `AutoModelForQuestionAnswering` |
| 掩码语言模型 | - | `AutoModelForMaskedLM` |
| 文本生成 | - | `AutoModelForCausalLM` |
| Seq2Seq | - | `AutoModelForSeq2SeqLM` |

### 3.3 Base Model 调用示例

```python
from transformers import AutoModel, AutoTokenizer
import torch

# 加载 Base Model
model = AutoModel.from_pretrained(
    "hfl/rbt3",
    output_attentions=True,      # 输出注意力
    output_hidden_states=True    # 输出所有层隐藏状态
)
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 准备输入
sen = "弱小的我也有大梦想！"
inputs = tokenizer(sen, return_tensors="pt")

# 模型调用
outputs = model(**inputs)

# 查看输出
print(f"last_hidden_state: {outputs.last_hidden_state.shape}")
print(f"pooler_output: {outputs.pooler_output.shape}")
print(f"hidden_states: {len(outputs.hidden_states)} 层")
print(f"attentions: {len(outputs.attentions)} 层")
```

**输出：**
```python
last_hidden_state: torch.Size([1, 12, 768])
  ├── batch_size: 1
  ├── sequence_length: 12 (token 数)
  └── hidden_size: 768

pooler_output: torch.Size([1, 768])
  └── 用于分类任务的句向量

hidden_states: 13 层
  ├── 0: embedding 层
  └── 1-12: 12 个 Transformer 层

attentions: 12 层
  └── 每层的注意力权重
```

### 3.4 Head Model 调用示例

```python
from transformers import AutoModelForSequenceClassification, AutoTokenizer
import torch

# 加载带分类头的模型
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=2  # 二分类
)
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 准备输入
sen = "弱小的我也有大梦想！"
inputs = tokenizer(sen, return_tensors="pt")

# 模型调用
outputs = model(**inputs)

# 查看输出
print(f"logits: {outputs.logits}")
print(f"logits shape: {outputs.logits.shape}")
```

**输出：**
```python
logits: tensor([[-1.2345, 2.3456]])
logits shape: torch.Size([1, 2])
  ├── batch_size: 1
  └── num_labels: 2
```

---

## 4. 模型调用详解

### 4.1 输入参数

```python
outputs = model(
    input_ids=None,           # 输入 ID
    attention_mask=None,      # 注意力掩码
    token_type_ids=None,      # 句子类型 ID
    position_ids=None,        # 位置 ID
    head_mask=None,           # 注意力头掩码
    inputs_embeds=None,       # 嵌入向量（替代 input_ids）
    labels=None,              # 标签（用于计算 loss）
    output_attentions=None,   # 是否输出注意力
    output_hidden_states=None,# 是否输出隐藏状态
    return_dict=None,         # 是否返回 dict 格式
)
```

### 4.2 完整的推理流程

```python
from transformers import AutoModel, AutoTokenizer
import torch
import torch.nn.functional as F

# 1. 加载模型和分词器
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")
model = AutoModel.from_pretrained("hfl/rbt3")

# 2. 准备输入文本
input_text = "我觉得这家酒店不错！"

# 3. Tokenizer 处理
inputs = tokenizer(
    input_text,
    return_tensors="pt",
    padding=True,
    truncation=True,
    max_length=128
)

print(f"输入：{inputs}")
# {'input_ids': tensor([[101, 2769, ...]]), 'attention_mask': tensor([[1, 1, ...]])}

# 4. 模型推理（Base Model）
with torch.no_grad():
    outputs = model(**inputs)

# 5. 获取 [CLS] token 的输出（句向量）
cls_output = outputs.last_hidden_state[:, 0, :]
print(f"句向量维度：{cls_output.shape}")  # [1, 768]

# 6. 手动添加分类头进行预测
classifier = torch.nn.Linear(768, 2)  # 二分类
logits = classifier(cls_output)

# 7. 计算概率
probabilities = F.softmax(logits, dim=-1)
print(f"概率：{probabilities}")

# 8. 获取预测结果
pred = torch.argmax(probabilities, dim=-1)
print(f"预测类别：{pred.item()}")
```

### 4.3 带标签的训练模式

```python
from transformers import AutoModelForSequenceClassification

# 加载带分类头的模型
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=2
)

# 准备输入
inputs = tokenizer("我觉得不错！", return_tensors="pt")
labels = torch.tensor([1])  # 正面标签

# 传入 labels，模型会自动计算 loss
outputs = model(**inputs, labels=labels)

print(f"loss: {outputs.loss}")      # 自动计算的损失
print(f"logits: {outputs.logits}")  # 预测 logits
```

---

## 5. 模型结构解析

### 5.1 BERT 模型结构图

```
输入："我 爱 你"
        │
        ↓
┌───────────────────────────────┐
│      Token Embeddings         │  [vocab_size, hidden_size]
├───────────────────────────────┤
│   Segment/Token Type Embed.   │  [type_vocab_size, hidden_size]
├───────────────────────────────┤
│    Position Embeddings        │  [max_position_embeddings, hidden_size]
└───────────────────────────────┘
        │ (相加)
        ↓
┌───────────────────────────────┐
│    LayerNorm + Dropout        │
└───────────────────────────────┘
        │
        ↓
┌───────────────────────────────┐
│     Transformer Encoder       │
│  ┌─────────────────────────┐  │
│  │  Self-Attention         │  │
│  │  - Multi-Head Attention │  │
│  │  - Add & Norm           │  │
│  ├─────────────────────────┤  │
│  │  Feed Forward           │  │
│  │  - Add & Norm           │  │
│  └─────────────────────────┘  │
│           × 12 层              │
└───────────────────────────────┘
        │
        ↓
┌───────────────────────────────┐
│        Pooler                 │
│  Dense + Tanh (取 [CLS])      │
└───────────────────────────────┘
```

### 5.2 查看模型参数

```python
# 查看模型总参数量
total_params = sum(p.numel() for p in model.parameters())
print(f"总参数量：{total_params:,}")  # ~110M (BERT-Base)

# 查看可训练参数
trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
print(f"可训练参数：{trainable_params:,}")

# 查看模型结构
print(model)

# 查看具体层
print(model.embeddings)      # 嵌入层
print(model.encoder.layer[0]) # 第一层 Transformer
print(model.pooler)          # Pooler 层
```

### 5.3 提取特定层输出

```python
from transformers import AutoModel, AutoTokenizer

model = AutoModel.from_pretrained(
    "hfl/rbt3",
    output_hidden_states=True
)
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

inputs = tokenizer("弱小的我也有大梦想！", return_tensors="pt")

with torch.no_grad():
    outputs = model(**inputs)

# hidden_states 是一个元组，包含：
# - 第 0 个：embedding 层输出
# - 第 1-12 个：12 个 Transformer 层的输出
hidden_states = outputs.hidden_states

# 获取最后一层的输出
last_hidden = hidden_states[-1]
print(f"最后一层：{last_hidden.shape}")  # [1, seq_len, 768]

# 获取第一层的输出
first_hidden = hidden_states[1]
print(f"第一层：{first_hidden.shape}")

# 获取 [CLS] token 的输出（所有层）
cls_outputs = [hs[:, 0, :] for hs in hidden_states]
print(f"[CLS] 输出：{len(cls_outputs)} 层")
```

---

## 6. 实战：文本分类完整流程

### 6.1 使用 Base Model 构建分类器

```python
import torch
import torch.nn as nn
from transformers import AutoModel, AutoTokenizer

class TextClassifier(nn.Module):
    def __init__(self, model_name="hfl/rbt3", num_labels=2):
        super().__init__()
        # 加载 Base Model
        self.bert = AutoModel.from_pretrained(model_name)
        # 添加分类头
        self.classifier = nn.Linear(768, num_labels)

    def forward(self, input_ids, attention_mask, labels=None):
        # Base Model 输出
        outputs = self.bert(
            input_ids=input_ids,
            attention_mask=attention_mask
        )

        # 取 [CLS] token 的输出
        cls_output = outputs.last_hidden_state[:, 0, :]  # [batch, 768]

        # 分类
        logits = self.classifier(cls_output)  # [batch, num_labels]

        # 计算 loss
        loss = None
        if labels is not None:
            loss_fn = nn.CrossEntropyLoss()
            loss = loss_fn(logits, labels)

        return {"logits": logits, "loss": loss} if loss is not None else {"logits": logits}

# 使用示例
model = TextClassifier()
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 推理
inputs = tokenizer("这家酒店不错！", return_tensors="pt")
outputs = model(**inputs)
probs = torch.softmax(outputs["logits"], dim=-1)
print(f"预测概率：{probs}")
```

### 6.2 使用 Head Model 构建分类器

```python
from transformers import AutoModelForSequenceClassification

# 直接使用带分类头的模型
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=2
)
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 推理
inputs = tokenizer("这家酒店不错！", return_tensors="pt")
outputs = model(**inputs)
probs = torch.softmax(outputs.logits, dim=-1)
print(f"预测概率：{probs}")
```

---

## 7. 模型保存与加载

### 7.1 保存模型

```python
# 保存模型权重和配置
model.save_pretrained("./my_model")

# 同时保存 tokenizer
tokenizer.save_pretrained("./my_model")

# 保存完整模型（包括优化器状态，用于训练恢复）
torch.save({
    'epoch': epoch,
    'model_state_dict': model.state_dict(),
    'optimizer_state_dict': optimizer.state_dict(),
    'loss': loss,
}, "./checkpoint.pt")
```

### 7.2 加载模型

```python
# 从本地加载
from transformers import AutoModel

model = AutoModel.from_pretrained("./my_model")

# 加载训练检查点
checkpoint = torch.load("./checkpoint.pt")
model.load_state_dict(checkpoint['model_state_dict'])
optimizer.load_state_dict(checkpoint['optimizer_state_dict'])
epoch = checkpoint['epoch']
```

---

## 8. 高级用法

### 8.1 部分加载（用于大模型）

```python
# 只加载部分层
model = AutoModel.from_pretrained(
    "hfl/rbt3",
    ignore_mismatched_sizes=True  # 忽略不匹配的层
)

# 加载时指定数据类型（节省显存）
model = AutoModel.from_pretrained(
    "hfl/rbt3",
    torch_dtype=torch.float16  # 半精度
)
```

### 8.2 模型并行

```python
# 多 GPU 数据并行
if torch.cuda.device_count() > 1:
    model = nn.DataParallel(model)

# 模型并行（超大模型）
from accelerate import dispatch_model
from accelerate.utils import infer_auto_device_map

device_map = infer_auto_device_map(model)
model = dispatch_model(model, device_map=device_map)
```

### 8.3 梯度检查点（节省显存）

```python
model.gradient_checkpointing_enable()
# 或
model.config.use_gradient_checkpointing = True
```

---

## 9. 常见问题与解决方案

### 9.1 维度不匹配

**问题**：加载预训练模型时出现维度不匹配错误

```python
# 错误示例
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=10  # 但预训练模型是 2 分类
)

# 解决方案：忽略不匹配的层
model = AutoModelForSequenceClassification.from_pretrained(
    "hfl/rbt3",
    num_labels=10,
    ignore_mismatched_sizes=True
)
```

### 9.2 显存不足

```python
# 方案 1：使用半精度
model = AutoModel.from_pretrained("hfl/rbt3", torch_dtype=torch.float16)

# 方案 2：使用更小的模型
model = AutoModel.from_pretrained("hfl/tiny-bert")

# 方案 3：梯度检查点
model.gradient_checkpointing_enable()
```

### 9.3 CUDA 相关错误

```python
# 确保输入在正确的设备上
model = model.cuda()
inputs = {k: v.cuda() for k, v in inputs.items()}

# 或者使用 device 参数
device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
model = model.to(device)
```

---

## 10. 本节小结

### 10.1 核心知识点

| 知识点 | 关键内容 |
|--------|----------|
| 模型加载 | AutoModel.from_pretrained() |
| 配置管理 | AutoConfig, model.config |
| Base Model | 特征提取，输出 hidden_state |
| Head Model | 任务特定，输出 logits + loss |
| 模型调用 | model(**inputs) |
| 保存加载 | save_pretrained / from_pretrained |

### 10.2 模型选择指南

| 任务类型 | 推荐模型类 |
|----------|-----------|
| 特征提取 | `AutoModel` |
| 文本分类 | `AutoModelForSequenceClassification` |
| 命名实体识别 | `AutoModelForTokenClassification` |
| 问答 | `AutoModelForQuestionAnswering` |
| 文本生成 | `AutoModelForCausalLM` |
| 翻译/摘要 | `AutoModelForSeq2SeqLM` |

### 10.3 下一步学习

1. 学习 [Datasets](04-数据处理-Datasets 详解.md) 进行数据处理
2. 学习 [Evaluate + Trainer](05-训练实战-Evaluate+Trainer.md) 进行模型训练

---

## 附录：完整代码示例

### A.1 完整推理流程

```python
from transformers import AutoModelForSequenceClassification, AutoTokenizer
import torch

# 1. 加载
model = AutoModelForSequenceClassification.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)
tokenizer = AutoTokenizer.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)

# 2. 输入
texts = ["这家酒店不错！", "太差了，再也不会来了"]
inputs = tokenizer(
    texts,
    return_tensors="pt",
    padding=True,
    truncation=True,
    max_length=128
)

# 3. 推理
with torch.no_grad():
    outputs = model(**inputs)

# 4. 后处理
probs = torch.softmax(outputs.logits, dim=-1)
predictions = torch.argmax(probs, dim=-1)

# 5. 输出
id2label = {0: "负面", 1: "正面"}
for text, pred, prob in zip(texts, predictions, probs):
    print(f"文本：{text}")
    print(f"预测：{id2_label[pred.item()]} ({prob[pred].item():.4f})")
```

---

> **提示**：下一节将学习如何使用 Datasets 库进行高效的数据处理和加载。
