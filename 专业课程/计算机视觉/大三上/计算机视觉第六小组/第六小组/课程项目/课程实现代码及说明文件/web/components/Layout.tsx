import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import {
  LayoutGrid,
  Image as ImageIcon,
  LogOut,
  PlusCircle,
  Aperture,
  Activity
} from 'lucide-react';

interface LayoutProps {
  children: React.ReactNode;
  onLogout: () => void;
}

const NavItem = ({ to, icon: Icon, label, active }: { to: string; icon: any; label: string; active: boolean }) => (
  <Link
    to={to}
    className={`flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200 group ${active
      ? 'bg-surface text-primary shadow-inner shadow-black/50 border border-border'
      : 'text-secondary hover:text-primary hover:bg-surface/50'
      }`}
  >
    <Icon size={20} className={`${active ? 'text-accent' : 'text-current group-hover:text-primary'}`} />
    <span className="font-medium text-sm tracking-wide">{label}</span>
  </Link>
);

export const Layout: React.FC<LayoutProps> = ({ children, onLogout }) => {
  const location = useLocation();

  return (
    <div className="flex h-screen w-full overflow-hidden bg-background text-primary selection:bg-accent selection:text-white">
      {/* Sidebar */}
      <aside className="w-64 flex-shrink-0 flex flex-col border-r border-border bg-background/50 backdrop-blur-xl z-20">
        <div className="p-8 flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-accent to-purple-600 flex items-center justify-center shadow-lg shadow-accent/20">
            <Aperture className="text-white" size={20} />
          </div>
          <h1 className="text-xl font-bold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-white to-gray-400">
            Mindsight
          </h1>
        </div>

        <nav className="flex-1 px-4 space-y-2">
          <NavItem
            to="/"
            icon={ImageIcon}
            label="照片"
            active={location.pathname === '/' || location.pathname.startsWith('/photos')}
          />
          <NavItem
            to="/albums"
            icon={LayoutGrid}
            label="相册"
            active={location.pathname.startsWith('/albums')}
          />
          <NavItem
            to="/tasks"
            icon={Activity}
            label="后台任务"
            active={location.pathname === '/tasks'}
          />
          <NavItem
            to="/upload"
            icon={PlusCircle}
            label="上传"
            active={location.pathname === '/upload'}
          />
        </nav>

        <div className="p-4 border-t border-border">
          <button
            onClick={onLogout}
            className="flex w-full items-center gap-3 px-4 py-3 text-secondary hover:text-red-400 hover:bg-red-500/10 rounded-xl transition-all duration-200"
          >
            <LogOut size={20} />
            <span className="font-medium text-sm">退出登录</span>
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-y-auto relative scroll-smooth">
        <div className="max-w-7xl mx-auto p-8 lg:p-12 pb-24">
          {children}
        </div>
      </main>
    </div>
  );
};