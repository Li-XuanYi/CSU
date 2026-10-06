import { AuthResponse, ApiResponse, Album, Photo, SimilarPhotoResult, UpdateAlbumRequest, UpdatePhotoMetadataRequest, TextSearchStatusResponse, CreateUserRequest, RefreshTokenRequest, PeopleAlbumResult, UserInfo, ProcessingProgressResponse, TaskType, LlavaConversationResponse, CreateLlavaRequest } from '../types';

const BASE_URL = '';

// Helper to fix relative URLs from backend
const normalizeUrl = (url?: string | null): string | undefined | null => {
  if (!url) return url;
  if (url.startsWith('http')) return url;
  if (url.startsWith('/')) return `${BASE_URL}${url}`;
  return url;
};

const transformPhoto = (p: Photo): Photo => ({
  ...p,
  url: normalizeUrl(p.url) as string,
  thumbnail_url: normalizeUrl(p.thumbnail_url),
});

const transformAlbum = (a: Album): Album => ({
  ...a,
  cover_photo: a.cover_photo ? transformPhoto(a.cover_photo) : null
});

// Centralized fetch wrapper to handle auth headers and 401s
const fetchApi = async (endpoint: string, options: RequestInit = {}) => {
  const token = localStorage.getItem('access_token');
  const headers = new Headers(options.headers || {});

  if (token) {
    headers.set('Authorization', `Bearer ${token}`);
  }

  // Auto-set JSON content type if not FormData and not already set
  if (!(options.body instanceof FormData) && !headers.has('Content-Type')) {
    headers.set('Content-Type', 'application/json');
  }

  const response = await fetch(`${BASE_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (response.status === 401) {
    // Trigger global event for App.tsx to handle logout
    window.dispatchEvent(new Event('auth:unauthorized'));
    throw new Error('Unauthorized');
  }

  return response;
};

// --- Auth ---

export const login = async (email: string, password: string): Promise<ApiResponse<AuthResponse>> => {
  const res = await fetch(`${BASE_URL}/api/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password }),
  });
  return res.json();
};

export const register = async (username: string, email: string, password: string): Promise<ApiResponse<AuthResponse>> => {
  const res = await fetch(`${BASE_URL}/api/auth/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, email, password }),
  });
  return res.json();
};

export const refreshToken = async (refresh_token: string): Promise<ApiResponse<AuthResponse>> => {
  const res = await fetch(`${BASE_URL}/api/auth/refresh`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ refresh_token }),
  });
  return res.json();
};

// --- Admin ---

export const adminCreateUser = async (data: CreateUserRequest): Promise<ApiResponse<UserInfo>> => {
  const res = await fetchApi('/api/admin/users', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to create user');
  return res.json();
};

// --- Photos ---

export const getPhotos = async (): Promise<Photo[]> => {
  const res = await fetchApi('/api/photos');
  if (!res.ok) throw new Error('Failed to fetch photos');
  const photos: Photo[] = await res.json();
  return photos.map(transformPhoto);
};

export const getPhoto = async (id: number): Promise<Photo> => {
  const res = await fetchApi(`/api/photos/${id}`);
  if (!res.ok) throw new Error('Failed to fetch photo');
  const photo: Photo = await res.json();
  return transformPhoto(photo);
};

export const updatePhotoMetadata = async (id: number, data: UpdatePhotoMetadataRequest): Promise<Photo> => {
  const res = await fetchApi(`/api/photos/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to update photo metadata');
  const photo: Photo = await res.json();
  return transformPhoto(photo);
};

export const uploadPhoto = async (file: File): Promise<Photo> => {
  const formData = new FormData();
  formData.append('file', file);

  const res = await fetchApi('/api/photos/upload', {
    method: 'POST',
    body: formData,
  });
  if (!res.ok) throw new Error('Failed to upload photo');
  const photo: Photo = await res.json();
  return transformPhoto(photo);
};

export const deletePhoto = async (id: number): Promise<void> => {
  const res = await fetchApi(`/api/photos/${id}`, {
    method: 'DELETE',
  });
  if (!res.ok) throw new Error('Failed to delete photo');
};

export const getSimilarPhotos = async (id: number): Promise<SimilarPhotoResult[]> => {
  const res = await fetchApi(`/api/photos/${id}/similar`);
  if (!res.ok) throw new Error('Failed to find similar photos');
  const results: SimilarPhotoResult[] = await res.json();
  return results.map(r => ({ ...r, photo: transformPhoto(r.photo) }));
};

export const autoPeopleAlbums = async (): Promise<PeopleAlbumResult[]> => {
  const res = await fetchApi('/api/photos/auto_people_albums', {
    method: 'POST',
  });
  if (!res.ok) throw new Error('Failed to auto create people albums');
  return res.json();
};



// --- Processing ---

export const getProcessingProgress = async (): Promise<ProcessingProgressResponse> => {
  const res = await fetchApi('/api/processing/progress');
  if (!res.ok) throw new Error('Failed to fetch processing progress');
  return res.json();
};

export const controlProcessingTask = async (task: TaskType, action: 'pause' | 'resume' | 'reset'): Promise<void> => {
  const res = await fetchApi('/api/processing/control', {
    method: 'POST',
    body: JSON.stringify({ task, action })
  });
  if (!res.ok) throw new Error('Failed to control processing task');
};

// --- LLaVA ---

export const createLlavaConversation = async (photoId: number, config: CreateLlavaRequest): Promise<LlavaConversationResponse> => {
  const res = await fetchApi(`/api/photos/${photoId}/llava`, {
    method: 'POST',
    body: JSON.stringify(config),
  });
  if (!res.ok) throw new Error('Failed to create LLaVA conversation');
  return res.json();
};

export const getLlavaConversation = async (photoId: number, conversationId: number): Promise<LlavaConversationResponse> => {
  const res = await fetchApi(`/api/photos/${photoId}/llava/${conversationId}`);
  if (!res.ok) throw new Error('Failed to get LLaVA conversation');
  return res.json();
};

export const continueLlavaConversation = async (photoId: number, conversationId: number, message: string, config?: Partial<CreateLlavaRequest>): Promise<LlavaConversationResponse> => {
  const body = {
    ...config,
    query: message
  };

  const res = await fetchApi(`/api/photos/${photoId}/llava/${conversationId}`, {
    method: 'POST',
    body: JSON.stringify(body),
  });
  if (!res.ok) throw new Error('Failed to continue LLaVA conversation');
  return res.json();
};

// --- Search ---

export const initiateTextSearch = async (text: string): Promise<TextSearchStatusResponse> => {
  const res = await fetchApi('/api/search/text', {
    method: 'POST',
    body: JSON.stringify({ query: text }), // Use 'query' as parameter key
  });
  if (!res.ok) throw new Error('Failed to initiate search');
  return res.json();
};

export const getTextSearchResult = async (id: number): Promise<TextSearchStatusResponse> => {
  const res = await fetchApi(`/api/search/text/${id}`);
  if (!res.ok) throw new Error('Failed to fetch search result');
  const data: TextSearchStatusResponse = await res.json();
  if (data.result) {
    data.result = data.result.map(r => ({ ...r, photo: transformPhoto(r.photo) }));
  }
  return data;
};

// --- Albums ---

export const getAlbums = async (): Promise<Album[]> => {
  const res = await fetchApi('/api/albums');
  if (!res.ok) throw new Error('Failed to fetch albums');
  const albums: Album[] = await res.json();
  return albums.map(transformAlbum);
};

export const createAlbum = async (name: string, description?: string): Promise<Album> => {
  const res = await fetchApi('/api/albums', {
    method: 'POST',
    body: JSON.stringify({ name, description }),
  });
  if (!res.ok) throw new Error('Failed to create album');
  const album: Album = await res.json();
  return transformAlbum(album);
};

export const updateAlbum = async (id: number, data: UpdateAlbumRequest): Promise<Album> => {
  const res = await fetchApi(`/api/albums/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to update album');
  const album: Album = await res.json();
  return transformAlbum(album);
};

export const deleteAlbum = async (id: number): Promise<void> => {
  const res = await fetchApi(`/api/albums/${id}`, {
    method: 'DELETE',
  });
  if (!res.ok) throw new Error('Failed to delete album');
};

export const getAlbumPhotos = async (albumId: number): Promise<Photo[]> => {
  const res = await fetchApi(`/api/photos/album/${albumId}`);
  if (!res.ok) throw new Error('Failed to fetch album photos');
  const photos: Photo[] = await res.json();
  return photos.map(transformPhoto);
};

export const addPhotoToAlbum = async (albumId: number, photoId: number): Promise<void> => {
  const res = await fetchApi(`/api/albums/${albumId}/photos/${photoId}`, {
    method: 'POST',
  });
  if (!res.ok) throw new Error('Failed to add photo to album');
};

export const removePhotoFromAlbum = async (albumId: number, photoId: number): Promise<void> => {
  const res = await fetchApi(`/api/albums/${albumId}/photos/${photoId}`, {
    method: 'DELETE',
  });
  if (!res.ok) throw new Error('Failed to remove photo from album');
};