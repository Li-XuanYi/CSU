import React, { useEffect, useState, useMemo } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Photo, Album } from '../types';
import { getAlbumPhotos, getAlbums, removePhotoFromAlbum, deleteAlbum, updateAlbum } from '../services/api';
import { PhotoModal } from '../components/PhotoModal';
import { ImageWithPlaceholder } from '../components/ImageWithPlaceholder';
import { ConfirmDialog } from '../components/ConfirmDialog';
import { Loader2, ArrowLeft, Grid as GridIcon, Settings, Trash2, Edit2, Check, X, CheckSquare, FolderMinus } from 'lucide-react';

export const AlbumView: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [photos, setPhotos] = useState<Photo[]>([]);
  const [albumInfo, setAlbumInfo] = useState<Album | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedPhotoIndex, setSelectedPhotoIndex] = useState<number | null>(null);
  const [columns, setColumns] = useState(5);
  const [originRect, setOriginRect] = useState<DOMRect | null>(null);

  // Settings Menu State
  const [showSettings, setShowSettings] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [editName, setEditName] = useState('');

  // Selection Mode
  const [isSelectionMode, setIsSelectionMode] = useState(false);
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  // Dialogs
  const [showDeleteAlbumConfirm, setShowDeleteAlbumConfirm] = useState(false);
  const [showBatchRemoveConfirm, setShowBatchRemoveConfirm] = useState(false);
  const [processing, setProcessing] = useState(false);

  useEffect(() => {
    if (id) {
      loadData(parseInt(id));
    }
  }, [id]);

  const loadData = async (albumId: number) => {
    try {
      const [photosData, albumsData] = await Promise.all([
        getAlbumPhotos(albumId),
        getAlbums()
      ]);
      setPhotos(photosData);
      const currentAlbum = albumsData.find(a => a.id === albumId);
      setAlbumInfo(currentAlbum || null);
      if (currentAlbum) setEditName(currentAlbum.name);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const sortedPhotos = useMemo(() => {
    return [...photos].sort((a, b) => {
      const dateA = new Date(a.date_time_original || a.created_at || 0).getTime();
      const dateB = new Date(b.date_time_original || b.created_at || 0).getTime();
      return dateB - dateA;
    });
  }, [photos]);

  // --- Handlers ---

  const handlePhotoDeleted = (id: number) => {
    setPhotos(photos.filter(p => p.id !== id));
    setSelectedPhotoIndex(null);
  };

  const handleRemoveFromAlbumSingle = async (photoId: number) => {
    // Single remove called from Modal, confirmation handled inside modal usually or here?
    // For improved UX, the modal handles its own single confirmation logic, 
    // but we need to update state here.
    if (!albumInfo) return;
    try {
      await removePhotoFromAlbum(albumInfo.id, photoId);
      setPhotos(photos.filter(p => p.id !== photoId));
      setSelectedPhotoIndex(null);
    } catch (e) {
      alert("移除失败");
    }
  };

  const handleDeleteAlbum = async () => {
    if (!albumInfo) return;
    setProcessing(true);
    try {
      await deleteAlbum(albumInfo.id);
      navigate('/albums');
    } catch (e) {
      alert("删除相册失败");
    } finally {
      setProcessing(false);
    }
  };

  const handleUpdateAlbum = async () => {
    if (!albumInfo || !editName.trim()) return;
    try {
      const updated = await updateAlbum(albumInfo.id, { name: editName });
      setAlbumInfo(updated);
      setIsEditing(false);
    } catch (e) {
      alert("更新失败");
    }
  };

  const handleImageClick = (index: number, id: number, e: React.MouseEvent) => {
    if (isSelectionMode) {
      const newSet = new Set(selectedIds);
      if (newSet.has(id)) newSet.delete(id);
      else newSet.add(id);
      setSelectedIds(newSet);
    } else {
      const rect = e.currentTarget.getBoundingClientRect();
      setOriginRect(rect);
      setSelectedPhotoIndex(index);
    }
  };

  const toggleSelectionMode = () => {
    if (isSelectionMode) {
      setIsSelectionMode(false);
      setSelectedIds(new Set());
    } else {
      setIsSelectionMode(true);
    }
  };

  const handleBatchRemove = async () => {
    if (!albumInfo) return;
    setProcessing(true);
    try {
      await Promise.all(Array.from(selectedIds).map((pid: number) => removePhotoFromAlbum(albumInfo.id, pid)));
      setPhotos(prev => prev.filter(p => !selectedIds.has(p.id)));
      setIsSelectionMode(false);
      setSelectedIds(new Set());
      setShowBatchRemoveConfirm(false);
    } catch (e) {
      alert("部分移除失败");
    } finally {
      setProcessing(false);
    }
  };

  if (loading) {
    return (
      <div className="flex h-[50vh] items-center justify-center">
        <Loader2 className="animate-spin text-accent" size={32} />
      </div>
    );
  }

  const selectedPhoto = selectedPhotoIndex !== null ? sortedPhotos[selectedPhotoIndex] : null;

  return (
    <>
      <div className="flex flex-col gap-4 animate-fade-in relative pb-20" onClick={() => setShowSettings(false)}>
        {/* Header */}
        <header className="mb-2 bg-background/80 backdrop-blur-xl z-30 py-4 sticky top-0 -mx-8 px-8 border-b border-white/5">
          <button
            onClick={() => navigate('/albums')}
            className="flex items-center gap-2 text-secondary hover:text-white mb-4 transition-colors group text-sm"
          >
            <ArrowLeft size={16} className="group-hover:-translate-x-1 transition-transform" />
            <span>返回相册列表</span>
          </button>
          <div className="flex flex-col md:flex-row justify-between items-end md:items-center gap-4">
            <div className="flex-1">
              {isEditing ? (
                <div className="flex items-center gap-2 mb-1">
                  <input
                    type="text"
                    value={editName}
                    onChange={(e) => setEditName(e.target.value)}
                    className="bg-black/20 border border-border rounded px-3 py-1 text-2xl font-light text-white focus:outline-none focus:border-accent w-full max-w-md"
                    autoFocus
                  />
                  <button onClick={handleUpdateAlbum} className="p-2 bg-accent text-white rounded hover:bg-accent/90"><Check size={20} /></button>
                  <button onClick={() => setIsEditing(false)} className="p-2 bg-surface border border-border text-white rounded hover:bg-white/10"><X size={20} /></button>
                </div>
              ) : (
                <div className="flex items-center gap-4">
                  <h2 className="text-3xl font-light tracking-tight text-white">{albumInfo?.name || '相册'}</h2>
                  <div className="relative">
                    <button
                      onClick={(e) => { e.stopPropagation(); setShowSettings(!showSettings); }}
                      className="p-2 text-secondary hover:text-white hover:bg-white/5 rounded-full transition-colors"
                    >
                      <Settings size={20} />
                    </button>
                    {showSettings && (
                      <div className="absolute top-full left-0 mt-2 w-40 bg-[#18181b] border border-white/10 rounded-lg shadow-xl py-1 z-30 animate-fade-in">
                        <button onClick={() => { setIsEditing(true); setShowSettings(false); }} className="w-full text-left px-4 py-2 text-sm text-white hover:bg-white/10 flex items-center gap-2">
                          <Edit2 size={14} /> 重命名
                        </button>
                        <button onClick={() => setShowDeleteAlbumConfirm(true)} className="w-full text-left px-4 py-2 text-sm text-red-400 hover:bg-red-500/10 flex items-center gap-2">
                          <Trash2 size={14} /> 删除相册
                        </button>
                      </div>
                    )}
                  </div>
                </div>
              )}
              <p className="text-secondary mt-1 text-sm">{photos.length} 张照片</p>
            </div>

            <div className="flex items-center gap-3">
              <button
                onClick={toggleSelectionMode}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl text-sm font-medium transition-all ${isSelectionMode
                    ? 'bg-accent text-white shadow-lg shadow-accent/20'
                    : 'bg-surface border border-border text-secondary hover:text-white hover:bg-white/5'
                  }`}
              >
                {isSelectionMode ? <X size={16} /> : <CheckSquare size={16} />}
                <span>{isSelectionMode ? '取消选择' : '批量管理'}</span>
              </button>

              <div className="flex items-center gap-3 bg-surface p-2 rounded-xl border border-border">
                <GridIcon size={16} className="text-secondary" />
                <input
                  type="range"
                  min="2"
                  max="8"
                  value={columns}
                  onChange={(e) => setColumns(parseInt(e.target.value))}
                  className="w-32 accent-accent cursor-pointer"
                />
              </div>
            </div>
          </div>
        </header>

        {photos.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-64 border border-dashed border-border rounded-2xl bg-surface/50">
            <p className="text-secondary">此相册暂无照片。</p>
          </div>
        ) : (
          <div
            className={`gap-1 space-y-1 transition-all duration-300 ${isSelectionMode ? 'px-2' : ''}`}
            style={{
              columnCount: columns,
            }}
          >
            {sortedPhotos.map((photo, index) => (
              <div
                key={photo.id}
                className="relative group cursor-pointer break-inside-avoid overflow-hidden rounded-sm mb-1"
              >
                <ImageWithPlaceholder
                  src={photo.thumbnail_url || photo.url}
                  alt={photo.description || ''}
                  dominantColor={photo.dominant_color}
                  onClick={(e: any) => handleImageClick(index, photo.id, e)}
                  selectable={isSelectionMode}
                  selected={selectedIds.has(photo.id)}
                  className="w-full h-auto rounded-sm"
                />
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Floating Action Bar (Batch Operations) */}
      <div className={`fixed bottom-8 left-1/2 -translate-x-1/2 z-40 transition-all duration-500 cubic-bezier(0.175, 0.885, 0.32, 1.275) ${isSelectionMode && selectedIds.size > 0 ? 'translate-y-0 opacity-100' : 'translate-y-20 opacity-0 pointer-events-none'
        }`}>
        <div className="bg-[#18181b]/90 backdrop-blur-xl border border-white/10 rounded-full shadow-2xl px-6 py-3 flex items-center gap-6">
          <span className="text-sm font-medium text-white min-w-[3rem] text-center">
            已选 {selectedIds.size}
          </span>
          <div className="w-px h-6 bg-white/10" />

          <button
            onClick={() => setShowBatchRemoveConfirm(true)}
            className="flex flex-col items-center gap-1 group"
          >
            <div className="p-2 rounded-full bg-white/5 group-hover:bg-orange-500 group-hover:text-white text-secondary transition-colors">
              <FolderMinus size={18} />
            </div>
          </button>
        </div>
      </div>

      {/* Dialogs */}
      <ConfirmDialog
        isOpen={showDeleteAlbumConfirm}
        title="删除相册"
        message="确定要删除整个相册吗？相册内的照片将保留在您的图库中。"
        confirmText="删除相册"
        isDanger={true}
        loading={processing}
        onConfirm={handleDeleteAlbum}
        onCancel={() => setShowDeleteAlbumConfirm(false)}
      />

      <ConfirmDialog
        isOpen={showBatchRemoveConfirm}
        title={`移除 ${selectedIds.size} 张照片`}
        message="这些照片将从相册中移除，但仍会保留在您的图库中。"
        confirmText="移除"
        cancelText="取消"
        isDanger={false}
        loading={processing}
        onConfirm={handleBatchRemove}
        onCancel={() => setShowBatchRemoveConfirm(false)}
      />

      <PhotoModal
        photo={selectedPhoto}
        onClose={() => setSelectedPhotoIndex(null)}
        onDelete={handlePhotoDeleted}
        onRemoveFromAlbum={async (id) => {
          if (confirm("从相册移除？")) {
            await handleRemoveFromAlbumSingle(id);
          }
        }}
        onNext={() => setSelectedPhotoIndex(i => (i !== null && i < sortedPhotos.length - 1 ? i + 1 : i))}
        onPrev={() => setSelectedPhotoIndex(i => (i !== null && i > 0 ? i - 1 : i))}
        hasNext={selectedPhotoIndex !== null && selectedPhotoIndex < sortedPhotos.length - 1}
        hasPrev={selectedPhotoIndex !== null && selectedPhotoIndex > 0}
        originRect={originRect}
      />
    </>
  );
};