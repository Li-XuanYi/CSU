%% 编程实验3：信号与系统的频域分析
% 日期: 2025-06-06

% --- 准备工作 ---
clc;            % 清空命令行窗口
clear;          % 清空工作区变量
close all;      % 关闭所有图形窗口

% 定义符号变量
% t 用于时域, s 用于频域
syms t s;


%% 任务1: 使用 laplace 函数求单边拉普拉斯变换

disp('-------------------- 任务1: 拉普拉斯变换 --------------------');

% 1.1 求 f(t) = e^(-3t)u(t) 的拉普拉斯变换
f1 = exp(-3*t);
F1 = laplace(f1, t, s);
disp('f1(t) = exp(-3*t) 的拉普拉斯变换 F1(s) 为:');
pretty(F1);

% 1.2 求 f(t) = cos(4t)u(t) 的拉普拉斯变换
f2 = cos(4*t);
F2 = laplace(f2, t, s);
disp('f2(t) = cos(4*t) 的拉普拉斯变换 F2(s) 为:');
pretty(F2);

% 1.3 求 f(t) = e^(-3t)cos(4t)u(t) 的拉普拉斯变换
f3 = exp(-3*t) * cos(4*t);
F3 = laplace(f3, t, s);
disp('f3(t) = exp(-3*t)*cos(4*t) 的拉普拉斯变换 F3(s) 为:');
pretty(F3);


%% 任务2: 使用 ilaplace 函数求单边拉普拉斯逆变换

disp(' '); % 添加空行以分隔
disp('-------------------- 任务2: 拉普拉斯逆变换 --------------------');

% 2.1 求 F(s) = 1/(s^2 - 4s + 3) 的拉普拉斯逆变换
F4 = 1 / (s^2 - 4*s + 3);
f4 = ilaplace(F4, s, t);
disp('F4(s) = 1/(s^2 - 4s + 3) 的拉普拉斯逆变换 f4(t) 为:');
pretty(f4);

% 2.2 求 F(s) = (4s^2 + 11s + 10) / (2s^2 + 5s + 3) 的拉普拉斯逆变换
F5 = (4*s^2 + 11*s + 10) / (2*s^2 + 5*s + 3);
f5 = ilaplace(F5, s, t);
disp('F5(s) = (4s^2 + 11s + 10)/(2s^2 + 5s + 3) 的拉普拉斯逆变换 f5(t) 为:');
pretty(f5);


%% 任务3: 使用 residue 函数进行部分分式展开，并求逆变换

disp(' '); % 添加空行以分隔
disp('-------------------- 任务3: 部分分式展开及逆变换 --------------------');

% 注意: residue 函数处理的是多项式系数向量，而不是符号表达式。

% --- 3.1 对 F(s) = 1/(s^2 - 4s + 3) 进行处理 ---
disp('--- 3.1 分析 F(s) = 1/(s^2 - 4s + 3) ---');
num1 = [1]; % 分子多项式系数: 1
den1 = [1, -4, 3]; % 分母多项式系数: 1*s^2 - 4*s + 3
% [r, p, k] 分别是留数、极点、直接项
[r1, p1, k1] = residue(num1, den1);

% 显示部分分式展开结果
fprintf('部分分式展开结果 F(s) = r(1)/(s-p(1)) + r(2)/(s-p(2)) + ... + k\n');
fprintf('留数 (Residues) r1 = \n');
disp(r1');
fprintf('极点 (Poles) p1 = \n');
disp(p1');
fprintf('直接项 (Direct term) k1 = \n');
disp(k1);
fprintf('所以, F(s) = %.1f/(s - %.0f) + %.1f/(s - %.0f)\n', r1(1), p1(1), r1(2), p1(2));

% 基于基本变换对求解逆变换
% L^{-1}{A/(s-a)} = A*exp(a*t)u(t)
f_manual_1 = r1(1)*exp(p1(1)*t) + r1(2)*exp(p1(2)*t);
disp('基于部分分式展开手动求得的逆变换 f(t) 为:');
pretty(f_manual_1);
disp('这与任务2中使用 ilaplace 的结果是一致的。');

disp(' ');

% --- 3.2 对 F(s) = (4s^2 + 11s + 10) / (2s^2 + 5s + 3) 进行处理 ---
disp('--- 3.2 分析 F(s) = (4s^2 + 11s + 10) / (2s^2 + 5s + 3) ---');
% 分子分母的最高次幂相同，是假分式
num2 = [4, 11, 10];     % 分子: 4*s^2 + 11*s + 10
den2 = [2, 5, 3];       % 分母: 2*s^2 + 5*s + 3
[r2, p2, k2] = residue(num2, den2);

% 显示部分分式展开结果
fprintf('部分分式展开结果 F(s) = r(1)/(s-p(1)) + r(2)/(s-p(2)) + ... + k\n');
fprintf('留数 (Residues) r2 = \n');
disp(r2');
fprintf('极点 (Poles) p2 = \n');
disp(p2');
fprintf('直接项 (Direct term) k2 = \n');
disp(k2);
fprintf('所以, F(s) = %.1f/(s - (%.1f)) + %.1f/(s - (%.1f)) + %.1f\n', r2(1), p2(1), r2(2), p2(2), k2);

% 基于基本变换对求解逆变换
% L^{-1}{k} = k*delta(t)
% L^{-1}{A/(s-a)} = A*exp(a*t)u(t)
% 注意: k2 是一个常数项，它的逆变换是狄拉克delta函数(冲激函数)
f_manual_2 = r2(1)*exp(p2(1)*t) + r2(2)*exp(p2(2)*t) + k2 * dirac(t);
disp('基于部分分式展开手动求得的逆变换 f(t) 为:');
pretty(f_manual_2);
disp('这与任务2中使用 ilaplace 的结果是一致的。');