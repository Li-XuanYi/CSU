#!/usr/bin/env python3
"""
Master entrypoint for ML runners/services.
"""

from __future__ import annotations

import argparse
import os
import threading


def run_clip() -> None:
    from clip_runner import ClipRunner

    ClipRunner().run_forever()


def run_text() -> None:
    from text_runner import TextRunner

    TextRunner().run_forever()


def run_embed_service(host: str, port: int) -> None:
    import uvicorn

    uvicorn.run("embed_service:app", host=host, port=port)


def run_all() -> None:
    os.environ.setdefault("OCR_ENABLED", "1")
    clip_thread = threading.Thread(target=run_clip, name="clip_runner", daemon=True)
    text_thread = threading.Thread(target=run_text, name="text_runner", daemon=True)
    clip_thread.start()
    text_thread.start()
    try:
        while clip_thread.is_alive() or text_thread.is_alive():
            clip_thread.join(timeout=1)
            text_thread.join(timeout=1)
    except KeyboardInterrupt:
        return


def run_text_search(query: str, limit: int) -> None:
    from text_search import TextSearch

    app = TextSearch()
    app.search(query, limit)


def run_people_album() -> None:
    from people_album import PeopleAlbum

    PeopleAlbum().run()

def run_llava() -> None:
    from llava_runner import LlavaRunner

    LlavaRunner().run_forever()


def main() -> None:
    parser = argparse.ArgumentParser(description="ML master runner")
    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser("clip", help="Consume photo_tasks and write embeddings/ML results")
    sub.add_parser("text", help="Consume text_tasks for async text embedding")

    embed = sub.add_parser("embed-service", help="Run FastAPI text embedding service")
    embed.add_argument("--host", default="0.0.0.0")
    embed.add_argument("--port", type=int, default=8001)

    sub.add_parser("all", help="Run clip + text runners in one process")
    search = sub.add_parser("text-search", help="Run text-to-image search CLI")
    search.add_argument("query", help="Text query")
    search.add_argument("--limit", type=int, default=20, help="Result size")

    sub.add_parser("people-album", help="Auto-create people albums from face clusters")
    sub.add_parser("llava", help="Consume llava_tasks for image chat")

    args = parser.parse_args()
    if args.command == "clip":
        run_clip()
    elif args.command == "text":
        run_text()
    elif args.command == "embed-service":
        run_embed_service(args.host, args.port)
    elif args.command == "all":
        run_all()
    elif args.command == "text-search":
        run_text_search(args.query, args.limit)
    elif args.command == "people-album":
        run_people_album()
    elif args.command == "llava":
        run_llava()


if __name__ == "__main__":
    main()
