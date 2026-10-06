// API Response Wrappers
export interface ApiResponse<T> {
  code: number;
  data: T | null;
  msg: string | null;
}

// User Types
export interface UserInfo {
  id: number;
  username: string;
  email: string;
  role: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  user: UserInfo;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface RefreshTokenRequest {
  refresh_token: string;
}

export interface CreateUserRequest {
  username: string;
  email: string;
  password: string;
  role?: string;
}

export interface PeopleAlbumResult {
  id: number;
  name: string;
  face_count: number;
  cover_photo_id?: number | null;
}

export type TaskType = 'thumbnail' | 'clip' | 'face' | 'ocr';

export interface TaskProgress {
  task: TaskType;
  total: number;
  done: number;
  pending: number;
  state: 'running' | 'paused' | 'pending';
}

export interface ProcessingProgressResponse {
  tasks: TaskProgress[];
}

export interface LlavaConversationResponse {
  id: number;
  status: string;
  error?: string | null;
  history?: { role: string; content: string; created_at?: string }[] | null;
}

export interface CreateLlavaRequest {
  query: string;
  model?: string;
  temperature?: number;
  top_p?: number;
  num_beams?: number;
  max_new_tokens?: number;
}

// Album Types
export interface Album {
  id: number;
  user_id: number;
  name: string;
  description?: string | null;
  cover_photo_id?: number | null;
  cover_photo?: Photo | null; // Added cover_photo object
  created_at: string;
  updated_at: string;
}

export interface UpdateAlbumRequest {
  name?: string;
  description?: string;
  cover_photo_id?: number;
}

// Photo Types
export interface Photo {
  id: number;
  user_id: number;
  url: string;
  thumbnail_url?: string | null;
  processing_status: string; // 'pending', 'processed', etc.
  description?: string | null;
  width?: number | null;
  height?: number | null;
  file_size?: number | null;
  mime_type?: string | null;
  created_at?: string;
  updated_at?: string;

  // EXIF Data
  camera_make?: string | null;
  camera_model?: string | null;
  lens_model?: string | null;
  focal_length?: string | null;
  iso_speed?: number | null;
  f_number?: string | null;
  exposure_time?: string | null;
  date_time_original?: string | null;
  gps_latitude?: number | null;
  gps_longitude?: number | null;
  dominant_color?: string | null;

  // ML
  ml_result?: any;
}

export interface UpdatePhotoMetadataRequest {
  description?: string;
  // Add other fields if needed for editing
}

export interface SimilarPhotoResult {
  photo: Photo;
  distance: number;
}

// Search Types
export interface TextSearchStatusResponse {
  id: number;
  status: 'PENDING' | 'PROCESSING' | 'done' | 'FAILED' | 'COMPLETED'; // Added 'done'
  result?: SimilarPhotoResult[] | null;
  error?: string | null;
}