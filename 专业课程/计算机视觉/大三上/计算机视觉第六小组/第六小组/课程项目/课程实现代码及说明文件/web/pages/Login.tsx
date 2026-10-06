import React, { useState } from 'react';
import { login, register } from '../services/api';
import { Aperture, ArrowRight, Loader2 } from 'lucide-react';

interface LoginProps {
  onLoginSuccess: (token: string, user: any) => void;
}

export const Login: React.FC<LoginProps> = ({ onLoginSuccess }) => {
  const [isRegister, setIsRegister] = useState(false);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [username, setUsername] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');

    try {
      let res;
      if (isRegister) {
        res = await register(username, email, password);
      } else {
        res = await login(email, password);
      }

      if (res.code === 0 && res.data) {
        onLoginSuccess(res.data.access_token, res.data.user);
      } else {
        setError(res.msg || '认证失败');
      }
    } catch (e) {
      setError('网络错误或服务器不可用');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-black relative overflow-hidden">
      {/* Background Ambience */}
      <div className="absolute top-[-20%] left-[-10%] w-[500px] h-[500px] bg-accent/20 rounded-full blur-[120px]" />
      <div className="absolute bottom-[-20%] right-[-10%] w-[500px] h-[500px] bg-purple-900/20 rounded-full blur-[120px]" />

      <div className="w-full max-w-md p-8 relative z-10 animate-fade-in">
        <div className="flex justify-center mb-8">
          <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-accent to-purple-600 flex items-center justify-center shadow-lg shadow-accent/20">
            <Aperture className="text-white" size={28} />
          </div>
        </div>

        <h1 className="text-3xl font-bold text-center text-white mb-2 tracking-tight">
          {isRegister ? '创建账户' : '欢迎回来'}
        </h1>
        <p className="text-secondary text-center mb-8 text-sm">
          {isRegister ? '开启您的视觉之旅' : '登录以访问您的画廊'}
        </p>

        <form onSubmit={handleSubmit} className="space-y-4">
           {isRegister && (
            <div className="space-y-1">
              <label className="text-xs font-medium text-secondary ml-1">用户名</label>
              <input
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className="w-full bg-surface border border-border focus:border-accent rounded-xl px-4 py-3 text-white outline-none transition-colors"
                placeholder="请输入用户名"
                required
              />
            </div>
          )}
          
          <div className="space-y-1">
            <label className="text-xs font-medium text-secondary ml-1">邮箱</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full bg-surface border border-border focus:border-accent rounded-xl px-4 py-3 text-white outline-none transition-colors"
              placeholder="name@example.com"
              required
            />
          </div>

          <div className="space-y-1">
            <label className="text-xs font-medium text-secondary ml-1">密码</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full bg-surface border border-border focus:border-accent rounded-xl px-4 py-3 text-white outline-none transition-colors"
              placeholder="••••••••"
              required
            />
          </div>

          {error && (
            <div className="text-red-400 text-sm text-center bg-red-400/10 py-2 rounded-lg border border-red-400/20">
              {error}
            </div>
          )}

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-white text-black font-semibold rounded-xl py-3.5 hover:bg-gray-200 transition-colors flex items-center justify-center gap-2 mt-4"
          >
            {loading ? <Loader2 className="animate-spin" size={20} /> : (
              <>
                <span>{isRegister ? '注册' : '登录'}</span>
                <ArrowRight size={18} />
              </>
            )}
          </button>
        </form>

        <div className="mt-8 text-center">
          <button
            onClick={() => { setIsRegister(!isRegister); setError(''); }}
            className="text-secondary hover:text-white text-sm transition-colors"
          >
            {isRegister ? '已有账户？去登录' : "没有账户？去注册"}
          </button>
        </div>
      </div>
    </div>
  );
};