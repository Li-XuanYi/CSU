import React, { useState, useEffect } from 'react';
import { HashRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './components/Layout';
import { Gallery } from './pages/Gallery';
import { Albums } from './pages/Albums';
import { AlbumView } from './pages/AlbumView';
import { Upload } from './pages/Upload';
import { Login } from './pages/Login';
import { Tasks } from './pages/Tasks';

const App: React.FC = () => {
  const [token, setToken] = useState<string | null>(localStorage.getItem('access_token'));

  const handleLogin = (newToken: string) => {
    localStorage.setItem('access_token', newToken);
    setToken(newToken);
  };

  const handleLogout = () => {
    localStorage.removeItem('access_token');
    setToken(null);
  };

  useEffect(() => {
    // Listen for 401 events from api.ts
    const handleUnauthorized = () => {
      handleLogout();
    };
    window.addEventListener('auth:unauthorized', handleUnauthorized);
    return () => window.removeEventListener('auth:unauthorized', handleUnauthorized);
  }, []);

  if (!token) {
    return <Login onLoginSuccess={handleLogin} />;
  }

  return (
    <Router>
      <Layout onLogout={handleLogout}>
        <Routes>
          <Route path="/" element={<Gallery />} />
          <Route path="/photos" element={<Gallery />} />
          <Route path="/photos/:id" element={<Gallery />} />
          <Route path="/albums" element={<Albums />} />
          <Route path="/albums/:id" element={<AlbumView />} />
          <Route path="/upload" element={<Upload />} />
          <Route path="/tasks" element={<Tasks />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </Layout>
    </Router>
  );
};

export default App;