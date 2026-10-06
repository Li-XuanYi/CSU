from statsmodels.stats.outliers_influence import variance_inflation_factor
import pandas as pd
from scipy.stats import pearsonr
from sklearn.linear_model import LinearRegression
from sklearn.metrics import r2_score, mean_squared_error
import numpy as np


# 1. 读取数据
df = pd.read_excel('2010-2016早流(异常)样本(脱敏).xlsx', sheet_name='总表')

# 2. 提取候选特征列
cols = ['年龄', 'em', 'Ⅲ线', '助孕方式', '1ET天', '孕囊', '胚芽', '卵黄囊', '胎心']
df = df[cols].copy()

# 3. 处理缺失值
#    这里简单地删除含有任何缺失的行；如需插补，可替换为 df.fillna(...)
df = df.dropna(subset=cols)

df['1ET天'] = (
    df['1ET天']
    .astype(str)
    .str.replace('[^0-9]', '', regex=True)  # 移除非数字字符（包括"囊胚"）
    .pipe(pd.to_numeric, errors='coerce')    # 转换为数值型
)

df['胚芽']=df['胚芽'].pipe(pd.to_numeric, errors='coerce')
df['卵黄囊']=df['卵黄囊'].pipe(pd.to_numeric, errors='coerce')

df['助孕方式_binary'] = df['助孕方式'].apply(lambda x: 1 if x == '冻胚' else 0)

df['孕囊'] = (
    df['孕囊'].astype(str)
    .apply(lambda x: np.nan if x.strip() == '无' else x)  # "无"设为空值
    .str.replace('暗区', '', case=False)  # 去除所有"暗区"字样（不区分大小写）
    .str.replace('[^0-9Xx.]', '', regex=True)  # 移除非数字、X和小数点的字符
    .str.replace('x', 'X', case=False)  # 统一为X分隔符
    .str.replace(r'X+', 'X', regex=True)  # 合并多个连续的X
)

# 2. 安全分割处理
split_data = (
    df['孕囊'].str.split('X', expand=True, n=1)  # 最多分割1次得到两列
    .replace({'': np.nan, None: np.nan})         # 空字符串转NaN
    .iloc[:, :2]                                 # 确保只取前两列
    .rename(columns={0: 'X', 1: 'Y'})            # 重命名列
)

# 3. 合并回原数据
df = pd.concat([df, split_data], axis=1)

# 继续后续的数值转换和过滤（原代码）
df['X'] = pd.to_numeric(df['X'], errors='coerce')
df['Y'] = pd.to_numeric(df['Y'], errors='coerce')
df['xyAverage'] = (df['X'] + df['Y'])/2.0

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


def stratified_imputation(df):
    """基于ET天±5天且相同妊娠结局的分层填补"""
    missing_mask = df['胎心'].isna()
    print(f"需填补缺失记录数: {missing_mask.sum()}")

    for idx in df[missing_mask].index:
        # 获取当前样本特征
        et_day = df.loc[idx, '1ET天']

        # 定义候选数据条件
        candidates = df[
            (df['1ET天'].between(et_day - 5, et_day + 5)) &
            (df['胎心'].notna())
            ]

        # 随机抽样填补
        if not candidates.empty:
            fill_value = candidates.sample(1, random_state=42)['胎心'].values[0]
            '''
            .values[0] 功能：将提取的值转换为基本数据类型（如 int 或 float）。
            '''
            df.loc[idx, '胎心'] = fill_value

    Peiyamissing_mask = df['胚芽'].isna()

    for idx in df[Peiyamissing_mask].index:
        # 获取当前样本特征
        et_day = df.loc[idx, '1ET天']

        # 定义候选数据条件
        candidates = df[
            (df['1ET天'].between(et_day - 5, et_day + 5)) &
            (df['胚芽'].notna())
            ]

        # 随机抽样填补
        if not candidates.empty:
            fill_value = candidates.sample(1, random_state=42)['胚芽'].values[0]
            '''
            .values[0] 功能：将提取的值转换为基本数据类型（如 int 或 float）。
            '''
            df.loc[idx, '胚芽'] = fill_value
        else:
            print('索引',idx,'胚芽补充未完成')
    return df


# 执行填补
df = stratified_imputation(df.copy())


df_cleaned = df.dropna(
    subset=['1ET天', '胎心','卵黄囊'],  # 指定要检查的列（可选）
    how='any',                # 默认：只要有一个缺失值就删除行
    inplace=False             # 是否直接修改原DataFrame（False时返回新DataFrame）
)
Peiyamissing_mask = df_cleaned['胚芽'].isna()
print('剩余总数据: ',len(df_cleaned['胚芽']))
print(f"需填补缺失记录数: {Peiyamissing_mask.sum()}")

'''
numeric_rows = df_cleaned[df_cleaned['胚芽'].notna()].copy()

numeric_df = df_cleaned.select_dtypes(include=['number'])

numeric_df_clean = numeric_df.replace([np.inf, -np.inf], np.nan).dropna()

significant_features = []  # 创建空列表存储显著相关特征
for col in numeric_df_clean.columns.difference(['胚芽']):  # 遍历除"胎心"外的所有列
    r, p = pearsonr(numeric_df_clean['胚芽'], numeric_df_clean[col])     # 计算Pearson相关系数和p值
    #if p < 0.05 and abs(r) > 0.5:           # 双重筛选条件
    significant_features.append((col, r, p))  # 记录通过筛选的特征
print(significant_features)
'''
#胚芽[('1ET天', 0.5640404747936203, 4.107442812668834e-172), ('X', 0.48352020130912393, 2.641831625313337e-120), ('Y', 0.11413398717839106, 2.2779739146413303e-07), ('em', -0.019433707382646537, 0.3797427213071845), ('xyAverage', 0.20075754706394516, 4.885515556117555e-20), ('Ⅲ线_Ⅰ', 0.027565228069785182, 0.21275690086417137), ('Ⅲ线_Ⅱ', -0.027118145328980273, 0.22027258941874323), ('Ⅲ线_Ⅱ-Ⅲ', -0.019028041347972997, 0.3897715852173599), ('Ⅲ线_Ⅲ', 0.04103746692949072, 0.06353485129102633), ('Ⅲ线_有积液', 0.004467142207697274, 0.8400033828834036), ('Ⅲ线_欠清', -0.01692078263212345, 0.44440669703142094), ('助孕方式_binary', 0.03026100636404649, 0.17133399037374153), ('卵黄囊', 0.024238306483747685, 0.2732592814917209), ('年龄', -0.02500628457243258, 0.2583450177241828), ('胎心', 0.009722284445992697, 0.6603720393858221)]
#卵黄囊[('1ET天', 0.06064483845843858, 0.006082330875321874), ('X', 0.032309851666697496, 0.1441269974060472), ('Y', 0.010459809563203524, 0.6364030768463538), ('em', -0.002668379295678076, 0.9040118133921244), ('xyAverage', 0.016134387404864798, 0.4658637961082479), ('Ⅲ线_Ⅰ', -0.0005399699714384121, 0.9805308210469983), ('Ⅲ线_Ⅱ', 0.004980620783669281, 0.8219048218833916), ('Ⅲ线_Ⅱ-Ⅲ', 0.0006037849661282244, 0.978230445498497), ('Ⅲ线_Ⅲ', -0.019490669157551797, 0.37834731764072294), ('Ⅲ线_有积液', 8.043087064957911e-05, 0.9970997000521493), ('Ⅲ线_欠清', 0.02048104391925521, 0.3545925807832572), ('助孕方式_binary', -0.019626248888335685, 0.3750387084989111), ('年龄', 0.008010069920717193, 0.7173422245909712), ('胎心', -0.14934334866855703, 1.1407403490283489e-11), ('胚芽', 0.024238306483747685, 0.2732592814917209)]

'''
p 代表线性相关性，r 代表认为两者线性无关的概率
r的绝对值	​关系强度	​典型应用场景
​0.8~1.0	极强相关	物理实验、工程数据（如温度与电阻）
​0.6~0.8	强相关	生物学、医学指标（如药物剂量与疗效）
​0.4~0.6	中等相关	社会科学、经济学（如收入与消费）
​0.2~0.4	弱相关	探索性数据分析
​0.0~0.2	无/极弱相关	噪声数据或无关变量

p值范围	​统计显著性	​解读
​p < 0.01	高度显著	极强证据拒绝原假设（强相关性）
​0.01 ≤ p < 0.05	显著	有证据拒绝原假设（相关性存在）
​p ≥ 0.05	不显著	无法拒绝原假设（可能无相关性）
'''
df_cleaned['结果'] = 0

df_cleaned.to_excel('2010-2016早流(异常)样本(脱敏)输出.xlsx', index=False)
