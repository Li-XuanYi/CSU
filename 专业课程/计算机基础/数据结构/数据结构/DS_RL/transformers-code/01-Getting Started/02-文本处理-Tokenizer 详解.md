# Transformer 文本处理：Tokenizer 详解

> 本教程深入讲解 Tokenizer 的工作原理，理解文本如何被转换为模型可理解的数字表示。

---

## 📖 学习目标

完成本节后，你将能够：
1. 理解 Tokenizer 的作用和工作原理
2. 掌握 Tokenizer 的完整处理流程（8 个步骤）
3. 理解特殊 token 的作用（[CLS], [SEP], [PAD] 等）
4. 掌握填充、截断等批处理技巧
5. 了解 Fast Tokenizer 与 Slow Tokenizer 的区别

---

## 1. 为什么需要 Tokenizer？

### 1.1 核心问题

Transformer 模型只能处理数字，无法直接理解文本：

```
文本输入："弱小的我也有大梦想！"
          ↓  Tokenizer
数字输入：[101, 2769, 4688, 3297, 678, 2398, 511]
          ↓  Model
模型输出：[0.1, 0.9]  # 情感分类结果
```

### 1.2 Tokenizer 的三大功能

| 功能 | 说明 | 示例 |
|------|------|------|
| **分词** | 将文本切分为 token | "我爱你" → ["我", "爱", "你"] |
| **索引转换** | 将 token 转为 ID | ["我", "爱"] → [1234, 5678] |
| **添加特殊 token** | 添加模型需要的标记 | [CLS] 句子 [SEP] |

---

## 2. Tokenizer 基本使用流程

### 2.1 Step 1：加载与保存

```python
from transformers import AutoTokenizer

# 从 HuggingFace 加载预训练分词器
tokenizer = AutoTokenizer.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)

print(tokenizer)
```

**输出：**
```
RobertaTokenizerFast(name_or_path='uer/roberta-base-finetuned-dianping-chinese',
                     vocab_size=21128, model_max_len=512, ...)
```

**保存到本地：**
```python
# 保存 tokenizer 到本地
tokenizer.save_pretrained("./roberta_tokenizer")

# 从本地加载
tokenizer = AutoTokenizer.from_pretrained("./roberta_tokenizer/")
```

### 2.2 Step 2：句子分词

```python
sen = "弱小的我也有大梦想！"

# 将句子切分为 token
tokens = tokenizer.tokenize(sen)
print(tokens)
```

**输出：**
```python
['弱', '小', '的', '我', '也', '有', '大', '梦', '想', '！']
```

**处理过程图解：**
```
┌─────────────────────────────────────────┐
│   原始文本：弱小的我也有大梦想！             │
└─────────────────────────────────────────┘
                    ↓ tokenize()
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│ 弱  │ 小  │ 的  │ 我  │ 也  │ 有  │ 大  │ 梦  │ 想  │ ！  │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
```

### 2.3 Step 3：查看词典

```python
# 查看完整词典（部分）
print(f"词典大小：{tokenizer.vocab_size}")

# 查看特定 token 的 ID
print(f"'弱' 的 ID: {tokenizer.vocab['弱']}")

# ID 转 token
print(f"ID 1234 对应的 token: {tokenizer.convert_ids_to_tokens([1234])}")
```

**BERT/RoBERTa 词典说明：**
| 词表大小 | 说明 |
|----------|------|
| BERT-Base | 21,128 (中文) |
| BERT-Large | 30,522 (英文) |
| GPT-2 | 50,257 |

### 2.4 Step 4：索引转换

```python
# 将 token 序列转换为 ID 序列
ids = tokenizer.convert_tokens_to_ids(tokens)
print(f"Token 转 ID: {ids}")

# 将 ID 序列转换回 token 序列
tokens_back = tokenizer.convert_ids_to_tokens(ids)
print(f"ID 转 Token: {tokens_back}")

# 将 token 序列转换为字符串
str_sen = tokenizer.convert_tokens_to_string(tokens_back)
print(f"还原字符串：{str_sen}")
```

**输出：**
```python
Token 转 ID: [6810, 7399, 4638, 1962, 3221, 1962, 6821, 7828, 8561, 3290]
ID 转 Token: ['弱', '小', '的', '我', '也', '有', '大', '梦', '想', '！']
还原字符串：弱小的我也有大梦想！
```

### 2.5 Step 5：便捷转换方式

```python
sen = "弱小的我也有大梦想！"

# encode：字符串 → ID 序列（包含特殊 token）
ids = tokenizer.encode(sen, add_special_tokens=True)
print(f"encode 结果：{ids}")
# 输出：[0, 6810, 7399, 4638, 1962, 3221, 1962, 6821, 7828, 8561, 3290, 2]

# decode：ID 序列 → 字符串
str_sen = tokenizer.decode(ids, skip_special_tokens=False)
print(f"decode 结果：{str_sen}")
```

**特殊 Token 说明：**
```
[0, 6810, 7399, ..., 3290, 2]
 │                     │
 └─ <s>               └─ </s>
    (句首)                (句尾)

对于 BERT 模型：
[101, ..., 102]
 │          │
 └─ [CLS]   └─ [SEP]
```

---

## 3. 批处理关键技术

### 3.1 Step 6：填充（Padding）

**问题**：不同长度的句子如何处理？

**解决方案**：填充到相同长度

```python
sen = "弱小的我也有大梦想！"

# 填充到指定长度
ids = tokenizer.encode(
    sen,
    padding="max_length",
    max_length=15
)
print(f"填充后 ID: {ids}")
# 输出：[0, 6810, 7399, ..., 3290, 2, 1, 1, 1, 1, 1]
#                                   ↑ 填充 token
```

**填充示意图：**
```
句子 1: [CLS] 我 爱 你 [SEP] [PAD] [PAD]
句子 2: [CLS] 今 天 天 气 好 [SEP] [PAD]
句子 3: [CLS] 他 是 一 名 医 生 [SEP]
```

### 3.2 Step 7：截断（Truncation）

**问题**：句子太长超过模型限制怎么办？

**解决方案**：截断超长部分

```python
# 截断到指定长度
ids = tokenizer.encode(
    sen,
    max_length=5,
    truncation=True
)
print(f"截断后 ID: {ids}")
# 输出：[0, 6810, 7399, 4638, 2]
```

**截断策略：**
| 策略 | 说明 |
|------|------|
| `longest_first` | 优先截断长句子（默认） |
| `only_first` | 只截断第一个句子 |
| `only_second` | 只截断第二个句子 |

### 3.3 Step 8：注意力掩码（Attention Mask）

**问题**：如何告诉模型哪些是填充的无效 token？

**解决方案**：使用 attention_mask

```python
ids = tokenizer.encode(
    sen,
    padding="max_length",
    max_length=15
)

# 创建 attention_mask
# 1 表示真实 token，0 表示填充 token
attention_mask = [1 if idx != 1 else 0 for idx in ids]  # 1 是 padding 的 ID

print(f"Input IDs: {ids}")
print(f"Attention Mask: {attention_mask}")
```

**输出：**
```python
Input IDs:        [0, 6810, 7399, ..., 2, 1, 1, 1]
Attention Mask:   [1, 1,    1,    ..., 1, 0, 0, 0]
                                  ↑  真实 token
                                      ↓  填充 token
```

### 3.4 一步到位的调用方式

```python
# 最简洁的调用方式（推荐）
inputs = tokenizer(
    sen,
    padding="max_length",
    max_length=15,
    truncation=True,
    return_tensors="pt"  # 返回 PyTorch 张量
)

print(inputs)
```

**输出：**
```python
{
    'input_ids': tensor([[0, 6810, 7399, ..., 2, 1, 1, 1]]),
    'attention_mask': tensor([[1, 1, 1, ..., 1, 0, 0, 0]])
}
```

---

## 4. 处理 Batch 数据

### 4.1 批量编码

```python
sens = [
    "弱小的我也有大梦想",
    "有梦想谁都了不起",
    "追逐梦想的心，比梦想本身，更可贵"
]

# 批量处理（比循环快得多）
results = tokenizer(sens, padding=True, truncation=True, max_length=50)

print(f"batch size: {len(results['input_ids'])}")
print(f"每个句子的长度：{[len(ids) for ids in results['input_ids']]}")
```

**输出：**
```python
batch size: 3
每个句子的长度：[11, 10, 18]
```

### 4.2 性能对比

```python
import time

sen = "弱小的我也有大梦想"

# 方式 1：循环处理
start = time.time()
for i in range(1000):
    tokenizer(sen)
loop_time = time.time() - start
print(f"循环处理时间：{loop_time:.4f}秒")

# 方式 2：批量处理
start = time.time()
results = tokenizer([sen] * 1000)
batch_time = time.time() - start
print(f"批量处理时间：{batch_time:.4f}秒")

print(f"批量处理快 {loop_time / batch_time:.2f} 倍")
```

**典型结果：**
```
循环处理时间：0.5234 秒
批量处理时间：0.0821 秒
批量处理快 6.37 倍
```

---

## 5. Fast vs Slow Tokenizer

### 5.1 加载两种 Tokenizer

```python
# Fast Tokenizer（基于 Rust，默认）
fast_tokenizer = AutoTokenizer.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese"
)
print(f"Fast: {type(fast_tokenizer)}")

# Slow Tokenizer（纯 Python）
slow_tokenizer = AutoTokenizer.from_pretrained(
    "uer/roberta-base-finetuned-dianping-chinese",
    use_fast=False
)
print(f"Slow: {type(slow_tokenizer)}")
```

**输出：**
```
Fast: <class 'transformers.models.roberta.tokenization_roberta_fast.RobertaTokenizerFast'>
Slow: <class 'transformers.models.roberta.tokenization_roberta.RobertaTokenizer'>
```

### 5.2 性能对比

```python
sen = "弱小的我也有大 Dreaming!"

# Fast Tokenizer 性能
import time
start = time.time()
for i in range(10000):
    fast_tokenizer(sen)
fast_time = time.time() - start

# Slow Tokenizer 性能
start = time.time()
for i in range(10000):
    slow_tokenizer(sen)
slow_time = time.time() - start

print(f"Fast: {fast_time:.4f}秒")
print(f"Slow: {slow_time:.4f}秒")
print(f"Fast 快 {slow_time / fast_time:.2f} 倍")
```

**典型结果：**
```
Fast: 0.3521 秒
Slow: 1.8934 秒
Fast 快 5.38 倍
```

### 5.3 功能差异

| 功能 | Fast Tokenizer | Slow Tokenizer |
|------|----------------|----------------|
| 性能 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| offset_mapping | ✅ 支持 | ❌ 不支持 |
| 词边界信息 | ✅ 支持 | ❌ 部分支持 |
| 自定义修改 | ❌ 较难 | ✅ 容易 |

### 5.4 Offset Mapping（词边界映射）

```python
sen = "弱小的我也有大梦想！"

# Fast Tokenizer 可以提供 offset_mapping
inputs = fast_tokenizer(sen, return_offsets_mapping=True)

print(f"tokens: {inputs.tokens()}")
print(f"offsets: {inputs.offset_mapping}")
```

**输出：**
```python
tokens: ['<s>', '弱', '小', '的', '我', '也', '有', '大', '梦', '想', '!', '</s>']
offsets: [(0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6),
          (6, 7), (7, 8), (8, 9), (9, 10), (0, 0)]
```

**应用场景**：命名实体识别（NER）中定位实体位置

---

## 6. 特殊 Token 详解

### 6.1 BERT 系列特殊 Token

| Token | ID | 名称 | 作用 |
|-------|-----|------|------|
| `[CLS]` | 101 | Classification | 句首标记，用于分类任务 |
| `[SEP]` | 102 | Separator | 句子分隔符 |
| `[PAD]` | 0 | Padding | 填充标记 |
| `[MASK]` | 103 | Mask | 掩码标记，用于 MLM 任务 |
| `[UNK]` | 100 | Unknown | 未知词标记 |

### 6.2 RoBERTa 特殊 Token

| Token | ID | 名称 | 作用 |
|-------|-----|------|------|
| `<s>` | 0 | Start | 句首标记（同 [CLS]） |
| `</s>` | 2 | End | 句尾标记（同 [SEP]） |
| `<pad>` | 1 | Padding | 填充标记 |
| `<unk>` | 3 | Unknown | 未知词标记 |
| `<mask>` | 50264 | Mask | 掩码标记 |

### 6.3 查看特殊 Token

```python
tokenizer = AutoTokenizer.from_pretrained("uer/roberta-base-finetuned-dianping-chinese")

print(f"pad_token: {tokenizer.pad_token} (ID: {tokenizer.pad_token_id})")
print(f"cls_token: {tokenizer.cls_token} (ID: {tokenizer.cls_token_id})")
print(f"sep_token: {tokenizer.sep_token} (ID: {tokenizer.sep_token_id})")
print(f"unk_token: {tokenizer.unk_token} (ID: {tokenizer.unk_token_id})")
print(f"mask_token: {tokenizer.mask_token} (ID: {tokenizer.mask_token_id})")
```

---

## 7. 双句子输入处理

### 7.1 句子对编码

```python
sentence_a = "中国的首都是哪里？"
sentence_b = "中国的首都是北京。"

# 编码句子对
inputs = tokenizer(
    sentence_a,
    sentence_b,
    max_length=50,
    truncation=True
)

print(inputs)
```

**输出：**
```python
{
    'input_ids': [0, ..., 2, ..., 2],
                    ↑    ↑    ↑
                  <s>  </s> </s>
                  A 句   B 句结束
}
```

### 7.2 Token Type IDs

```python
inputs = tokenizer(
    sentence_a,
    sentence_b,
    return_tensors="pt"
)

# 对于 BERT 等模型，会有 token_type_ids
# 用于区分两个句子
if "token_type_ids" in inputs:
    print(f"Token Type IDs: {inputs['token_type_ids']}")
```

**说明：**
```
input_ids:     [CLS] 句子 A [SEP] 句子 B [SEP]
token_type_ids:  0     0...0   0     1...1   1
                 └─ 句子 A ─┘ └─ 句子 B ──┘
```

---

## 8. 实战：命名实体识别（NER）预处理

### 8.1 问题场景

给定句子和实体位置，准备 NER 模型的输入。

### 8.2 解决方案

```python
from transformers import AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("bert-base-chinese")

# 原始数据
sentence = "张三在北京大学工作。"
entities = [
    {"entity": "PERSON", "start": 0, "end": 2},   # 张三
    {"entity": "ORG", "start": 2, "end": 6},      # 北京大学
]

# 获取 offset_mapping
inputs = tokenizer(
    sentence,
    return_offsets_mapping=True,
    return_tensors="pt"
)

# 创建标签
labels = [-100] * len(inputs["input_ids"][0])  # -100 表示忽略

# 根据 offset_mapping 标注实体
for entity in entities:
    for i, (offset_start, offset_end) in enumerate(inputs["offset_mapping"][0]):
        if offset_start >= entity["start"] and offset_end <= entity["end"]:
            labels[i] = 1  # 实体内部
        elif offset_start < entity["end"] and offset_end > entity["end"]:
            labels[i] = 2  # 实体结尾

print(f"输入：{sentence}")
print(f"Labels: {labels}")
```

---

## 9. 常见问题与解决方案

### 9.1 问题：特殊字符处理

```python
# 某些特殊字符可能被拆分
sen = "Hello 👋 World"
tokens = tokenizer.tokenize(sen)
print(tokens)  # 可能包含<unk>或特殊拆分

# 解决方案：预处理文本，移除或替换特殊字符
import re
sen = re.sub(r'[^\w\s，。！？；：、]', '', sen)
```

### 9.2 问题：长文本处理

```python
# 超过 max_length 的文本会被截断
long_text = "这是一段非常非常长的文本..." * 1000

inputs = tokenizer(
    long_text,
    max_length=512,
    truncation=True,
    return_overflowing_tokens=True  # 保留被截断的信息
)

print(f"overflowing_tokens: {len(inputs.get('overflowing_tokens', []))}")
```

### 9.3 问题：自定义词表

```python
# 添加新 token
tokenizer.add_tokens(["新词 1", "新词 2"])

# 调整模型词表大小
model.resize_token_embeddings(len(tokenizer))
```

---

## 10. 本节小结

### 10.1 Tokenizer 核心流程

```
┌──────────────┐
│  原始文本     │
└──────┬───────┘
       │ tokenize()
       ↓
┌──────────────┐
│  Token 列表   │
└──────┬───────┘
       │ convert_tokens_to_ids()
       ↓
┌──────────────┐
│  ID 列表       │
└──────┬───────┘
       │ add_special_tokens()
       ↓
┌──────────────┐
│  最终输入     │ [CLS] ... [SEP]
└──────────────┘
```

### 10.2 关键参数速查

| 参数 | 说明 | 常用值 |
|------|------|--------|
| `padding` | 填充策略 | `True`, `"max_length"`, `False` |
| `truncation` | 截断策略 | `True`, `False` |
| `max_length` | 最大长度 | `128`, `256`, `512` |
| `return_tensors` | 返回类型 | `"pt"`, `"tf"`, `"np"` |
| `return_attention_mask` | 返回 attention_mask | `True`, `False` |
| `return_offsets_mapping` | 返回偏移映射 | `True`, `False` |

### 10.3 下一步学习

1. 深入学习 [Model](03-模型架构 -Model 详解.md) 的加载和调用
2. 学习 [Datasets](04-数据处理-Datasets 详解.md) 进行大规模数据处理

---

## 附录：Tokenizer 方法速查表

```python
# 加载
AutoTokenizer.from_pretrained(model_name)

# 分词
tokenizer.tokenize(text)

# 编码
tokenizer.encode(text, add_special_tokens=True)
tokenizer(text, padding=True, truncation=True)

# 解码
tokenizer.decode(ids, skip_special_tokens=True)

# 转换
tokenizer.convert_tokens_to_ids(tokens)
tokenizer.convert_ids_to_tokens(ids)
tokenizer.convert_tokens_to_string(tokens)

# 特殊方法
tokenizer.add_tokens(new_tokens)
tokenizer.add_special_tokens(special_tokens_dict)

# 保存
tokenizer.save_pretrained(save_directory)
```

---

> **提示**：下一节将深入讲解 Model 的内部结构，理解如何将 Tokenizer 的输出转换为最终的预测结果。
