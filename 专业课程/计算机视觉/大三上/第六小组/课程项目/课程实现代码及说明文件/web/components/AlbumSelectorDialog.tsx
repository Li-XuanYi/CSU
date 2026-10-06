import React, { useEffect, useState } from 'react';
import { Album } from '../types';
import { getAlbums } from '../services/api';
import { FolderPlus, X, Loader2 } from 'lucide-react';

interface AlbumSelectorDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSelect: (albumId: number) => void;
  loading?: boolean;
}

export const AlbumSelectorDialog: React.FC<AlbumSelectorDialogProps> = ({
  isOpen,
  onClose,
  onSelect,
  loading: parentLoading
}) => {
  const [albums, setAlbums] = useState<Album[]>([]);
  const [fetching, setFetching] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setFetching(true);
      getAlbums()
        .then(setAlbums)
        .catch(console.error)
        .finally(() => setFetching(false));
    }
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
      <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={onClose} />
      
      <div className="relative bg-[#18181b] border border-white/10 rounded-2xl shadow-2xl w-full max-w-md flex flex-col max-h-[80vh] animate-slide-up">
        <div className="p-5 border-b border-white/5 flex justify-between items-center">
          <h3 className="text-lg font-medium text-white flex items-center gap-2">
            <FolderPlus size={20} className="text-accent" />
            添加到相册
          </h3>
          <button onClick={onClose} className="p-1 text-white/50 hover:text-white rounded-full hover:bg-white/10 transition-colors">
            <X size={20} />
          </button>
        </div>

        <div className="flex-1 overflow-y-auto custom-scrollbar p-2">
          {fetching ? (
            <div className="flex justify-center py-8">
              <Loader2 className="animate-spin text-accent" size={24} />
            </div>
          ) : albums.length === 0 ? (
            <div className="text-center py-8 text-white/50 text-sm">暂无相册</div>
          ) : (
            <div className="space-y-1">
              {albums.map((album) => (
                <button
                  key={album.id}
                  onClick={() => onSelect(album.id)}
                  disabled={parentLoading}
                  className="w-full flex items-center gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors text-left group disabled:opacity-50"
                >
                  <div className="w-12 h-12 rounded-lg bg-white/5 flex items-center justify-center text-white/30 group-hover:text-accent group-hover:bg-accent/10 transition-colors">
                    <FolderPlus size={24} />
                  </div>
                  <div className="flex-1 min-w-0">
                    <h4 className="text-white font-medium truncate">{album.name}</h4>
                    <p className="text-xs text-white/40 truncate">{new Date(album.created_at).toLocaleDateString()}</p>
                  </div>
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};