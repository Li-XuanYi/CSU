#!/usr/bin/env python3
"""
自动生成人物相册：
- 从 API 拉取当前用户的照片列表
- 本地读取图片，使用 insightface 提取人脸向量
- 按余弦距离做简易聚类，输出“人物相册”，并调用 /api/albums 接口创建相册并加入对应照片

环境变量（可放 .env）：
- API_BASE_URL: 服务器地址，默认 http://127.0.0.1:5800
- ACCESS_TOKEN: 用户 Bearer Token（必填，用于访问 /api/photos /api/albums）
- PROJECT_ROOT 或 UPLOAD_BASE_DIR: 本地上传目录前缀，默认取仓库根目录（ml 的上一级）
- FACE_DISTANCE_THRESHOLD: 人脸聚类阈值，默认 0.42（向量已归一化，越小越严格）
- MIN_FACES_PER_PERSON: 聚类最小人脸数，默认 2，低于阈值的不建相册
- MAX_PHOTOS: 可选，限制处理的照片数量（按最新优先）
"""

import logging
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional

import numpy as np
import requests

from core import FaceEmbedder, resolve_base_dir

import cv2


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")


@dataclass
class PhotoItem:
    id: int
    url: str
    path: Optional[Path]


class PeopleAlbum:
    def __init__(self) -> None:
        self.api_base = os.getenv("API_BASE_URL", "http://127.0.0.1:5800")
        self.token = os.getenv("ACCESS_TOKEN")
        if not self.token:
            raise SystemExit("环境变量 ACCESS_TOKEN 为空，请先登录并填入 JWT")

        self.base_dir = resolve_base_dir()
        self.threshold = float(os.getenv("FACE_DISTANCE_THRESHOLD", "0.42"))
        self.min_faces = int(os.getenv("MIN_FACES_PER_PERSON", "2"))
        self.max_photos = int(os.getenv("MAX_PHOTOS", "0"))
        self.face_model_name = os.getenv("FACE_MODEL_NAME", "buffalo_l")

        self.face_embedder = FaceEmbedder(self.face_model_name)

    # ---------------------- API helpers ----------------------
    def _auth_headers(self) -> Dict[str, str]:
        return {"Authorization": f"Bearer {self.token}"}

    def fetch_photos(self) -> List[PhotoItem]:
        url = f"{self.api_base}/api/photos"
        resp = requests.get(url, headers=self._auth_headers(), timeout=30)
        if resp.status_code >= 300:
            raise SystemExit(f"拉取照片失败: {resp.status_code} {resp.text}")
        data = resp.json()
        photos = data.get("data") if isinstance(data, dict) and "data" in data else data
        if not isinstance(photos, list):
            raise SystemExit("响应格式不符合预期，需返回数组")

        items: List[PhotoItem] = []
        for p in photos:
            pid = p.get("id")
            url = p.get("url") or ""
            path = self._resolve_path(url)
            items.append(PhotoItem(id=pid, url=url, path=path))

        # 按 id 倒序，优先处理新照片
        items.sort(key=lambda x: x.id, reverse=True)
        if self.max_photos > 0:
            items = items[: self.max_photos]
        return items

    def create_album(self, name: str, description: str, photo_ids: List[int]) -> None:
        url = f"{self.api_base}/api/albums"
        payload = {"name": name, "description": description, "photo_ids": photo_ids}
        resp = requests.post(url, json=payload, headers=self._auth_headers(), timeout=30)
        if resp.status_code >= 300:
            raise RuntimeError(f"创建相册失败: {resp.status_code} {resp.text}")
        logging.info("创建相册成功: %s, 照片数=%s", name, len(photo_ids))

    # ---------------------- Core logic ----------------------
    def run(self) -> None:
        photos = self.fetch_photos()
        logging.info("待处理照片数量: %s", len(photos))

        faces = self._extract_faces(photos)
        logging.info("检测到人脸数量: %s", len(faces))

        clusters = self._cluster_faces(faces)
        if not clusters:
            logging.info("未形成有效聚类，跳过创建相册")
            return

        for idx, cluster in enumerate(clusters, 1):
            if len(cluster["faces"]) < self.min_faces:
                continue
            photo_ids = list({f["photo_id"] for f in cluster["faces"]})
            name = f"人物簇#{idx}"
            desc = f"自动聚类生成，含 {len(cluster['faces'])} 张人脸，阈值 {self.threshold:.2f}"
            self.create_album(name, desc, photo_ids)

    def _extract_faces(self, photos: List[PhotoItem]) -> List[Dict]:
        results: List[Dict] = []
        for p in photos:
            if not p.path or not p.path.exists():
                logging.warning("文件不存在，跳过 photo_id=%s, path=%s", p.id, p.path)
                continue

            img = cv2.imread(str(p.path))
            if img is None:
                logging.warning("读取图片失败，跳过 photo_id=%s, path=%s", p.id, p.path)
                continue

            faces = self.face_embedder.extract_faces(img)
            if not faces:
                continue

            for face in faces:
                emb = face["embedding"]
                if not emb:
                    continue
                vec = self._normalize(np.array(emb, dtype=np.float32))
                results.append({"photo_id": p.id, "vec": vec})
        return results

    def _cluster_faces(self, faces: List[Dict]) -> List[Dict]:
        clusters: List[Dict] = []
        for face in faces:
            vec = face["vec"]
            placed = False
            for cluster in clusters:
                center = cluster["center"]
                dist = 1 - float(np.dot(center, vec))  # 余弦距离
                if dist <= self.threshold:
                    cluster["faces"].append(face)
                    cluster["center"] = self._normalize(center + vec)
                    placed = True
                    break
            if not placed:
                clusters.append({"center": vec, "faces": [face]})
        logging.info("形成聚类数: %s", len(clusters))
        return clusters

    # ---------------------- Utils ----------------------
    def _normalize(self, v: np.ndarray) -> np.ndarray:
        v = np.array(v, dtype=np.float32)
        norm = np.linalg.norm(v)
        return v / norm if norm > 0 else v

    def _resolve_path(self, url: str) -> Optional[Path]:
        if not url:
            return None
        # 若已经是绝对路径
        candidate = Path(url)
        if candidate.is_absolute() and candidate.exists():
            return candidate

        # 去掉前导 /
        joined = self.base_dir / url.lstrip("/")
        if joined.exists():
            return joined

        return joined  # 返回推断路径，便于日志提示

