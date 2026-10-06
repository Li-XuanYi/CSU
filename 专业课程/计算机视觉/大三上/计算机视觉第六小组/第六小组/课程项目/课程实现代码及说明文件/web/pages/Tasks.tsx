import React, { useEffect, useState } from 'react';
import { TaskProgress, TaskType } from '../types';
import { getProcessingProgress, controlProcessingTask } from '../services/api';
import { Loader2, Play, Pause, RotateCcw, Image, ScanFace, FileText, Search, AlertCircle } from 'lucide-react';

const TASK_CONFIG: Record<TaskType, { name: string; icon: React.ReactNode; description: string }> = {
    thumbnail: {
        name: '缩略图生成',
        icon: <Image size={24} />,
        description: '生成高质量的图片缩略图，优化浏览体验'
    },
    clip: {
        name: '语义搜索索引',
        icon: <Search size={24} />,
        description: '使用 CLIP 模型分析图片内容，实现自然语言搜索'
    },
    face: {
        name: '人脸识别',
        icon: <ScanFace size={24} />,
        description: '检测并识别照片中的人脸，自动聚类人物相册'
    },
    ocr: {
        name: '文字识别',
        icon: <FileText size={24} />,
        description: '提取照片中的文字信息，支持文档检索'
    }
};

export const Tasks: React.FC = () => {
    const [tasks, setTasks] = useState<TaskProgress[]>([]);
    const [loading, setLoading] = useState(true);
    const [operatingTask, setOperatingTask] = useState<string | null>(null);

    const fetchStatus = async () => {
        try {
            const data = await getProcessingProgress();
            setTasks(data.tasks);
        } catch (e) {
            console.error("Failed to fetch status", e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchStatus();
        const interval = setInterval(fetchStatus, 2000);
        return () => clearInterval(interval);
    }, []);

    const handleControl = async (task: TaskType, action: 'pause' | 'resume' | 'reset') => {
        if (action === 'reset' && !confirm(`确定要重置 ${TASK_CONFIG[task].name} 吗？\n这将清空已处理的数据并重新开始。`)) {
            return;
        }

        setOperatingTask(`${task}-${action}`);
        try {
            await controlProcessingTask(task, action);
            await fetchStatus();
        } catch (e) {
            alert("操作失败");
        } finally {
            setOperatingTask(null);
        }
    };

    if (loading && tasks.length === 0) {
        return (
            <div className="flex h-[50vh] items-center justify-center">
                <Loader2 className="animate-spin text-accent" size={32} />
            </div>
        );
    }

    return (
        <div className="flex flex-col gap-8 animate-fade-in max-w-6xl mx-auto pb-20">
            {/* Header */}
            <div>
                <h2 className="text-3xl font-light tracking-tight text-white">任务管理</h2>
                <p className="text-secondary mt-1 text-sm">监控并控制 AI 处理流水线</p>
            </div>

            {/* Tasks Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {tasks.map((task) => {
                    const config = TASK_CONFIG[task.task];
                    const percentage = Math.round((task.done / (task.total || 1)) * 100);
                    const isRunning = task.state === 'running';

                    return (
                        <div key={task.task} className="bg-surface border border-border rounded-xl p-6 flex flex-col gap-6 hover:border-white/10 transition-colors">

                            {/* Card Header */}
                            <div className="flex items-start justify-between">
                                <div className="flex items-center gap-4">
                                    <div className="p-3 bg-white/5 rounded-lg text-secondary">
                                        {config.icon}
                                    </div>
                                    <div>
                                        <h3 className="text-lg font-medium text-white">{config.name}</h3>
                                        <p className="text-xs text-secondary mt-1">{config.description}</p>
                                    </div>
                                </div>
                                <div className={`flex items-center gap-2 px-3 py-1 rounded-full text-xs font-medium border ${isRunning
                                        ? 'bg-green-500/10 text-green-500 border-green-500/20'
                                        : 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20'
                                    }`}>
                                    <div className={`w-1.5 h-1.5 rounded-full ${isRunning ? 'bg-green-500 animate-pulse' : 'bg-yellow-500'}`} />
                                    {isRunning ? '进行中' : (task.state === 'paused' ? '已暂停' : '等待中')}
                                </div>
                            </div>

                            {/* Progress */}
                            <div className="space-y-2">
                                <div className="flex justify-between text-xs text-secondary">
                                    <span>进度 ({percentage}%)</span>
                                    <span>{task.done} / {task.total}</span>
                                </div>
                                <div className="w-full bg-white/5 rounded-full h-2 overflow-hidden">
                                    <div
                                        className={`h-full transition-all duration-500 ease-out ${isRunning ? 'bg-accent' : 'bg-secondary'}`}
                                        style={{ width: `${Math.min(100, percentage)}%` }}
                                    />
                                </div>
                                <div className="flex items-center gap-4 text-xs text-secondary mt-2">
                                    <span>待处理: {task.pending}</span>
                                </div>
                            </div>

                            {/* Controls */}
                            <div className="pt-4 border-t border-white/5 flex items-center justify-between">
                                <button
                                    disabled={!!operatingTask}
                                    onClick={() => handleControl(task.task, isRunning ? 'pause' : 'resume')}
                                    className={`flex items-center gap-2 text-sm font-medium transition-colors ${isRunning
                                            ? 'text-yellow-500 hover:text-yellow-400'
                                            : 'text-green-500 hover:text-green-400'
                                        }`}
                                >
                                    {operatingTask?.startsWith(task.task) && operatingTask.includes(isRunning ? 'pause' : 'resume') ? (
                                        <Loader2 size={16} className="animate-spin" />
                                    ) : (
                                        isRunning ? <Pause size={16} /> : <Play size={16} />
                                    )}
                                    {isRunning ? '暂停任务' : '继续任务'}
                                </button>

                                <button
                                    disabled={!!operatingTask}
                                    onClick={() => handleControl(task.task, 'reset')}
                                    className="flex items-center gap-2 text-sm text-secondary hover:text-red-400 transition-colors"
                                >
                                    {operatingTask === `${task.task}-reset` ? (
                                        <Loader2 size={16} className="animate-spin" />
                                    ) : (
                                        <RotateCcw size={16} />
                                    )}
                                    重置
                                </button>
                            </div>
                        </div>
                    );
                })}
            </div>

            {tasks.length === 0 && !loading && (
                <div className="text-center py-20 text-secondary flex flex-col items-center">
                    <AlertCircle size={48} className="mb-4 opacity-50" />
                    <p>无法获取任务状态，请检查服务连接。</p>
                </div>
            )}
        </div>
    );
};
