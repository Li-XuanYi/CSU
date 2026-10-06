# Transformer 数据处理：Datasets 详解

> 本教程深入讲解 Hugging Face Datasets 库的使用，掌握高效数据加载、处理和批量化操作。

---

## 📖 学习目标

完成本节后，你将能够：
1. 掌握在线/离线数据集的加载方法
2. 理解 Dataset 的数据结构和访问方式
3. 熟练运用 map、filter 等数据处理方法
4. 掌握数据集划分、保存与加载
5. 理解 DataCollator 的作用和使用
6. 构建完整的训练数据管道

---

## 1. Datasets 库简介

### 1.1 为什么使用 Datasets？

传统数据处理方式的问题：
```python
# 传统方式：一次性加载所有数据到内存
import pandas as pd
data = pd.read_csv("large_file.csv")  # 内存爆炸！

# 手动处理数据划分
from sklearn.model_selection import train_test_split
train, test = train_test_split(data, test_size=0.1)

# 手动创建 DataLoader
from torch.utils.data import Dataset, DataLoader
class MyDataset(Dataset):
    def __init__(self, data):
        self.data = data
    def __getitem__(self, idx):
        return self.data.iloc[idx]
    def __len__(self):
        return len(self.data)
```

使用 Datasets 的优势：
```python
from datasets import load_dataset

# 一行代码加载数据集（内存映射，不占用大量内存）
dataset = load_dataset("csv", data_files="large_file.csv")

# 一行代码划分数据集
datasets = dataset["train"].train_test_split(test_size=0.1)

# 自动支持高效的数据处理
dataset = dataset.map(process_function, batched=True)
```

### 1.2 核心概念

| 概念 | 说明 |
|------|------|
| `Dataset` | 单个数据集，支持内存映射 |
| `DatasetDict` | 多个 Dataset 的字典（train/validation/test） |
| `Features` | 数据列的类型定义 |
| `map` | 数据转换/处理 |
| `filter` | 数据过滤 |

---

## 2. 加载数据集

### 2.1 加载在线数据集

```python
from datasets import load_dataset

# 方式 1：直接加载（返回 DatasetDict）
datasets = load_dataset("madao33/new-title-chinese")
print(datasets)
```

**输出：**
```python
DatasetDict({
    train: Dataset({
        features: ['title', 'content'],
        num_rows: 5850
    })
    validation: Dataset({
        features: ['title', 'content'],
        num_rows: 1679
    })
})
```

### 2.2 加载带子集的数据集

```python
# 某些数据集有多个子集/配置
boolq_dataset = load_dataset("super_glue", "boolq")
print(boolq_dataset)
```

**输出：**
```python
DatasetDict({
    train: Dataset({
        features: ['question', 'passage', 'idx', 'label'],
        num_rows: 9427
    })
    validation: Dataset({
        features: ['question', 'passage', 'idx', 'label'],
        num_rows: 3270
    })
    test: Dataset({
        features: ['question', 'passage', 'idx', 'label'],
        num_rows: 3245
    })
})
```

### 2.3 指定数据划分加载

```python
# 只加载训练集
dataset = load_dataset("madao33/new-title-chinese", split="train")
print(dataset)
# Dataset({features: ['title', 'content'], num_rows: 5850})

# 加载部分数据（索引切片）
dataset = load_dataset("madao33/new-title-chinese", split="train[10:100]")
print(dataset)
# Dataset({num_rows: 90})

# 按比例加载
dataset = load_dataset("madao33/new-title-chinese", split="train[:50%]")
print(dataset)
# Dataset({num_rows: 2925})

# 同时加载多个划分
datasets = load_dataset(
    "madao33/new-title-chinese",
    split=["train[:50%]", "train[50%:]"]
)
print(datasets)
# [Dataset(...), Dataset(...)]
```

### 2.4 数据访问方式

```python
from datasets import load_dataset

datasets = load_dataset("madao33/new-title-chinese")

# 方式 1：获取单条数据（字典格式）
example = datasets["train"][0]
print(example)
# {'title': '望海楼...', 'content': '近期...'}

# 方式 2：获取多条数据（列名作为 key）
examples = datasets["train"][:2]
print(examples)
# {
#     'title': ['望海楼...', '大力推进...'],
#     'content': ['近期...', '在推进...']
# }

# 方式 3：获取特定列
titles = datasets["train"]["title"][:5]
print(titles)
# ['望海楼...', '大力推进...', ...]

# 方式 4：查看列名
print(datasets["train"].column_names)
# ['title', 'content']

# 方式 5：查看特征类型
print(datasets["train"].features)
# {'title': Value(dtype='string'), 'content': Value(dtype='string')}
```

---

## 3. 加载本地数据集

### 3.1 加载 CSV 文件

```python
from datasets import load_dataset, Dataset

# 方式 1：使用 load_dataset 加载 CSV
dataset = load_dataset(
    "csv",
    data_files="./ChnSentiCorp_htl_all.csv",
    split="train"
)
print(dataset)
# Dataset({features: ['label', 'review'], num_rows: 7766})

# 方式 2：使用 Dataset.from_csv
dataset = Dataset.from_csv("./ChnSentiCorp_htl_all.csv")
```

### 3.2 加载多个文件

```python
# 加载多个 CSV 文件
dataset = load_dataset(
    "csv",
    data_files=[
        "./all_data/ChnSentiCorp_htl_all.csv",
        "./all_data/ChnSentiCorp_htl_all copy.csv"
    ],
    split="train"
)
print(dataset)
# Dataset({num_rows: 15532})  # 两个文件合并

# 使用通配符
dataset = load_dataset(
    "csv",
    data_files="./all_data/*.csv",
    split="train"
)
```

### 3.3 加载 JSON 文件

```python
# 加载 JSON 文件
dataset = load_dataset(
    "json",
    data_files="./cmrc2018_trial.json",
    field="data"  # 指定 JSON 中的字段
)
print(dataset)
# DatasetDict({train: Dataset({features: ['paragraphs', 'id', 'title'], ...})})
```

### 3.4 从 Pandas DataFrame 加载

```python
import pandas as pd
from datasets import Dataset

# 读取 CSV 到 DataFrame
data = pd.read_csv("./ChnSentiCorp_htl_all.csv")
print(data.head())

# 转换为 Dataset
dataset = Dataset.from_pandas(data)
print(dataset)
# Dataset({features: ['label', 'review'], num_rows: 7766})
```

### 3.5 从 List 加载

```python
from datasets import Dataset

# List 格式（必须是字典列表）
data = [
    {"text": "这是一条正面评论", "label": 1},
    {"text": "这是一条负面评论", "label": 0},
]
dataset = Dataset.from_list(data)
print(dataset)
# Dataset({features: ['text', 'label'], num_rows: 2})

# 错误示例：直接传字符串列表
# data = ["abc", "def"]  # 会报错！
```

---

## 4. 数据集操作

### 4.1 数据集划分

```python
from datasets import load_dataset

dataset = load_dataset("madao33/new-title-chinese")["train"]

# 划分训练集和测试集
split_dataset = dataset.train_test_split(test_size=0.1)
print(split_dataset)
# DatasetDict({
#     train: Dataset({num_rows: 5265}),
#     test: Dataset({num_rows: 585})
# })

# 分类数据集按标签分层抽样
boolq_dataset = load_dataset("super_glue", "boolq")["train"]
split_dataset = boolq_dataset.train_test_split(
    test_size=0.1,
    stratify_by_column="label"  # 按标签分层
)
```

### 4.2 数据选择（Select）

```python
# 选择指定索引的数据
dataset = load_dataset("madao33/new-title-chinese")["train"]
selected = dataset.select([0, 1, 2, 10, 20])
print(selected)
# Dataset({num_rows: 5})
```

### 4.3 数据过滤（Filter）

```python
dataset = load_dataset("madao33/new-title-chinese")["train"]

# 过滤标题包含"中国"的数据
filtered = dataset.filter(lambda example: "中国" in example["title"])
print(f"过滤后数量：{len(filtered)}")
print(filtered["title"][:5])
# ['聚焦两会，世界探寻中国成功秘诀',
#  '望海楼中国经济的信心来自哪里',
#  ...]

# 过滤长文本
filtered = dataset.filter(lambda example: len(example["content"]) > 500)
```

### 4.4 数据映射（Map）

```python
from datasets import load_dataset

dataset = load_dataset("madao33/new-title-chinese")

# 方式 1：添加前缀
def add_prefix(example):
    example["title"] = 'Prefix: ' + example["title"]
    return example

prefix_dataset = dataset.map(add_prefix)
print(prefix_dataset["train"]["title"][:3])
# ['Prefix: 望海楼...', 'Prefix: 大力推进...', ...]
```

**Map 处理流程图：**
```
┌─────────────────────────────────────────────────────┐
│  原始数据                                            │
│  [{'title': 'A', 'content': '...'},                 │
│   {'title': 'B', 'content': '...'}]                 │
└─────────────────────────────────────────────────────┘
                       ↓ map(add_prefix)
┌─────────────────────────────────────────────────────┐
│  处理后数据                                          │
│  [{'title': 'Prefix: A', 'content': '...'},         │
│   {'title': 'Prefix: B', 'content': '...'}]         │
└─────────────────────────────────────────────────────┘
```

---

## 5. 文本预处理（Preprocessing）

### 5.1 完整的预处理函数

```python
from transformers import AutoTokenizer
from datasets import load_dataset

# 加载 tokenizer
tokenizer = AutoTokenizer.from_pretrained("bert-base-chinese")

# 加载数据集
datasets = load_dataset("madao33/new-title-chinese")

def preprocess_function(example):
    """
    预处理函数：对内容进行编码，生成 label

    Args:
        example: 单条数据，如 {'title': '...', 'content': '...'}

    Returns:
        处理后的数据字典
    """
    # 对内容进行编码
    model_inputs = tokenizer(
        example["content"],
        max_length=512,
        truncation=True
    )

    # 对 title 进行编码（作为 label）
    labels = tokenizer(
        example["title"],
        max_length=32,
        truncation=True
    )

    # 添加 labels 字段
    model_inputs["labels"] = labels["input_ids"]

    return model_inputs

# 应用预处理
processed_datasets = datasets.map(preprocess_function)
print(processed_datasets)
```

**输出：**
```python
DatasetDict({
    train: Dataset({
        features: ['title', 'content', 'input_ids', 'token_type_ids',
                   'attention_mask', 'labels'],
        num_rows: 5850
    })
    validation: Dataset({
        features: [...],
        num_rows: 1679
    })
})
```

### 5.2 批量处理（Batched）

```python
# 单条处理（较慢）
processed = datasets.map(preprocess_function)

# 批量处理（更快）
processed = datasets.map(preprocess_function, batched=True)

# 多进程处理（最快）
processed = datasets.map(
    preprocess_function,
    batched=True,
    num_proc=4  # 使用 4 个进程
)
```

**性能对比：**
| 方式 | 处理时间 |
|------|----------|
| 单条处理 | ~60 秒 |
| 批量处理 | ~30 秒 |
| 多进程批量 | ~10 秒 |

### 5.3 移除原始列

```python
# 处理完成后移除不需要的列
processed_datasets = datasets.map(
    preprocess_function,
    batched=True,
    remove_columns=datasets["train"].column_names  # 移除原始列
)
print(processed_datasets["train"].column_names)
# ['input_ids', 'token_type_ids', 'attention_mask', 'labels']
```

### 5.4 分类任务预处理

```python
from transformers import AutoTokenizer
from datasets import load_dataset

tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 加载情感分析数据集
dataset = load_dataset(
    "csv",
    data_files="./ChnSentiCorp_htl_all.csv",
    split="train"
)

# 过滤空值
dataset = dataset.filter(lambda x: x["review"] is not None)

def process_function(examples):
    """情感分析预处理"""
    # 编码评论文本
    tokenized = tokenizer(
        examples["review"],
        max_length=128,
        truncation=True
    )
    # 添加标签
    tokenized["labels"] = examples["label"]
    return tokenized

tokenized_dataset = dataset.map(
    process_function,
    batched=True,
    remove_columns=dataset.column_names
)

print(tokenized_dataset[0])
# {'input_ids': [...], 'attention_mask': [...], 'labels': 1}
```

---

## 6. 数据保存与加载

### 6.1 保存处理后的数据集

```python
# 保存到磁盘
processed_datasets.save_to_disk("./processed_data")
```

### 6.2 加载已保存的数据集

```python
from datasets import load_from_disk

# 从磁盘加载
processed_datasets = load_from_disk("./processed_data")
print(processed_datasets)
```

### 6.3 保存的好处

| 优点 | 说明 |
|------|------|
| **快速加载** | 避免重复处理 |
| **共享数据** | 团队共享预处理结果 |
| **版本控制** | 保存不同版本的数据 |

---

## 7. DataCollator 与 DataLoader

### 7.1 为什么需要 DataCollator？

**问题**：不同长度的句子如何组成 batch？

**解决方案**：使用 DataCollator 动态填充

```python
from transformers import AutoTokenizer, DataCollatorWithPadding

tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# 创建 DataCollator
collator = DataCollatorWithPadding(tokenizer=tokenizer)

# 模拟 batch 数据
batch = [
    {"input_ids": [1, 2, 3], "labels": 0},
    {"input_ids": [1, 2, 3, 4, 5], "labels": 1},
]

# 动态填充
collated_batch = collator(batch)
print(collated_batch)
```

**输出：**
```python
{
    'input_ids': tensor([
        [1, 2, 3, 0, 0],      # 填充到最长
        [1, 2, 3, 4, 5]
    ]),
    'attention_mask': tensor([
        [1, 1, 1, 0, 0],      # 填充位置为 0
        [1, 1, 1, 1, 1]
    ]),
    'labels': tensor([0, 1])
}
```

### 7.2 常见 DataCollator

| DataCollator | 用途 |
|--------------|------|
| `DataCollatorWithPadding` | 动态填充（最常用） |
| `DataCollatorForTokenClassification` | Token 分类 |
| `DataCollatorForLanguageModeling` | 语言模型 |
| `DataCollatorForSeq2Seq` | Seq2Seq 任务 |

### 7.3 创建 DataLoader

```python
from torch.utils.data import DataLoader
from transformers import DataCollatorWithPadding

# 准备数据集
tokenized_dataset = tokenized_dataset.remove_columns(["input_ids", "attention_mask", "labels"])

trainset = tokenized_dataset["train"]
validset = tokenized_dataset["test"]

# 创建 DataCollator
collator = DataCollatorWithPadding(tokenizer=tokenizer)

# 创建 DataLoader
trainloader = DataLoader(
    trainset,
    batch_size=32,
    shuffle=True,
    collate_fn=collator
)

validloader = DataLoader(
    validset,
    batch_size=64,
    shuffle=False,
    collate_fn=collator
)

# 遍历 DataLoader
for batch in trainloader:
    print(batch["input_ids"].shape)  # [32, dynamic_length]
    break
```

---

## 8. 自定义数据集加载脚本

### 8.1 创建加载脚本

对于特殊格式的数据，可以创建自定义加载脚本：

```python
# load_script.py
import json
import datasets
from datasets import DownloadManager, DatasetInfo


class CMRC2018TRIAL(datasets.GeneratorBasedBuilder):

    def _info(self) -> DatasetInfo:
        """定义数据集的字段和类型"""
        return datasets.DatasetInfo(
            description="CMRC2018 trial",
            features=datasets.Features({
                "id": datasets.Value("string"),
                "context": datasets.Value("string"),
                "question": datasets.Value("string"),
                "answers": datasets.features.Sequence({
                    "text": datasets.Value("string"),
                    "answer_start": datasets.Value("int32"),
                })
            })
        )

    def _split_generators(self, dl_manager: DownloadManager):
        """定义数据集划分"""
        return [
            datasets.SplitGenerator(
                name=datasets.Split.TRAIN,
                gen_kwargs={"filepath": "./cmrc2018_trial.json"}
            )
        ]

    def _generate_examples(self, filepath):
        """生成具体样本"""
        with open(filepath, encoding="utf-8") as f:
            data = json.load(f)
            for example in data["data"]:
                for paragraph in example["paragraphs"]:
                    context = paragraph["context"].strip()
                    for qa in paragraph["qas"]:
                        question = qa["question"].strip()
                        id_ = qa["id"]

                        answer_starts = [a["answer_start"] for a in qa["answers"]]
                        answers = [a["text"].strip() for a in qa["answers"]]

                        yield id_, {
                            "context": context,
                            "question": question,
                            "id": id_,
                            "answers": {
                                "answer_start": answer_starts,
                                "text": answers,
                            },
                        }
```

### 8.2 使用自定义脚本

```python
from datasets import load_dataset

# 使用自定义脚本加载
dataset = load_dataset("./load_script.py", split="train")
print(dataset[0])
```

---

## 9. 实战：完整数据管道

### 9.1 问题场景

构建酒店评论情感分析的数据管道。

### 9.2 完整解决方案

```python
from datasets import load_dataset
from transformers import AutoTokenizer, DataCollatorWithPadding
from torch.utils.data import DataLoader

# ============ Step 1: 加载数据 ============
dataset = load_dataset(
    "csv",
    data_files="./ChnSentiCorp_htl_all.csv",
    split="train"
)

# ============ Step 2: 数据清洗 ============
# 过滤空值
dataset = dataset.filter(lambda x: x["review"] is not None)

# 过滤过短评论
dataset = dataset.filter(lambda x: len(x["review"]) > 10)

# ============ Step 3: 划分数据集 ============
datasets = dataset.train_test_split(
    test_size=0.1,
    seed=42,
    stratify_by_column="label"  # 分层抽样
)

# ============ Step 4: 创建 Tokenizer ============
tokenizer = AutoTokenizer.from_pretrained("hfl/rbt3")

# ============ Step 5: 预处理 ============
def process_function(examples):
    tokenized = tokenizer(
        examples["review"],
        max_length=128,
        truncation=True,
        padding=False  # 使用 DataCollator 动态填充
    )
    tokenized["labels"] = examples["label"]
    return tokenized

tokenized_datasets = datasets.map(
    process_function,
    batched=True,
    remove_columns=datasets["train"].column_names,
    num_proc=4
)

# ============ Step 6: 创建 DataCollator ============
data_collator = DataCollatorWithPadding(tokenizer=tokenizer)

# ============ Step 7: 创建 DataLoader ============
trainloader = DataLoader(
    tokenized_datasets["train"],
    batch_size=32,
    shuffle=True,
    collate_fn=data_collator
)

validloader = DataLoader(
    tokenized_datasets["test"],
    batch_size=64,
    shuffle=False,
    collate_fn=data_collator
)

# ============ Step 8: 验证数据管道 ============
batch = next(iter(trainloader))
print(f"input_ids shape: {batch['input_ids'].shape}")
print(f"attention_mask shape: {batch['attention_mask'].shape}")
print(f"labels shape: {batch['labels'].shape}")
```

**输出：**
```
input_ids shape: torch.Size([32, 89])  # batch_size=32, dynamic_length=89
attention_mask shape: torch.Size([32, 89])
labels shape: torch.Size([32])
```

---

## 10. 性能优化技巧

### 10.1 缓存处理结果

```python
# 第一次处理（慢）
processed = dataset.map(process_function, batched=True)

# 保存
processed.save_to_disk("./cached_data")

# 后续使用（快）
from datasets import load_from_disk
processed = load_from_disk("./cached_data")
```

### 10.2 流式加载大文件

```python
# 对于超大文件，使用流式加载
dataset = load_dataset(
    "csv",
    data_files="huge_file.csv",
    streaming=True  # 流式模式
)

# 流式模式下数据是迭代器
for example in dataset:
    process(example)
```

### 10.3 使用缓存目录

```python
# 设置缓存目录
from datasets import set_caching_enabled

set_caching_enabled(True)  # 启用缓存

# 或者指定缓存目录
from datasets import config
config.HF_DATASETS_CACHE = "./my_cache"
```

---

## 11. 常见问题与解决方案

### 11.1 内存不足

```python
# 方案 1：使用流式加载
dataset = load_dataset("csv", data_files="large.csv", streaming=True)

# 方案 2：只加载需要的列
dataset = load_dataset(
    "csv",
    data_files="large.csv",
    split="train",
    columns=["review", "label"]  # 只加载需要的列
)

# 方案 3：分批处理
for i in range(0, len(dataset), 1000):
    batch = dataset.select(range(i, min(i+1000, len(dataset))))
    process(batch)
```

### 11.2 数据不平衡

```python
# 方案 1：分层抽样
datasets = dataset.train_test_split(
    test_size=0.1,
    stratify_by_column="label"
)

# 方案 2：过采样
from datasets import concatenate_datasets

minority = dataset.filter(lambda x: x["label"] == 1)
majority = dataset.filter(lambda x: x["label"] == 0)

# 复制少数类
oversampled = concatenate_datasets([majority, minority, minority])
```

### 11.3 处理中文乱码

```python
# 指定编码
dataset = load_dataset(
    "csv",
    data_files="./data.csv",
    encoding="utf-8"  # 指定编码
)
```

---

## 12. 本节小结

### 12.1 Datasets 核心 API 速查

| 方法 | 说明 | 示例 |
|------|------|------|
| `load_dataset` | 加载数据集 | `load_dataset("csv", ...)` |
| `train_test_split` | 划分数据集 | `dataset.train_test_split(0.1)` |
| `map` | 数据转换 | `dataset.map(func)` |
| `filter` | 数据过滤 | `dataset.filter(func)` |
| `select` | 选择数据 | `dataset.select([0,1,2])` |
| `save_to_disk` | 保存数据 | `dataset.save_to_disk("./path")` |
| `load_from_disk` | 加载数据 | `load_from_disk("./path")` |

### 12.2 数据处理流程

```
加载数据 → 清洗数据 → 划分数据集 → 预处理 → 创建 DataLoader
    ↓           ↓           ↓           ↓           ↓
load_dataset  filter    train_test  map/tokenize  DataLoader
                                  split
```

### 12.3 下一步学习

学习 [Evaluate + Trainer](05-训练实战-Evaluate+Trainer.md) 进行模型训练和评估。

---

## 附录：Dataset vs PyTorch Dataset

### A.1 Datasets 库的 Dataset

```python
from datasets import load_dataset

dataset = load_dataset("csv", data_files="data.csv")

# 特点：
# 1. 内存映射，不占用大量内存
# 2. 支持 map/filter 等操作
# 3. 自动缓存
# 4. 支持流式加载
```

### A.2 PyTorch 的 Dataset

```python
from torch.utils.data import Dataset

class MyDataset(Dataset):
    def __init__(self):
        self.data = [...]

    def __getitem__(self, idx):
        return self.data[idx]

    def __len__(self):
        return len(self.data)

# 特点：
# 1. 需要自己实现数据加载
# 2. 所有数据加载到内存
# 3. 灵活性高
```

### A.3 对比总结

| 特性 | Datasets Dataset | PyTorch Dataset |
|------|------------------|-----------------|
| 内存效率 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| 处理功能 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| 灵活性 | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 易用性 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |

**推荐**：优先使用 Datasets 库，特殊需求时自定义 PyTorch Dataset。

---

> **提示**：下一节将学习如何使用 Evaluate 库进行模型评估，以及使用 Trainer 进行模型训练。
