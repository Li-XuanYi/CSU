%% 任务 1：Fourier 变换与幅度谱
clear; close all; clc;
syms t w real
x = exp(-t)*heaviside(t);     % e^{-t} ε(t)
X = fourier(x,t,w);           % 得 1/(1 + j*w)

% 幅度谱
fplot(@(w) abs(1./(1+1i*w)), [-20 20], 'LineWidth',1.6);
xlabel('\omega (rad/s)'); ylabel('|X(j\omega)|');
title('|X(j\omega)|  for  x(t)=e^{-t}u(t)');
grid on;

%% 任务 2：Inverse Fourier 变换与时域波形
syms w t real
X2 = 1/(w^2 + 1);
x2 = ifourier(X2,w,t,'sym');   % π*exp(-abs(t))

% 绘制 x2(t)=π e^{-|t|}
fplot(@(t) pi*exp(-abs(t)), [-4 4], 'LineWidth',1.6);
xlabel('t (s)'); ylabel('x_2(t)');
title('x_2(t)=\pi e^{-|t|}');
grid on;
%% 任务 3：频率响应 |H(jw)| 与 ∠H(jw)
%% 连续-时间系统频率响应
% 微分方程: 5 y'' + 3 y' + 2 y = f
% 对应传递函数: H(s) = 1 / (5 s^2 + 3 s + 2)

close all; clc;

s = tf('s');                     % 定义拉普拉斯变量 s
H = 1/(5*s^2 + 3*s + 2);         % 传递函数

w = linspace(0, 10, 2000);       % 自行修改上限以查看更多/更少频段
[mag, phase] = bode(H, w);       % |H(jω)| 和 ∠H(jω) (deg)
mag   = squeeze(mag);
phase = squeeze(phase);

figure;
plot(w, mag, 'LineWidth', 1.2);
xlabel('\omega (rad/s)'); ylabel('|H(j\omega)|');
title('Magnitude Response (linear \omega)');
grid on;

figure;
plot(w, phase, 'LineWidth', 1.2);
xlabel('\omega (rad/s)'); ylabel('\angle H(j\omega)  (deg)');
title('Phase Response (linear \omega)');
grid on;

