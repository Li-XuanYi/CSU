from statsmodels.stats.outliers_influence import variance_inflation_factor
import pandas as pd
from scipy.stats import pearsonr
from sklearn.linear_model import LinearRegression
from sklearn.metrics import r2_score, mean_squared_error
import numpy as np


# 1. 读取数据
df = pd.read_excel('2010-2016单胎正常样本（脱敏）.xlsx', sheet_name='原表')

# 2. 提取候选特征列
cols = ['年龄', 'em', 'Ⅲ线', '助孕方式', '1ET天', '孕囊', '胚芽', '卵黄囊', '胎心']
df = df[cols].copy()

# 3. 处理缺失值
#    这里简单地删除含有任何缺失的行；如需插补，可替换为 df.fillna(...)
df = df.dropna(subset=cols)

df['助孕方式_binary'] = df['助孕方式'].apply(lambda x: 1 if x == '冻胚' else 0)

df['孕囊'] = df['孕囊'].astype(str).str.replace('x', 'X', case=False)
df[['X', 'Y']] = df['孕囊'].str.split('X', expand=True)

# 2. 数值转换（处理非数字字符）
df['X'] = pd.to_numeric(df['X'], errors='coerce')
df['Y'] = pd.to_numeric(df['Y'], errors='coerce')
df['xyAverage'] = (df['X'] + df['Y'])/2

# 3. 计算面积并过滤异常值
df['面积'] = df['X'] * df['Y']

def map_iii_line(value):
    if pd.isna(value):
        return 'Ⅲ线_欠清'
    value = str(value).strip()
    if value in {'微量宫腔积液', '少量宫腔积液', '宫腔积液', '积液', '微量积液', '宫腔积水', '大量宫腔积液', '少量积液'}:
        return 'Ⅲ线_有积液'
    elif value in {'1'}:
        return 'Ⅲ线_Ⅰ'
    elif value in {'2稍欠清', '2                2', '2', '2均'}:
        return 'Ⅲ线_Ⅱ'
    elif value in {'3                3', '3均', '3均匀', '33', '3均，宫腔积液', '3'}:
        return 'Ⅲ线_Ⅲ'
    elif value in {'2-3'}:
        return 'Ⅲ线_Ⅱ-Ⅲ'
    else:
        return 'Ⅲ线_欠清'

# 应用这个函数替换原列
df['Ⅲ线'] = df['Ⅲ线'].apply(map_iii_line)

df = df[~((df['em'] == '无') & (df['Ⅲ线'] == 'Ⅲ线_欠清'))]

# 6类Ⅲ线特征的独热编码
mapping = {
    'Ⅲ线_Ⅰ':     [1, 0, 0, 0, 0, 0],
    'Ⅲ线_Ⅱ':     [0, 1, 0, 0, 0, 0],
    'Ⅲ线_Ⅱ-Ⅲ':  [0, 0, 1, 0, 0, 0],
    'Ⅲ线_Ⅲ':     [0, 0, 0, 1, 0, 0],
    'Ⅲ线_有积液': [0, 0, 0, 0, 1, 0],
    'Ⅲ线_欠清':   [0, 0, 0, 0, 0, 1],
}

# 检查是否存在未定义值
unknown_values = set(df['Ⅲ线']) - set(mapping.keys())
if unknown_values:
    print("未定义的Ⅲ线值：", unknown_values)
    raise ValueError("请检查这些值的分类！")

# 构造新的6个列名
onehot_cols = ['Ⅲ线_Ⅰ', 'Ⅲ线_Ⅱ', 'Ⅲ线_Ⅱ-Ⅲ', 'Ⅲ线_Ⅲ', 'Ⅲ线_有积液', 'Ⅲ线_欠清']

# 应用映射生成6列的独热编码
onehots = df['Ⅲ线'].apply(lambda x: pd.Series(mapping[x], index=onehot_cols))

# 合并到原数据中
df = pd.concat([df, onehots], axis=1)

# 6. 丢弃原始“Ⅲ线”和“助孕方式”列（可选）
df = df.drop(columns=['Ⅲ线', '助孕方式'])

# 将胎心列转为数值，非数字转为NaN
df['胎心'] = pd.to_numeric(df['胎心'], errors='coerce')

# 复制所有胎心为数字的行（非NaN的行）
numeric_rows = df[df['胎心'].notna()].copy()  # .copy()避免链式赋值警告
# 选择所有数值类型的列进行相关性计算
numeric_df = df.select_dtypes(include=['number'])

numeric_df_clean = numeric_df.replace([np.inf, -np.inf], np.nan).dropna()

significant_features = []  # 创建空列表存储显著相关特征
for col in numeric_df_clean.columns.difference(['胎心']):  # 遍历除"胎心"外的所有列
    r, p = pearsonr(numeric_df_clean['胎心'], numeric_df_clean[col])     # 计算Pearson相关系数和p值
    if p < 0.05 and abs(r) > 0.5:           # 双重筛选条件
        significant_features.append((col, r, p))  # 记录通过筛选的特征
print(significant_features)
valid_mask = df[['X', 'Y', '胎心','xyAverage']].notna().all(axis=1)  # 每个元素根据是否是nan赋值 ture、false，  .all(axis=1)每一行做与操作

model_df = df[valid_mask].copy()

# 定义特征矩阵X和目标变量y
X = model_df[['xyAverage']]  # 使用三个几何特征
y = model_df['胎心']

# 数据标准化（提升模型稳定性）
X_normalized = (X - X.mean()) / X.std()

'''
使用X、Y拟合
R² Score: 0.726
MSE: 203.31

使用X、Y的平均值拟合
R² Score: 0.727
MSE: 202.62
'''

# 划分训练测试集（80%训练，20%测试）
np.random.seed(42)
train_mask = np.random.rand(len(X)) < 0.8
X_train, X_test = X_normalized[train_mask], X_normalized[~train_mask]
y_train, y_test = y[train_mask], y[~train_mask]
'''
vif_data = pd.DataFrame()
vif_data["特征"] = X_train.columns

vif_data["VIF"] = [variance_inflation_factor(X_train.values, i)
                   for i in range(X_train.shape[1])]

print("\n方差膨胀因子 (VIF):")
print(vif_data)
'''

# 建立并训练线性回归模型
model = LinearRegression()
model.fit(X_train, y_train)

# 模型预测
y_pred = model.predict(X_test)

# == 模型评估 ==
# 计算评估指标
r2 = r2_score(y_test, y_pred)
mse = mean_squared_error(y_test, y_pred)
metrics = {
    "R²": r2,
    "均方误差": mse,
    "特征系数": dict(zip(['X', 'Y', '面积'], model.coef_)),
    "截距项": model.intercept_
}


# 打印模型参数
print("模型评估结果：")
print(f"R² Score: {r2:.3f}")
print(f"MSE: {mse:.2f}")
print("回归系数：")
for feat, coef in metrics['特征系数'].items():
    print(f"  {feat}: {coef:.3f}")
print(f"截距: {metrics['截距项']:.2f}")

missing_heart_mask = df['胎心'].isna() & df[['xyAverage']].notna().all(axis=1)
X_missing = df.loc[missing_heart_mask, ['xyAverage']]

if not X_missing.empty:
    # 使用训练集的均值和标准差标准化
    X_missing_normalized = (X_missing - X.mean()) / X.std()

    # 模型预测
    y_pred_missing = model.predict(X_missing_normalized)

    # 填补缺失值
    df.loc[missing_heart_mask, '胎心'] = y_pred_missing

df['结果']=1


df.to_excel('2010-2016单胎正常样本（脱敏）输出.xlsx', index=False)
