import React, { useEffect, useState, useRef } from 'react';
import { X, Info, Calendar, Camera, MapPin, Sparkles, Download, Trash2, ChevronLeft, ChevronRight, ZoomIn, ZoomOut, FolderPlus, Edit2, Check, FolderMinus, MessageCircle } from 'lucide-react';
import { Photo, SimilarPhotoResult, Album } from '../types';
import { getSimilarPhotos, deletePhoto, updatePhotoMetadata, getAlbums, addPhotoToAlbum } from '../services/api';
import { LlavaChat } from './LlavaChat';

interface PhotoModalProps {
  photo: Photo | null;
  onClose: () => void;
  onDelete: (id: number) => void;
  onNext?: () => void;
  onPrev?: () => void;
  onRemoveFromAlbum?: (photoId: number) => void;
  onPhotoSelect?: (photo: Photo) => void; // New prop to handle switching to similar photos
  hasNext: boolean;
  hasPrev: boolean;
  originRect?: DOMRect | null;
}

export const PhotoModal: React.FC<PhotoModalProps> = ({
  photo,
  onClose,
  onDelete,
  onNext,
  onPrev,
  onRemoveFromAlbum,
  onPhotoSelect,
  hasNext,
  hasPrev,
  originRect
}) => {
  const [similar, setSimilar] = useState<SimilarPhotoResult[]>([]);
  const [loadingSimilar, setLoadingSimilar] = useState(false);
  const [showConfirmDelete, setShowConfirmDelete] = useState(false);
  const [showChat, setShowChat] = useState(false);

  // Image State
  const [currentSrc, setCurrentSrc] = useState<string>('');
  const [loadingHighRes, setLoadingHighRes] = useState(false);
  const [progress, setProgress] = useState(0);
  const [loadedBytes, setLoadedBytes] = useState(0);
  const [totalBytes, setTotalBytes] = useState(0);

  // Edit Description State
  const [isEditingDesc, setIsEditingDesc] = useState(false);
  const [newDesc, setNewDesc] = useState('');
  const [isSavingDesc, setIsSavingDesc] = useState(false);

  // Add to Album State
  const [showAddToAlbum, setShowAddToAlbum] = useState(false);
  const [albums, setAlbums] = useState<Album[]>([]);
  const [loadingAlbums, setLoadingAlbums] = useState(false);

  // Zoom/Pan State
  const [scale, setScale] = useState(1);
  const [position, setPosition] = useState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const containerRef = useRef<HTMLDivElement>(null);

  const abortControllerRef = useRef<AbortController | null>(null);

  // Animation State
  const [isAnimating, setIsAnimating] = useState(false);
  const [animStyle, setAnimStyle] = useState<React.CSSProperties>({});

  const prevPhotoIdRef = useRef<number | null>(null);

  useEffect(() => {
    if (!photo) return;

    // Reset UI states
    setSimilar([]);
    setShowConfirmDelete(false);
    setScale(1);
    setPosition({ x: 0, y: 0 });
    setLoadedBytes(0);
    setTotalBytes(0);
    setProgress(0);
    setIsEditingDesc(false);
    setShowAddToAlbum(false);
    setShowChat(false);
    setNewDesc(photo.description || '');

    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }

    setCurrentSrc(photo.thumbnail_url || photo.url);

    const controller = new AbortController();
    abortControllerRef.current = controller;

    loadHighResImage(photo.url, controller.signal);

    const isNavigation = prevPhotoIdRef.current !== null && prevPhotoIdRef.current !== photo.id;
    prevPhotoIdRef.current = photo.id;

    // Only run animation on first open (not navigation)
    if (!isNavigation) {
      if (originRect) {
        runEnterAnimation();
      } else {
        // Deep link or missing rect: just fade in
        setIsAnimating(false);
      }
    }

    return () => {
      controller.abort();
    };
  }, [photo]);

  const runEnterAnimation = () => {
    if (!originRect) return;

    setIsAnimating(true);

    const isDesktop = window.innerWidth >= 768;
    const sidebarWidth = isDesktop ? 384 : 0;

    // Initial State (Thumbnail Position)
    setAnimStyle({
      position: 'fixed',
      top: originRect.top,
      left: originRect.left,
      width: originRect.width,
      height: originRect.height,
      zIndex: 100,
      transition: 'none',
      objectFit: 'cover',
      borderRadius: '2px',
      opacity: 1,
    });

    // Forced Reflow
    // requestAnimationFrame ensures browser paints the initial state
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        // Target State (Full Screen)
        const availableWidth = window.innerWidth - sidebarWidth;
        setAnimStyle({
          position: 'fixed',
          top: 0,
          left: 0,
          width: isDesktop ? availableWidth : '100%',
          height: '100%',
          zIndex: 100,
          transition: 'all 0.35s cubic-bezier(0.2, 0, 0.2, 1)', // Improved cubic-bezier
          objectFit: 'contain',
          borderRadius: '0px',
          opacity: 1,
        });
      });
    });

    setTimeout(() => {
      setIsAnimating(false);
    }, 360); // Slightly longer than transition to be safe
  };

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!photo) return;
      if (isEditingDesc) return; // Disable navigation while editing text
      if (e.key === 'Escape') onClose();
      if (e.key === 'ArrowRight' && hasNext && onNext) onNext();
      if (e.key === 'ArrowLeft' && hasPrev && onPrev) onPrev();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [photo, hasNext, hasPrev, onClose, onNext, onPrev, isEditingDesc]);

  useEffect(() => {
    return () => {
      prevPhotoIdRef.current = null;
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }
    }
  }, []);


  const loadHighResImage = async (url: string, signal: AbortSignal) => {
    setLoadingHighRes(true);

    try {
      const response = await fetch(url, { signal });
      if (!response.ok) throw new Error('Network response was not ok');
      if (!response.body) {
        const blob = await response.blob();
        const objectUrl = URL.createObjectURL(blob);
        setCurrentSrc(objectUrl);
        return;
      }
      const contentLength = response.headers.get('content-length');
      const total = contentLength ? parseInt(contentLength, 10) : 0;
      setTotalBytes(total);

      let loaded = 0;
      const reader = response.body.getReader();
      const chunks = [];

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        chunks.push(value);
        loaded += value.length;
        setLoadedBytes(loaded);
        if (total) setProgress(Math.round((loaded / total) * 100));
      }

      const blob = new Blob(chunks);
      const objectUrl = URL.createObjectURL(blob);
      setCurrentSrc(objectUrl);
    } catch (error: any) {
      if (error.name !== 'AbortError') {
        console.error("Error loading high res", error);
      }
    } finally {
      if (!signal.aborted) {
        setLoadingHighRes(false);
        setProgress(100);
      }
    }
  };

  const handleWheel = (e: React.WheelEvent) => {
    e.stopPropagation();
    if (e.deltaY < 0) {
      setScale(s => Math.min(s + 0.2, 5));
    } else {
      setScale(s => Math.max(s - 0.2, 1));
    }
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    if (scale > 1) {
      setIsDragging(true);
      setDragStart({ x: e.clientX - position.x, y: e.clientY - position.y });
    }
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isDragging && scale > 1) {
      setPosition({ x: e.clientX - dragStart.x, y: e.clientY - dragStart.y });
    }
  };

  const handleMouseUp = () => setIsDragging(false);

  const handleFindSimilar = async () => {
    if (!photo) return;
    setLoadingSimilar(true);
    try {
      const results = await getSimilarPhotos(photo.id);
      setSimilar(results);
    } catch (e) {
      console.error(e);
    } finally {
      setLoadingSimilar(false);
    }
  };

  const handleDelete = async () => {
    if (!photo) return;
    try {
      await deletePhoto(photo.id);
      onDelete(photo.id);
      onClose();
    } catch (e) { console.error(e); }
  };

  const handleUpdateDescription = async () => {
    if (!photo) return;
    setIsSavingDesc(true);
    try {
      await updatePhotoMetadata(photo.id, { description: newDesc });
      // Ideally update the photo object in parent, but locally is fine for now
      photo.description = newDesc;
      setIsEditingDesc(false);
    } catch (e) {
      console.error("Failed to update description", e);
    } finally {
      setIsSavingDesc(false);
    }
  };

  const handleAddToAlbumClick = async () => {
    setShowAddToAlbum(!showAddToAlbum);
    if (!showAddToAlbum && albums.length === 0) {
      setLoadingAlbums(true);
      try {
        const data = await getAlbums();
        setAlbums(data);
      } catch (e) {
        console.error(e);
      } finally {
        setLoadingAlbums(false);
      }
    }
  };

  const addToAlbum = async (albumId: number) => {
    if (!photo) return;
    try {
      await addPhotoToAlbum(albumId, photo.id);
      setShowAddToAlbum(false);
      alert('已添加到相册');
    } catch (e) {
      alert('添加失败，可能照片已存在于该相册');
    }
  };

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 MB';
    return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
  };

  if (!photo) return null;

  const dateStr = photo.date_time_original
    ? new Date(photo.date_time_original).toLocaleDateString('zh-CN', { dateStyle: 'long' })
    : '未知日期';

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className={`absolute inset-0 bg-black/95 backdrop-blur-xl transition-opacity duration-300 ${isAnimating ? 'opacity-0' : 'opacity-100'}`}
        onClick={onClose}
      />

      {isAnimating && (
        <img
          src={photo.thumbnail_url || photo.url}
          alt="transition"
          style={animStyle}
          className="pointer-events-none shadow-2xl"
        />
      )}

      {/* Real Modal Content */}
      <div
        className={`relative w-full h-full flex flex-col md:flex-row overflow-hidden bg-[#09090b] transition-opacity duration-200 ${isAnimating ? 'opacity-0' : 'opacity-100'}`}
      >
        <button
          onClick={onClose}
          className="absolute top-4 right-4 z-20 p-2 bg-black/50 text-white rounded-full md:hidden backdrop-blur-sm"
        >
          <X size={20} />
        </button>

        {/* Image Viewer Area */}
        <div
          className="flex-1 bg-[#050505] relative overflow-hidden flex items-center justify-center select-none"
          onWheel={handleWheel}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          ref={containerRef}
        >
          {/* Nav Arrows */}
          {hasPrev && (
            <button
              onClick={(e) => { e.stopPropagation(); onPrev?.(); }}
              className="absolute left-4 z-20 p-3 rounded-full bg-black/40 text-white hover:bg-white/20 hover:scale-110 transition-all backdrop-blur-sm group"
            >
              <ChevronLeft size={24} className="group-hover:-translate-x-0.5 transition-transform" />
            </button>
          )}
          {hasNext && (
            <button
              onClick={(e) => { e.stopPropagation(); onNext?.(); }}
              className="absolute right-4 z-20 p-3 rounded-full bg-black/40 text-white hover:bg-white/20 hover:scale-110 transition-all backdrop-blur-sm group"
            >
              <ChevronRight size={24} className="group-hover:translate-x-0.5 transition-transform" />
            </button>
          )}

          <div className="w-full h-full flex items-center justify-center">
            <img
              src={currentSrc}
              alt={photo.description || 'Photo'}
              className="w-full h-full object-contain transition-transform duration-100 ease-linear shadow-2xl"
              style={{
                transform: `scale(${scale}) translate(${position.x / scale}px, ${position.y / scale}px)`,
                cursor: scale > 1 ? (isDragging ? 'grabbing' : 'grab') : 'default'
              }}
              draggable={false}
            />
          </div>

          {loadingHighRes && progress < 100 && (
            <div className="absolute bottom-10 left-1/2 -translate-x-1/2 z-30 flex items-center gap-4 px-6 py-4 bg-black/80 backdrop-blur-xl rounded-xl border border-white/10 shadow-2xl animate-fade-in">
              <div className="relative">
                <div className="w-6 h-6 border-2 border-white/20 rounded-full" />
                <div className="absolute top-0 left-0 w-6 h-6 border-2 border-accent border-t-transparent rounded-full animate-spin" />
              </div>
              <div className="flex flex-col min-w-[140px]">
                <div className="flex justify-between items-end mb-1">
                  <span className="text-xs font-bold text-white tracking-wide">加载原图中...</span>
                  <span className="text-[10px] font-mono text-accent">{progress}%</span>
                </div>
                <span className="text-[10px] font-mono text-white/50 mb-1.5 block">
                  {formatSize(loadedBytes)} / {totalBytes ? formatSize(totalBytes) : '--'}
                </span>
                <div className="w-full h-1 bg-white/10 rounded-full overflow-hidden">
                  <div className="h-full bg-gradient-to-r from-accent to-blue-400 transition-all duration-300 ease-out" style={{ width: `${progress}%` }} />
                </div>
              </div>
            </div>
          )}

          <div className="absolute bottom-6 left-1/2 -translate-x-1/2 z-20 flex items-center gap-2 px-2 py-1.5 bg-black/60 backdrop-blur-md rounded-full border border-white/10 opacity-0 hover:opacity-100 transition-opacity">
            <button onClick={() => setScale(s => Math.max(1, s - 0.5))} className="p-1.5 hover:text-accent transition-colors"><ZoomOut size={16} /></button>
            <span className="text-xs w-8 text-center">{Math.round(scale * 100)}%</span>
            <button onClick={() => setScale(s => Math.min(5, s + 0.5))} className="p-1.5 hover:text-accent transition-colors"><ZoomIn size={16} /></button>
          </div>

          {/* AI Chat Trigger (Left Bottom) */}
          {!showChat && (
            <button
              onClick={(e) => { e.stopPropagation(); setShowChat(true); }}
              className="absolute bottom-6 left-6 z-20 flex items-center gap-2 px-3 py-2 bg-black/60 backdrop-blur-md rounded-full border border-white/10 text-white hover:bg-white/20 hover:scale-105 transition-all shadow-lg animate-fade-in group"
            >
              <Sparkles size={14} className="text-accent group-hover:rotate-12 transition-transform" />
              <span className="text-xs font-medium">图片描述</span>
            </button>
          )}
        </div>

        {/* Sidebar Info */}
        <div
          className="w-full md:w-96 bg-surface border-l border-border flex flex-col overflow-y-auto custom-scrollbar shrink-0 z-10 shadow-xl"
          style={{
            animation: isAnimating ? 'none' : 'slideInRight 0.5s cubic-bezier(0.2, 0, 0.2, 1)',
            opacity: isAnimating ? 0 : 1
          }}
        >
          <div className="p-6 border-b border-border flex justify-between items-start">
            <div className="flex-1 pr-2">
              {isEditingDesc ? (
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={newDesc}
                    onChange={(e) => setNewDesc(e.target.value)}
                    className="w-full bg-black/30 border border-border rounded px-2 py-1 text-sm text-white focus:outline-none focus:border-accent"
                    placeholder="添加描述..."
                    autoFocus
                  />
                  <button onClick={handleUpdateDescription} disabled={isSavingDesc} className="p-1.5 bg-accent rounded text-white hover:bg-accent/80">
                    <Check size={14} />
                  </button>
                </div>
              ) : (
                <div className="group flex items-center gap-2 cursor-pointer" onClick={() => setIsEditingDesc(true)}>
                  <h2 className={`text-lg font-semibold mb-1 truncate ${!photo.description ? 'text-secondary italic' : 'text-primary'}`} title={photo.description || ''}>
                    {photo.description || '无标题照片'}
                  </h2>
                  <Edit2 size={12} className="text-secondary opacity-0 group-hover:opacity-100 transition-opacity" />
                </div>
              )}
              <p className="text-xs text-secondary font-mono uppercase tracking-wider">
                ID: {photo.id}
              </p>
            </div>
            <button
              onClick={onClose}
              className="hidden md:block p-2 text-secondary hover:text-primary transition-colors shrink-0"
            >
              <X size={20} />
            </button>
          </div>

          <div className="p-6 space-y-8 flex-1">
            {/* Action Buttons */}
            <div className="grid grid-cols-4 gap-2">
              {/* 1. Similar */}
              <button
                onClick={handleFindSimilar}
                disabled={loadingSimilar}
                className="col-span-2 flex items-center justify-center gap-2 bg-accent/10 hover:bg-accent/20 text-accent py-2.5 rounded-lg text-sm font-medium transition-colors border border-accent/20"
                title="查找相似照片"
              >
                <Sparkles size={16} />
                {loadingSimilar ? '思考中...' : '查找相似'}
              </button>

              {/* 2. Add to Album (Relative Box) */}
              <div className="relative">
                <button
                  onClick={handleAddToAlbumClick}
                  className="w-full h-full flex items-center justify-center bg-border hover:bg-white/10 text-secondary hover:text-primary rounded-lg transition-colors border border-transparent"
                  title="添加到相册"
                >
                  <FolderPlus size={18} />
                </button>

                {/* Add to Album Dropdown */}
                {showAddToAlbum && (
                  <div className="absolute top-full left-0 mt-2 w-48 bg-surface border border-border rounded-lg shadow-xl z-50 p-1">
                    {loadingAlbums ? (
                      <div className="p-2 text-center text-xs text-secondary">加载中...</div>
                    ) : albums.length > 0 ? (
                      <div className="max-h-40 overflow-y-auto custom-scrollbar">
                        {albums.map(album => (
                          <button
                            key={album.id}
                            onClick={() => addToAlbum(album.id)}
                            className="w-full text-left px-3 py-2 text-sm text-secondary hover:text-white hover:bg-white/5 rounded truncate"
                          >
                            {album.name}
                          </button>
                        ))}
                      </div>
                    ) : (
                      <div className="p-2 text-center text-xs text-secondary">暂无相册</div>
                    )}
                  </div>
                )}
              </div>

              {/* 3. Download */}
              <a
                href={photo.url}
                download
                target="_blank"
                rel="noreferrer"
                className="flex items-center justify-center bg-border hover:bg-white/10 text-secondary hover:text-primary rounded-lg transition-colors border border-transparent"
                title="下载原图"
              >
                <Download size={18} />
              </a>
            </div>

            {/* Danger Zone: Remove from Album OR Delete Photo */}
            <div className="grid grid-cols-1 gap-2">
              {onRemoveFromAlbum ? (
                <button
                  onClick={() => onRemoveFromAlbum(photo.id)}
                  className="flex items-center justify-center gap-2 py-2.5 bg-orange-500/10 hover:bg-orange-500/20 text-orange-500 rounded-lg transition-colors border border-orange-500/20"
                >
                  <FolderMinus size={16} />
                  <span className="text-sm">从相册移除</span>
                </button>
              ) : (
                <div className="relative">
                  {!showConfirmDelete ? (
                    <button
                      onClick={() => setShowConfirmDelete(true)}
                      className="w-full flex items-center justify-center gap-2 py-2.5 bg-red-500/10 hover:bg-red-500/20 text-red-500 rounded-lg transition-colors border border-red-500/20"
                    >
                      <Trash2 size={16} />
                      <span className="text-sm">删除照片</span>
                    </button>
                  ) : (
                    <div className="p-3 bg-red-900/20 border border-red-900/50 rounded-lg animate-fade-in flex flex-col gap-2">
                      <p className="text-xs text-red-200 text-center">确定彻底删除吗？</p>
                      <div className="flex gap-2 justify-center">
                        <button onClick={handleDelete} className="text-xs bg-red-600 text-white px-3 py-1 rounded hover:bg-red-500">是</button>
                        <button onClick={() => setShowConfirmDelete(false)} className="text-xs bg-transparent border border-white/20 text-white px-3 py-1 rounded hover:bg-white/10">否</button>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>

            <div className="space-y-4">
              <h3 className="text-xs font-bold text-secondary uppercase tracking-widest flex items-center gap-2">
                <Info size={14} /> 图片信息
              </h3>

              <div className="grid grid-cols-2 gap-y-4 gap-x-2 text-sm">
                <div className="flex items-center gap-2 text-secondary">
                  <Calendar size={14} />
                  <span>日期</span>
                </div>
                <div className="text-primary text-right">{dateStr}</div>

                <div className="flex items-center gap-2 text-secondary">
                  <Camera size={14} />
                  <span>相机</span>
                </div>
                <div className="text-primary text-right truncate" title={photo.camera_model || '未知'}>
                  {photo.camera_model || '未知'}
                </div>

                <div className="flex items-center gap-2 text-secondary">
                  <MapPin size={14} />
                  <span>地点</span>
                </div>
                <div className="text-primary text-right">
                  {photo.gps_latitude ? '已定位' : '无位置信息'}
                </div>

                <div className="col-span-2 pt-2 border-t border-border mt-2 grid grid-cols-3 gap-2 text-center">
                  <div className="bg-black/20 p-2 rounded overflow-hidden">
                    <span className="block text-[10px] text-secondary">ISO</span>
                    <span className="block text-sm font-medium truncate" title={String(photo.iso_speed || '')}>{photo.iso_speed || '-'}</span>
                  </div>
                  <div className="bg-black/20 p-2 rounded overflow-hidden">
                    <span className="block text-[10px] text-secondary">光圈</span>
                    <span className="block text-sm font-medium truncate" title={photo.f_number || ''}>{photo.f_number || '-'}</span>
                  </div>
                  <div className="bg-black/20 p-2 rounded overflow-hidden">
                    <span className="block text-[10px] text-secondary">快门</span>
                    <span className="block text-sm font-medium truncate" title={photo.exposure_time || ''}>{photo.exposure_time || '-'}</span>
                  </div>
                </div>
              </div>
            </div>

            {similar.length > 0 && (
              <div className="animate-fade-in pt-4 border-t border-border">
                <h3 className="text-xs font-bold text-secondary uppercase tracking-widest mb-4">
                  视觉相似推荐
                </h3>
                <div className="grid grid-cols-3 gap-2">
                  {similar.map((sim, index) => (
                    <div
                      key={sim.photo.id}
                      className="relative aspect-square rounded-lg overflow-hidden group cursor-pointer border border-border"
                      onClick={() => onPhotoSelect?.(sim.photo)}
                    >
                      <img
                        src={sim.photo.thumbnail_url || sim.photo.url}
                        className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
                        alt="Similar"
                      />
                      <div className="absolute bottom-0 left-0 right-0 bg-black/60 p-1 text-[10px] text-center text-white/80 opacity-0 group-hover:opacity-100 transition-opacity flex flex-col items-center">
                        <span className="font-medium text-white">#{index + 1}</span>
                        <span className="text-white/60 scale-90">距离: {sim.distance.toFixed(3)}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            <style>{`
        @keyframes slideInRight {
          from { transform: translateX(100%); opacity: 0; }
          to { transform: translateX(0); opacity: 1; }
        }
      `}</style>
          </div>
        </div>
      </div>
      {/* Floating Chat UI */}
      {showChat && (
        <div className="fixed left-4 bottom-4 z-50 animate-slide-in-up">
          <LlavaChat photoId={photo.id} onClose={() => setShowChat(false)} />
        </div>
      )}

    </div>
  );
};