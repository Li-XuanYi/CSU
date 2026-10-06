import React, { useCallback, useState, useRef, useEffect } from 'react';
import { Upload as UploadIcon, Check, AlertCircle, Loader2, FileImage, X, Plus } from 'lucide-react';
import { uploadPhoto } from '../services/api';
import { useNavigate } from 'react-router-dom';

interface FileItem {
    file: File;
    id: string;
    preview: string;
    status: 'pending' | 'uploading' | 'success' | 'error';
    error?: string;
}

export const Upload: React.FC = () => {
    const [isDragging, setIsDragging] = useState(false);
    const [fileItems, setFileItems] = useState<FileItem[]>([]);
    const [uploading, setUploading] = useState(false);
    const navigate = useNavigate();
    const fileInputRef = useRef<HTMLInputElement>(null);

    // Cleanup previews
    useEffect(() => {
        return () => {
            fileItems.forEach(item => URL.revokeObjectURL(item.preview));
        };
    }, []);

    const handleDrag = useCallback((e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        if (e.type === 'dragenter' || e.type === 'dragover') {
            setIsDragging(true);
        } else if (e.type === 'dragleave') {
            setIsDragging(false);
        }
    }, []);

    const handleDrop = useCallback((e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        setIsDragging(false);
        if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
            processFiles(Array.from(e.dataTransfer.files));
        }
    }, []);

    const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files && e.target.files.length > 0) {
            processFiles(Array.from(e.target.files));
        }
    };

    const processFiles = (newFiles: File[]) => {
        const validFiles = newFiles.filter(file => file.type.startsWith('image/'));
        const newItems: FileItem[] = validFiles.map(file => ({
            file,
            id: Math.random().toString(36).substr(2, 9),
            preview: URL.createObjectURL(file),
            status: 'pending'
        }));
        setFileItems(prev => [...prev, ...newItems]);
    };

    const removeFile = (id: string) => {
        setFileItems(prev => prev.filter(item => item.id !== id));
    };

    const formatSize = (bytes: number) => {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    };

    const handleUpload = async () => {
        if (fileItems.length === 0 || uploading) return;

        setUploading(true);
        let successCount = 0;

        // Clone items to avoid state mutation issues during async loop
        const itemsToUpload = [...fileItems];

        for (let i = 0; i < itemsToUpload.length; i++) {
            const item = itemsToUpload[i];

            // Skip already uploaded
            if (item.status === 'success') {
                successCount++;
                continue;
            }

            // Update status to uploading
            setFileItems(prev => prev.map(p => p.id === item.id ? { ...p, status: 'uploading' } : p));

            try {
                await uploadPhoto(item.file);
                setFileItems(prev => prev.map(p => p.id === item.id ? { ...p, status: 'success' } : p));
                successCount++;
            } catch (e) {
                console.error(`Failed to upload ${item.file.name}`, e);
                setFileItems(prev => prev.map(p => p.id === item.id ? { ...p, status: 'error', error: 'Upload failed' } : p));
            }
        }

        setUploading(false);

        // If all success, maybe navigate or show toast
        if (successCount === itemsToUpload.length) {
            setTimeout(() => navigate('/'), 1000);
        }
    };

    const pendingCount = fileItems.filter(f => f.status === 'pending').length;
    const totalSize = fileItems.reduce((acc, curr) => acc + curr.file.size, 0);

    return (
        <div className="min-h-screen pt-24 pb-20 px-6 max-w-3xl mx-auto flex flex-col animate-fade-in">
            <div className="flex items-end justify-between mb-8">
                <div>
                    <h1 className="text-3xl font-light text-white tracking-tight mb-2">上传照片</h1>
                    <p className="text-secondary text-sm font-light">
                        {fileItems.length === 0
                            ? "拖拽或选择照片添加到库"
                            : `${fileItems.length} 个文件 · ${formatSize(totalSize)}`
                        }
                    </p>
                </div>
                {fileItems.length > 0 && !uploading && (
                    <button
                        onClick={() => fileInputRef.current?.click()}
                        className="p-2 rounded-full bg-white/5 hover:bg-white/10 text-white transition-colors"
                    >
                        <Plus size={20} />
                    </button>
                )}
            </div>

            {/* Drop Zone (Empty State) */}
            {fileItems.length === 0 ? (
                <div
                    onDragEnter={handleDrag}
                    onDragLeave={handleDrag}
                    onDragOver={handleDrag}
                    onDrop={handleDrop}
                    onClick={() => fileInputRef.current?.click()}
                    className={`
                        flex-1 min-h-[400px] border border-dashed rounded-3xl flex flex-col items-center justify-center gap-6 cursor-pointer transition-all duration-500 group
                        ${isDragging ? 'border-accent bg-accent/5 scale-[1.01]' : 'border-white/10 hover:border-white/20 hover:bg-white/[0.02]'}
                    `}
                >
                    <div className="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center text-secondary group-hover:text-white group-hover:scale-110 transition-all duration-300">
                        <UploadIcon size={24} />
                    </div>
                    <div className="text-center">
                        <p className="text-lg text-white font-light group-hover:tracking-wide transition-all">点击或拖拽上传</p>
                    </div>
                </div>
            ) : (
                <div className="flex-1 space-y-3 mb-24">
                    {/* File List */}
                    {fileItems.map((item) => (
                        <div
                            key={item.id}
                            className="group relative bg-[#18181b] border border-white/5 rounded-xl p-3 flex items-center gap-4 hover:border-white/10 transition-all duration-300 animate-slide-up"
                        >
                            <div className="w-12 h-12 rounded-lg overflow-hidden bg-white/5 flex-shrink-0 relative">
                                <img src={item.preview} alt="" className="w-full h-full object-cover" />
                                {item.status === 'uploading' && (
                                    <div className="absolute inset-0 bg-black/50 flex items-center justify-center">
                                        <Loader2 size={16} className="text-white animate-spin" />
                                    </div>
                                )}
                                {item.status === 'success' && (
                                    <div className="absolute inset-0 bg-green-500/20 flex items-center justify-center">
                                        <div className="bg-green-500 rounded-full p-0.5">
                                            <Check size={10} className="text-white" />
                                        </div>
                                    </div>
                                )}
                            </div>

                            <div className="flex-1 min-w-0">
                                <p className="text-sm text-white truncate font-medium">{item.file.name}</p>
                                <div className="flex items-center gap-2 mt-0.5">
                                    <span className="text-xs text-secondary/70 font-mono">{formatSize(item.file.size)}</span>
                                    {item.status === 'error' && <span className="text-xs text-red-400 flex items-center gap-1"><AlertCircle size={10} /> 失败</span>}
                                    {item.status === 'uploading' && <span className="text-xs text-accent">上传中...</span>}
                                    {item.status === 'success' && <span className="text-xs text-green-400">已完成</span>}
                                    {item.status === 'pending' && <span className="text-xs text-secondary/50">等待中</span>}
                                </div>
                            </div>

                            {item.status === 'pending' && !uploading && (
                                <button
                                    onClick={() => removeFile(item.id)}
                                    className="p-2 text-secondary hover:text-white opacity-0 group-hover:opacity-100 transition-all"
                                >
                                    <X size={16} />
                                </button>
                            )}
                        </div>
                    ))}
                </div>
            )}

            {/* Hidden Input */}
            <input
                type="file"
                ref={fileInputRef}
                className="hidden"
                accept="image/*"
                multiple
                onChange={handleFileInput}
            />

            {/* Bottom Action Bar */}
            {fileItems.length > 0 && (
                <div className="fixed bottom-8 left-1/2 -translate-x-1/2 z-40 animate-slide-in-up">
                    <button
                        onClick={handleUpload}
                        disabled={uploading || pendingCount === 0}
                        className={`
                            px-8 py-3.5 rounded-full font-medium text-sm transition-all shadow-2xl flex items-center gap-2
                            ${uploading
                                ? 'bg-white/10 text-white/50 cursor-not-allowed'
                                : pendingCount === 0
                                    ? 'bg-green-500 text-white hover:bg-green-600'
                                    : 'bg-white text-black hover:bg-gray-200 active:scale-95'
                            }
                        `}
                    >
                        {uploading ? (
                            <>
                                <Loader2 size={16} className="animate-spin" />
                                正在上传...
                            </>
                        ) : pendingCount === 0 ? (
                            <>
                                <Check size={16} />
                                全部完成
                            </>
                        ) : (
                            <>
                                <UploadIcon size={16} />
                                开始上传 {pendingCount} 项
                            </>
                        )}
                    </button>
                    {pendingCount === 0 && !uploading && (
                        <div className="absolute top-full text-center w-full mt-2">
                            <span className="text-xs text-secondary hover:text-white cursor-pointer" onClick={() => navigate('/')}>返回相册</span>
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};