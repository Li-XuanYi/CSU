import React, { useState } from 'react';
import { Check } from 'lucide-react';

interface ImageWithPlaceholderProps {
  src: string;
  alt: string;
  dominantColor?: string | null;
  className?: string;
  onClick?: (event: React.MouseEvent<HTMLDivElement>) => void;
  style?: React.CSSProperties;
  selectable?: boolean;
  selected?: boolean;
}

export const ImageWithPlaceholder: React.FC<ImageWithPlaceholderProps> = ({
  src,
  alt,
  dominantColor,
  className = '',
  onClick,
  style,
  selectable = false,
  selected = false,
}) => {
  const [loaded, setLoaded] = useState(false);

  return (
    <div 
      className={`relative overflow-hidden bg-surface group ${className}`} 
      onClick={onClick}
      style={{ 
        backgroundColor: dominantColor || '#18181b', 
        ...style 
      }}
    >
      <img
        src={src}
        alt={alt}
        loading="lazy"
        onLoad={() => setLoaded(true)}
        className={`w-full h-full object-cover transition-all duration-500 ease-out will-change-transform 
          ${loaded ? 'opacity-100 blur-0' : 'opacity-0 blur-xl scale-110'}
          ${selectable && selected ? 'scale-90' : 'scale-100 group-hover:scale-105'}
        `}
      />

      {/* Selection Overlay */}
      {selectable && (
        <div className={`absolute inset-0 transition-colors duration-200 ${selected ? 'bg-black/40' : 'bg-transparent hover:bg-black/10'}`}>
          <div className={`absolute top-3 right-3 w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all duration-200 
            ${selected 
              ? 'bg-accent border-accent scale-100 shadow-lg' 
              : 'border-white/50 bg-black/20 scale-90 opacity-0 group-hover:opacity-100 hover:border-white hover:bg-black/40'
            }`}
          >
            {selected && <Check size={14} className="text-white stroke-[3px]" />}
          </div>
        </div>
      )}
    </div>
  );
};