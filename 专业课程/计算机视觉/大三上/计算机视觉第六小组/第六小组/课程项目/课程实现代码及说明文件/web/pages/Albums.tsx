import React, { useEffect, useState } from 'react';
import { Album } from '../types';
import { getAlbums, createAlbum, deleteAlbum, updateAlbum } from '../services/api';
import { Folder, Plus, Loader2, MoreVertical, Edit2, Trash2, X, Check, Image as ImageIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import { ConfirmDialog } from '../components/ConfirmDialog';

export const Albums: React.FC = () => {
  const [albums, setAlbums] = useState<Album[]>([]);
  const [loading, setLoading] = useState(true);
  
  // Create State
  const [isCreating, setIsCreating] = useState(false);
  const [newAlbumName, setNewAlbumName] = useState('');
  
  // Edit State
  const [editingAlbumId, setEditingAlbumId] = useState<number | null>(null);
  const [editName, setEditName] = useState('');
  
  // Menu State
  const [openMenuId, setOpenMenuId] = useState<number | null>(null);

  // Delete Dialog State
  const [albumToDelete, setAlbumToDelete] = useState<number | null>(null);
  const [deleting, setDeleting] = useState(false);

  useEffect(() => {
    loadAlbums();
  }, []);

  const loadAlbums = async () => {
    try {
      const data = await getAlbums();
      setAlbums(data);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newAlbumName.trim()) return;
    try {
      const newAlbum = await createAlbum(newAlbumName);
      setAlbums([newAlbum, ...albums]);
      setNewAlbumName('');
      setIsCreating(false);
    } catch (e) {
      alert("创建相册失败");
    }
  };

  const confirmDelete = async () => {
    if (albumToDelete === null) return;
    setDeleting(true);
    try {
        await deleteAlbum(albumToDelete);
        setAlbums(albums.filter(a => a.id !== albumToDelete));
        setAlbumToDelete(null);
    } catch(e) {
        alert("删除失败");
    } finally {
        setDeleting(false);
    }
  };

  const handleDeleteClick = (e: React.MouseEvent, id: number) => {
      e.preventDefault();
      e.stopPropagation();
      setAlbumToDelete(id);
      setOpenMenuId(null);
  };

  const startEdit = (e: React.MouseEvent, album: Album) => {
      e.preventDefault();
      e.stopPropagation();
      setEditingAlbumId(album.id);
      setEditName(album.name);
      setOpenMenuId(null);
  };

  const handleUpdate = async (e: React.MouseEvent, id: number) => {
      e.preventDefault();
      e.stopPropagation();
      if (!editName.trim()) return;
      try {
          const updated = await updateAlbum(id, { name: editName });
          setAlbums(albums.map(a => a.id === id ? updated : a));
          setEditingAlbumId(null);
      } catch(e) {
          alert("更新失败");
      }
  };

  const toggleMenu = (e: React.MouseEvent, id: number) => {
      e.preventDefault();
      e.stopPropagation();
      setOpenMenuId(openMenuId === id ? null : id);
  };

  useEffect(() => {
      const close = () => setOpenMenuId(null);
      window.addEventListener('click', close);
      return () => window.removeEventListener('click', close);
  }, []);

  if (loading) {
    return (
      <div className="flex h-[50vh] items-center justify-center">
        <Loader2 className="animate-spin text-accent" size={32} />
      </div>
    );
  }

  return (
    <div className="animate-fade-in">
       <header className="flex justify-between items-center mb-10">
          <div>
            <h2 className="text-3xl font-light tracking-tight text-white">相册集</h2>
            <p className="text-secondary mt-1 text-sm">管理您的精彩瞬间</p>
          </div>
          <button 
            onClick={() => setIsCreating(!isCreating)}
            className="flex items-center gap-2 bg-primary text-background px-4 py-2 rounded-lg font-medium hover:bg-white/90 transition-colors"
          >
            <Plus size={18} />
            <span>新建相册</span>
          </button>
        </header>

        {isCreating && (
          <form onSubmit={handleCreate} className="mb-8 p-6 bg-surface border border-border rounded-xl animate-slide-up max-w-md">
            <h3 className="text-lg font-medium mb-4">创建新集合</h3>
            <div className="flex gap-2">
              <input 
                type="text" 
                value={newAlbumName}
                onChange={(e) => setNewAlbumName(e.target.value)}
                placeholder="相册名称"
                className="flex-1 bg-black/20 border border-border rounded-lg px-4 py-2 text-white focus:outline-none focus:border-accent"
                autoFocus
              />
              <button type="submit" className="bg-accent text-white px-4 py-2 rounded-lg hover:bg-accent/90">
                创建
              </button>
            </div>
          </form>
        )}

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {albums.map((album) => (
            <div key={album.id} className="relative group block bg-surface border border-border rounded-xl overflow-hidden hover:border-accent/50 transition-all duration-300 hover:shadow-2xl hover:shadow-accent/5">
                <Link to={`/albums/${album.id}`} className="block h-full">
                    <div className="h-48 bg-black/40 flex items-center justify-center relative overflow-hidden">
                        {album.cover_photo ? (
                          <>
                            <img 
                              src={album.cover_photo.thumbnail_url || album.cover_photo.url} 
                              alt={album.name} 
                              className="absolute inset-0 w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
                            />
                            <div className="absolute inset-0 bg-black/20 group-hover:bg-black/10 transition-colors" />
                          </>
                        ) : (
                          <>
                            <div className="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent opacity-50" />
                            <Folder size={48} className="text-secondary opacity-20 group-hover:scale-110 transition-transform duration-500" />
                          </>
                        )}
                    </div>
                    
                    <div className="p-5">
                        <div className="flex justify-between items-center mb-1">
                            {editingAlbumId === album.id ? (
                                <div className="flex-1 flex gap-2" onClick={(e) => e.stopPropagation()}>
                                    <input 
                                        type="text" 
                                        value={editName}
                                        onChange={(e) => setEditName(e.target.value)}
                                        className="w-full bg-black/20 border border-border rounded px-2 py-1 text-sm focus:outline-none focus:border-accent"
                                        autoFocus
                                    />
                                    <button onClick={(e) => handleUpdate(e, album.id)} className="p-1 text-accent hover:bg-accent/10 rounded"><Check size={16} /></button>
                                    <button onClick={(e) => { e.stopPropagation(); setEditingAlbumId(null); }} className="p-1 text-secondary hover:bg-white/10 rounded"><X size={16} /></button>
                                </div>
                            ) : (
                                <>
                                    <h3 className="text-lg font-semibold text-primary truncate pr-8">{album.name}</h3>
                                    <button 
                                        onClick={(e) => toggleMenu(e, album.id)} 
                                        className={`absolute right-4 p-1 rounded-full hover:bg-white/10 text-secondary hover:text-white transition-colors z-20 ${openMenuId === album.id ? 'opacity-100 bg-white/10 text-white' : 'opacity-0 group-hover:opacity-100'}`}
                                    >
                                        <MoreVertical size={16} />
                                    </button>
                                </>
                            )}
                        </div>
                        <p className="text-sm text-secondary flex items-center gap-2">
                           <ImageIcon size={12} />
                           {album.cover_photo ? '已设封面' : '无封面'}
                        </p>
                    </div>
                </Link>

                {/* Context Menu */}
                {openMenuId === album.id && (
                    <div className="absolute right-4 bottom-16 w-32 bg-[#18181b] backdrop-blur-md border border-white/10 rounded-lg shadow-xl py-1 z-30 animate-fade-in">
                        <button onClick={(e) => startEdit(e, album)} className="w-full text-left px-4 py-2 text-sm text-white hover:bg-white/10 flex items-center gap-2">
                            <Edit2 size={14} /> 编辑
                        </button>
                        <button onClick={(e) => handleDeleteClick(e, album.id)} className="w-full text-left px-4 py-2 text-sm text-red-400 hover:bg-red-500/10 flex items-center gap-2">
                            <Trash2 size={14} /> 删除
                        </button>
                    </div>
                )}
            </div>
          ))}
        </div>

        <ConfirmDialog 
            isOpen={albumToDelete !== null}
            title="删除相册"
            message="确定要删除这个相册吗？照片将保留在您的图库中。"
            isDanger={true}
            confirmText="删除"
            loading={deleting}
            onConfirm={confirmDelete}
            onCancel={() => setAlbumToDelete(null)}
        />
    </div>
  );
};