import React, { useEffect, useState, useMemo, useRef } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { parseSearchQuery, SearchFilter } from '../utils/searchParser';
import { Photo, SimilarPhotoResult } from '../types';
import { getPhotos, deletePhoto, addPhotoToAlbum, initiateTextSearch, getTextSearchResult } from '../services/api';
import { PhotoModal } from '../components/PhotoModal';
import { ImageWithPlaceholder } from '../components/ImageWithPlaceholder';
import { ConfirmDialog } from '../components/ConfirmDialog';
import { AlbumSelectorDialog } from '../components/AlbumSelectorDialog';
import { Loader2, Grid as GridIcon, CheckSquare, Trash2, FolderPlus, X, Search as SearchIcon, ImageOff } from 'lucide-react';

export const Gallery: React.FC = () => {
  // Data State
  const [allPhotos, setAllPhotos] = useState<Photo[]>([]);
  const [searchResults, setSearchResults] = useState<Photo[] | null>(null); // null means not in result mode
  const [loading, setLoading] = useState(true);

  // Search State
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearching, setIsSearching] = useState(false);
  const searchInputRef = useRef<HTMLInputElement>(null);

  // View State
  const [selectedPhotoIndex, setSelectedPhotoIndex] = useState<number | null>(null);
  const [columns, setColumns] = useState(5);
  const [originRect, setOriginRect] = useState<DOMRect | null>(null);

  // Selection Mode State
  const [isSelectionMode, setIsSelectionMode] = useState(false);
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  // Dialog States
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [showAlbumSelector, setShowAlbumSelector] = useState(false);
  const [processing, setProcessing] = useState(false);

  // Search Filter State
  const [searchFilters, setSearchFilters] = useState<SearchFilter[]>([]);

  const { id: routePhotoId } = useParams<{ id: string }>();
  const navigate = useNavigate();

  useEffect(() => {
    const { filters } = parseSearchQuery(searchQuery);
    setSearchFilters(filters);
  }, [searchQuery]);

  useEffect(() => {
    loadPhotos();
  }, []);

  // Deep linking: Sync URL -> State
  useEffect(() => {
    if (routePhotoId && allPhotos.length > 0) {
      const id = parseInt(routePhotoId, 10);
      if (!isNaN(id)) {
        const index = allPhotos.findIndex(p => p.id === id);
        if (index !== -1) {
          setSelectedPhotoIndex(index);
        }
      }
    } else if (!routePhotoId && selectedPhotoIndex !== null) {
      // If URL has no ID but state has selection (back button pressed), clear selection
      setSelectedPhotoIndex(null);
    }
  }, [routePhotoId, allPhotos]);

  const loadPhotos = async () => {
    try {
      const data = await getPhotos();
      setAllPhotos(data);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    const query = searchQuery.trim();
    if (!query) {
      // If user tries to search empty string, clear search
      clearSearch();
      return;
    }

    setIsSearching(true);
    // CRITICAL CHANGE: Do NOT set searchResults to null here.
    // Keeping the old results (or all photos) visible prevents the "jitter" 
    // where the grid disappears and reappears.

    setSelectedIds(new Set()); // Clear selections
    setIsSelectionMode(false);

    try {
      // 1. Initiate Search
      const initRes = await initiateTextSearch(query);
      const taskId = initRes.id;

      // 2. Poll for results
      const pollInterval = setInterval(async () => {
        try {
          const statusRes = await getTextSearchResult(taskId);

          if (statusRes.status === 'done' && statusRes.result) {
            clearInterval(pollInterval);
            // Update the results only when ready
            setSearchResults(statusRes.result.map(r => r.photo));
            setIsSearching(false);
          } else if (statusRes.error) {
            clearInterval(pollInterval);
            setIsSearching(false);
            alert("搜索失败: " + statusRes.error);
          } else if (statusRes.status === 'FAILED') {
            clearInterval(pollInterval);
            setIsSearching(false);
            alert("搜索失败，请重试");
          }
        } catch (err) {
          console.error("Poll error", err);
        }
      }, 1000);

      // Safety timeout (30s)
      setTimeout(() => {
        clearInterval(pollInterval);
        if (isSearching) setIsSearching(false);
      }, 30000);

    } catch (e) {
      console.error(e);
      setIsSearching(false);
      alert("无法启动搜索");
    }
  };

  const clearSearch = () => {
    setSearchQuery('');
    setSearchResults(null); // Immediate switch back to All Photos
    setIsSearching(false);
  };

  // Logic to determine what to render
  // If searchResults is null, we show allPhotos.
  // If searchResults is NOT null, we show searchResults.
  // Exception: If isSearching is true AND searchResults is null (meaning first search from home), 
  // we effectively want to show a loading screen instead of "All Photos" to indicate work is starting,
  // OR we can keep showing All Photos with a spinner. 
  // The user requested "don't show complete list" when starting task.
  // Let's interpret "don't show complete list" as "don't flash the All Photos list if we are in a transition state".

  // Actually, the jitter usually comes from `searchResults` becoming null -> renders All Photos -> then renders Results.
  // By NOT clearing `searchResults` in `handleSearch`, we fixed the jitter between Search A -> Search B.

  // For All Photos -> Search A:
  // `searchResults` is null. `isSearching` becomes true.
  // We should probably show a "Searching..." placeholder instead of the All Photos grid 
  // if we want to distinguish the state clearly, OR overlay the spinner.

  const isFirstSearch = isSearching && searchResults === null;

  const currentPhotos = searchResults !== null ? searchResults : allPhotos;

  const sortedPhotos = useMemo(() => {
    // Only sort by date if it's the main gallery (allPhotos). 
    // If it's search results, keep the relevance order returned by the API.
    if (searchResults !== null) return currentPhotos;

    return [...currentPhotos].sort((a, b) => {
      const dateA = new Date(a.date_time_original || a.created_at || 0).getTime();
      const dateB = new Date(b.date_time_original || b.created_at || 0).getTime();
      return dateB - dateA;
    });
  }, [currentPhotos, searchResults]);

  // Deep linking: Sync URL -> State (Single Source of Truth)
  useEffect(() => {
    if (loading) return;

    // If there is a route ID, we MUST try to select it.
    if (routePhotoId && allPhotos.length > 0) {
      const id = parseInt(routePhotoId, 10);
      if (!isNaN(id)) {
        const index = allPhotos.findIndex(p => p.id === id);
        if (index !== -1) {
          if (selectedPhotoIndex !== index) {
            setSelectedPhotoIndex(index);
          }
        } else {
          // ID not found in current list (maybe filtered?)
          // We could redirect to /photos or just do nothing.
          // Let's do nothing to avoid loop if the list is just filtered.
        }
      }
    } else if (!routePhotoId && selectedPhotoIndex !== null) {
      // If URL is clean but we have a selection, it means user hit Back button or we need to close.
      setSelectedPhotoIndex(null);
    }
  }, [routePhotoId, allPhotos, loading]); // Removed selectedPhotoIndex from deps to avoid loop? No, we need it to detect mismatch.

  // --- Handlers ---

  const handlePhotoDeleted = (id: number) => {
    setAllPhotos(prev => prev.filter(p => p.id !== id));
    if (searchResults) {
      setSearchResults(prev => prev ? prev.filter(p => p.id !== id) : null);
    }
    // Navigate to close modal
    navigate('/photos', { replace: true });
  };

  const toggleSelectionMode = () => {
    if (isSelectionMode) {
      setIsSelectionMode(false);
      setSelectedIds(new Set());
    } else {
      setIsSelectionMode(true);
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
      // Navigate instead of setting state directly
      navigate(`/photos/${id}`);
    }
  };

  const handlePhotoSelect = (photo: Photo) => {
    navigate(`/photos/${photo.id}`);
  };

  const handleBatchDelete = async () => {
    setProcessing(true);
    try {
      await Promise.all(Array.from(selectedIds).map((id: number) => deletePhoto(id)));
      setAllPhotos(prev => prev.filter(p => !selectedIds.has(p.id)));
      if (searchResults) {
        setSearchResults(prev => prev ? prev.filter(p => !selectedIds.has(p.id)) : null);
      }
      setIsSelectionMode(false);
      setSelectedIds(new Set());
      setShowDeleteConfirm(false);
    } catch (e) {
      alert("部分照片删除失败");
    } finally {
      setProcessing(false);
    }
  };

  const handleBatchAddToAlbum = async (albumId: number) => {
    setProcessing(true);
    try {
      await Promise.all(Array.from(selectedIds).map((id: number) => addPhotoToAlbum(albumId, id)));
      setShowAlbumSelector(false);
      setIsSelectionMode(false);
      setSelectedIds(new Set());
    } catch (e) {
      alert("部分照片添加失败");
    } finally {
      setProcessing(false);
    }
  };

  if (loading && !isSearching) {
    return (
      <div className="flex h-[50vh] items-center justify-center">
        <Loader2 className="animate-spin text-accent" size={32} />
      </div>
    );
  }

  const selectedPhoto = selectedPhotoIndex !== null ? sortedPhotos[selectedPhotoIndex] : null;

  return (
    <>
      <div className="flex flex-col gap-4 animate-fade-in pb-20">
        {/* Sticky Header */}
        <header className="flex flex-col gap-4 sticky top-0 bg-background/80 backdrop-blur-xl z-30 py-4 -mx-8 px-8 border-b border-white/5 transition-all">
          <div className="flex flex-col md:flex-row justify-between items-end md:items-center gap-4">

            {/* Title / Search State */}
            <div className="flex-1 w-full md:w-auto">
              <h2 className="text-3xl font-light tracking-tight text-white transition-all flex items-center gap-3">
                {searchResults !== null ? '搜索结果' : '所有照片'}
              </h2>
              <p className="text-secondary mt-1 text-sm flex items-center gap-2 h-5">
                {isSearching ? (
                  <span className="flex items-center gap-2 text-accent animate-pulse font-medium">
                    <Loader2 size={12} className="animate-spin" />
                    {searchResults !== null ? '正在优化结果...' : `正在寻找 "${searchQuery}"...`}
                  </span>
                ) : (
                  <span>{sortedPhotos.length} 张{searchResults !== null ? '匹配照片' : '美好回忆'}</span>
                )}
              </p>
            </div>

            {/* Controls */}
            <div className="flex flex-col md:flex-row items-stretch md:items-center gap-3 w-full md:w-auto">



              {/* Search Bar */}
              <form onSubmit={handleSearch} className="relative group w-full md:w-64 lg:w-80">
                <div className="absolute inset-y-0 left-3 flex items-center pointer-events-none">
                  <SearchIcon size={16} className={`text-secondary transition-colors ${isSearching ? 'text-accent' : ''}`} />
                </div>
                <input
                  ref={searchInputRef}
                  type="text"
                  value={searchQuery}
                  onChange={(e) => {
                    setSearchQuery(e.target.value);
                    if (e.target.value === '') {
                      clearSearch();
                    }
                  }}
                  placeholder="搜索内容（如：今天上午的猫）"
                  className="w-full bg-surface border border-border focus:border-accent rounded-xl pl-10 pr-10 py-2 text-sm text-white placeholder-secondary/50 outline-none transition-all shadow-sm focus:shadow-md focus:shadow-accent/5"
                />

                {/* Active Filters Display */}
                {searchFilters.length > 0 && (
                  <div className="absolute top-full left-0 mt-2 flex flex-wrap gap-2 px-1">
                    {searchFilters.map((filter, idx) => (
                      <span key={idx} className={`text-[10px] px-2 py-0.5 rounded-full flex items-center gap-1 border ${filter.type === 'date'
                        ? 'bg-blue-500/10 text-blue-400 border-blue-500/20'
                        : 'bg-purple-500/10 text-purple-400 border-purple-500/20'
                        }`}>
                        {filter.label}
                      </span>
                    ))}
                  </div>
                )}

                {searchQuery && (
                  <button
                    type="button"
                    onClick={clearSearch}
                    className="absolute inset-y-0 right-2 flex items-center p-1 text-secondary hover:text-white transition-colors"
                  >
                    <X size={14} />
                  </button>
                )}
              </form>

              <div className="flex items-center gap-3">
                <button
                  onClick={toggleSelectionMode}
                  className={`flex-1 md:flex-none flex items-center justify-center gap-2 px-4 py-2 rounded-xl text-sm font-medium transition-all whitespace-nowrap ${isSelectionMode
                    ? 'bg-accent text-white shadow-lg shadow-accent/20'
                    : 'bg-surface border border-border text-secondary hover:text-white hover:bg-white/5'
                    }`}
                >
                  {isSelectionMode ? <X size={16} /> : <CheckSquare size={16} />}
                  <span>{isSelectionMode ? '取消' : '选择'}</span>
                </button>

                <div className="hidden md:flex items-center gap-3 bg-surface p-2 rounded-xl border border-border">
                  <GridIcon size={16} className="text-secondary" />
                  <input
                    type="range"
                    min="2"
                    max="8"
                    value={columns}
                    onChange={(e) => setColumns(parseInt(e.target.value))}
                    className="w-20 lg:w-32 accent-accent cursor-pointer"
                  />
                </div>
              </div>
            </div>
          </div>
        </header>

        {/* Content Area */}
        {isFirstSearch ? (
          /* Loading State for First Search (hides All Photos) */
          <div className="flex flex-col items-center justify-center h-[50vh] animate-fade-in">
            <Loader2 className="animate-spin text-accent mb-4" size={48} />
            <p className="text-white text-lg font-light">正在探索视觉内容...</p>
            <p className="text-secondary text-sm">AI 正在分析您的图库</p>
          </div>
        ) : sortedPhotos.length === 0 ? (
          /* Empty State */
          <div className="flex flex-col items-center justify-center h-64 border border-dashed border-border rounded-2xl bg-surface/50 mt-4 animate-fade-in">
            {searchResults !== null ? (
              <div className="text-center">
                <ImageOff size={48} className="text-secondary mx-auto mb-4 opacity-50" />
                <p className="text-white mb-1">未找到相关照片</p>
                <p className="text-secondary text-sm">尝试更换关键词，例如 "猫" 或 "蓝色天空"</p>
                <button onClick={clearSearch} className="mt-4 text-accent hover:underline text-sm">清除搜索</button>
              </div>
            ) : (
              <div className="text-center">
                <p className="text-secondary">暂无照片，快去上传一些吧。</p>
              </div>
            )}
          </div>
        ) : (
          /* Grid */
          <div
            className={`gap-1 space-y-1 transition-all duration-300 ${isSelectionMode ? 'px-2' : ''} ${isSearching ? 'opacity-50 grayscale-[0.5] scale-[0.99] origin-top' : 'opacity-100 scale-100'}`}
            style={{ columnCount: columns }}
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
                  className={`w-full h-auto rounded-sm`}
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
            onClick={() => setShowAlbumSelector(true)}
            className="flex flex-col items-center gap-1 group"
          >
            <div className="p-2 rounded-full bg-white/5 group-hover:bg-accent group-hover:text-white text-secondary transition-colors">
              <FolderPlus size={18} />
            </div>
          </button>

          <button
            onClick={() => setShowDeleteConfirm(true)}
            className="flex flex-col items-center gap-1 group"
          >
            <div className="p-2 rounded-full bg-white/5 group-hover:bg-red-500 group-hover:text-white text-secondary transition-colors">
              <Trash2 size={18} />
            </div>
          </button>
        </div>
      </div>

      {/* Dialogs */}
      <ConfirmDialog
        isOpen={showDeleteConfirm}
        title={`删除 ${selectedIds.size} 张照片`}
        message="这些照片将被永久删除，无法恢复。是否确认？"
        confirmText="彻底删除"
        isDanger={true}
        loading={processing}
        onConfirm={handleBatchDelete}
        onCancel={() => setShowDeleteConfirm(false)}
      />

      <AlbumSelectorDialog
        isOpen={showAlbumSelector}
        onClose={() => setShowAlbumSelector(false)}
        onSelect={handleBatchAddToAlbum}
        loading={processing}
      />

      <PhotoModal
        photo={selectedPhoto}
        onClose={() => navigate('/photos')}
        onDelete={handlePhotoDeleted}
        onNext={() => {
          if (selectedPhotoIndex !== null && selectedPhotoIndex < sortedPhotos.length - 1) {
            navigate(`/photos/${sortedPhotos[selectedPhotoIndex + 1].id}`);
          }
        }}
        onPrev={() => {
          if (selectedPhotoIndex !== null && selectedPhotoIndex > 0) {
            navigate(`/photos/${sortedPhotos[selectedPhotoIndex - 1].id}`);
          }
        }}
        onRemoveFromAlbum={undefined} // Not implementing remove from here for now
        onPhotoSelect={handlePhotoSelect}
        hasNext={selectedPhotoIndex !== null && selectedPhotoIndex < sortedPhotos.length - 1}
        hasPrev={selectedPhotoIndex !== null && selectedPhotoIndex > 0}
        originRect={originRect}
      />
    </>
  );
};