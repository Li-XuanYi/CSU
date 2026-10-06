import React, { useState, useEffect, useRef } from 'react';
import { Send, Sparkles, Loader2, Bot, User, RefreshCw, AlertCircle, Settings, ChevronDown, ChevronUp } from 'lucide-react';
import { createLlavaConversation, getLlavaConversation, continueLlavaConversation } from '../services/api';
import { LlavaConversationResponse, CreateLlavaRequest } from '../types';

interface LlavaChatProps {
    photoId: number;
    onClose?: () => void;
}

export const LlavaChat: React.FC<LlavaChatProps> = ({ photoId, onClose }) => {
    const [conversation, setConversation] = useState<LlavaConversationResponse | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [isSending, setIsSending] = useState(false);
    const [input, setInput] = useState('');
    const [error, setError] = useState<string | null>(null);
    const scrollRef = useRef<HTMLDivElement>(null);

    // Config State
    const [showConfig, setShowConfig] = useState(false);
    const [config, setConfig] = useState<CreateLlavaRequest>({
        query: '这张图里有什么？',
        model: 'liuhaotian/llava-v1.5-7b',
        temperature: 0.2,
        top_p: 0.9,
        num_beams: 1,
        max_new_tokens: 512
    });

    // Poll for updates
    useEffect(() => {
        let interval: NodeJS.Timeout;

        if (conversation && (conversation.status === 'running' || conversation.status === 'created' || conversation.status === 'pending')) {
            interval = setInterval(async () => {
                try {
                    const updated = await getLlavaConversation(photoId, conversation.id);
                    setConversation(updated);
                    if (updated.status !== 'running' && updated.status !== 'created' && updated.status !== 'pending') {
                        clearInterval(interval);
                    }
                } catch (e) {
                    console.error("Polling error", e);
                }
            }, 2000);
        }

        return () => clearInterval(interval);
    }, [conversation, photoId]);

    // Auto-scroll
    useEffect(() => {
        if (scrollRef.current) {
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
    }, [conversation?.history]);

    const startConversation = async () => {
        setIsLoading(true);
        setError(null);
        try {
            const res = await createLlavaConversation(photoId, config);
            setConversation(res);
            setShowConfig(false);
        } catch (e) {
            setError('无法启动对话');
            console.error(e);
        } finally {
            setIsLoading(false);
        }
    };

    const handleSend = async (e?: React.FormEvent) => {
        e?.preventDefault();
        if (!input.trim() || !conversation) return;

        const msg = input;
        setInput('');
        setIsSending(true);

        try {
            const res = await continueLlavaConversation(photoId, conversation.id, msg, config);
            setConversation(res);
        } catch (e) {
            setError('发送失败');
        } finally {
            setIsSending(false);
        }
    };

    // Initial Configuration View
    if (!conversation) {
        return (
            <div className="flex flex-col h-full bg-[#18181b]/95 backdrop-blur-xl rounded-2xl overflow-hidden shadow-2xl animate-fade-in w-80 md:w-96 border border-white/5">
                <div className="flex-shrink-0 px-5 py-4 flex items-center justify-between border-b border-white/5">
                    <div className="flex items-center gap-2.5">
                        <Sparkles size={16} className="text-accent" />
                        <span className="font-medium text-sm text-white tracking-wide">图片描述</span>
                    </div>
                    {onClose && (
                        <button
                            onClick={onClose}
                            className="opacity-50 hover:opacity-100 transition-opacity text-white"
                        >
                            <ChevronDown size={18} />
                        </button>
                    )}
                </div>

                <div className="p-6 flex-1 overflow-y-auto custom-scrollbar">
                    <div className="mb-8 p-4 bg-white/[0.02] border border-white/5 rounded-xl">
                        <p className="text-sm text-secondary font-light leading-relaxed">
                            通过 LLAVA，获取图片的描述
                        </p>
                    </div>

                    <div className="space-y-6">
                        <div className="space-y-2">
                            <label className="text-[10px] uppercase font-bold tracking-wider text-secondary/70 ml-1">Prompt</label>
                            <input
                                type="text"
                                value={config.query}
                                onChange={e => setConfig({ ...config, query: e.target.value })}
                                className="w-full bg-transparent border-b border-white/10 focus:border-accent py-2 text-sm text-white placeholder-secondary/30 outline-none transition-colors"
                            />
                        </div>

                        <div className="space-y-2">
                            <button
                                onClick={() => setShowConfig(!showConfig)}
                                className="flex items-center gap-2 text-[10px] uppercase font-bold tracking-wider text-secondary/70 ml-1 hover:text-white transition-colors"
                            >
                                <Settings size={12} />
                                <span>Advanced Settings</span>
                                {showConfig ? <ChevronUp size={12} /> : <ChevronDown size={12} />}
                            </button>

                            {showConfig && (
                                <div className="p-4 space-y-4 bg-white/[0.02] rounded-xl border border-white/5 animate-fade-in">
                                    <div className="space-y-1">
                                        <label className="text-[10px] text-secondary">Model</label>
                                        <input
                                            type="text"
                                            value={config.model}
                                            onChange={e => setConfig({ ...config, model: e.target.value })}
                                            className="w-full bg-transparent border-b border-white/10 py-1 text-xs text-white focus:border-accent outline-none font-mono"
                                        />
                                    </div>
                                    <div className="grid grid-cols-2 gap-4">
                                        <div className="space-y-1">
                                            <label className="text-[10px] text-secondary">Temperature</label>
                                            <input
                                                type="number"
                                                step="0.1"
                                                value={config.temperature}
                                                onChange={e => setConfig({ ...config, temperature: parseFloat(e.target.value) })}
                                                className="w-full bg-transparent border-b border-white/10 py-1 text-xs text-white focus:border-accent outline-none font-mono"
                                            />
                                        </div>
                                        <div className="space-y-1">
                                            <label className="text-[10px] text-secondary">Top P</label>
                                            <input
                                                type="number"
                                                step="0.1"
                                                value={config.top_p}
                                                onChange={e => setConfig({ ...config, top_p: parseFloat(e.target.value) })}
                                                className="w-full bg-transparent border-b border-white/10 py-1 text-xs text-white focus:border-accent outline-none font-mono"
                                            />
                                        </div>
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                <div className="p-5 border-t border-white/5 bg-white/[0.02]">
                    <button
                        onClick={startConversation}
                        disabled={isLoading}
                        className="w-full flex items-center justify-center gap-2 bg-white text-black hover:bg-white/90 py-3 rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed font-medium text-sm"
                    >
                        {isLoading ? <Loader2 className="animate-spin" size={16} /> : <Sparkles size={16} />}
                        <span>开始生成描述</span>
                    </button>
                    {error && <p className="text-red-400 text-xs mt-3 text-center font-light">{error}</p>}
                </div>
            </div>
        );
    }

    // Active Conversation View
    return (
        <div className="flex flex-col h-full bg-[#18181b]/95 backdrop-blur-xl rounded-2xl overflow-hidden shadow-2xl animate-fade-in w-80 md:w-96 border border-white/5">
            {/* Header */}
            <div className="flex-shrink-0 px-5 py-4 flex items-center justify-between border-b border-white/5">
                <div className="flex items-center gap-2.5">
                    <div className={`w-1.5 h-1.5 rounded-full ${conversation.status === 'running' || conversation.status === 'created' || conversation.status === 'pending'
                        ? 'bg-accent animate-pulse'
                        : conversation.status === 'failed' ? 'bg-red-500' : 'bg-green-500'
                        }`} />
                    <span className="font-medium text-sm text-white tracking-wide">图片描述</span>
                </div>
                {onClose && (
                    <button
                        onClick={onClose}
                        className="opacity-50 hover:opacity-100 transition-opacity text-white"
                    >
                        <ChevronDown size={18} />
                    </button>
                )}
            </div>

            {/* Messages (Text Only, No Bubbles) */}
            <div ref={scrollRef} className="flex-1 overflow-y-auto px-5 py-2 space-y-6 scrollbar-thin scrollbar-thumb-white/10 scrollbar-track-transparent">
                {conversation.history?.map((msg, idx) => (
                    <div key={idx} className="animate-fade-in space-y-1">
                        <div className="flex items-center gap-2">
                            <span className={`text-[10px] uppercase font-bold tracking-wider ${msg.role === 'user' ? 'text-secondary/60' : 'text-accent'
                                }`}>
                                {msg.role === 'user' ? 'You' : 'Analysis'}
                            </span>
                        </div>
                        <div className={`text-sm leading-relaxed ${msg.role === 'user'
                            ? 'text-white font-medium'
                            : 'text-gray-300 font-light'
                            }`}>
                            {msg.content}
                        </div>
                    </div>
                ))}

                {(conversation.status === 'running' || conversation.status === 'created' || conversation.status === 'pending') && (
                    <div className="space-y-1 animate-pulse">
                        <span className="text-[10px] uppercase font-bold tracking-wider text-accent">Analysis</span>
                        <div className="flex items-center gap-1.5 text-secondary">
                            <span className="text-sm font-light">正在生成描述...</span>
                        </div>
                    </div>
                )}
            </div>

            {/* Input */}
            <div className="p-4 border-t border-white/5 bg-white/[0.02]">
                <form onSubmit={handleSend} className="relative flex items-center gap-2">
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        disabled={isSending || ['running', 'created', 'pending'].includes(conversation.status)}
                        placeholder="输入问题..."
                        className="flex-1 bg-transparent border-b border-white/10 focus:border-accent py-2 text-sm text-white placeholder-secondary/30 outline-none transition-colors disabled:opacity-50"
                    />
                    <button
                        type="submit"
                        disabled={!input.trim() || isSending || ['running', 'created', 'pending'].includes(conversation.status)}
                        className="p-2 text-white/50 hover:text-white hover:bg-white/5 rounded-full transition-all disabled:opacity-30"
                    >
                        {isSending ? <Loader2 size={16} className="animate-spin" /> : <Send size={16} />}
                    </button>
                </form>
            </div>
        </div>
    );
};
