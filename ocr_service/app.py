import io
import os

import numpy as np
from fastapi import FastAPI, File, HTTPException, UploadFile
from paddleocr import PaddleOCR
from PIL import Image, ImageOps

app = FastAPI()


def get_env_bool(name: str, default: bool) -> bool:
    raw = os.getenv(name)
    if raw is None:
        return default
    return raw.strip().lower() in {"1", "true", "yes", "on"}


def get_env_int(name: str, default: int) -> int:
    raw = os.getenv(name)
    if raw is None:
        return default
    try:
        return int(raw)
    except ValueError:
        return default


def get_env_float(name: str, default: float) -> float:
    raw = os.getenv(name)
    if raw is None:
        return default
    try:
        return float(raw)
    except ValueError:
        return default


def preprocess_image(image: Image.Image, threshold: int, scale: float) -> Image.Image:
    gray = ImageOps.grayscale(image)
    gray = ImageOps.autocontrast(gray)
    if scale != 1.0:
        gray = gray.resize(
            (int(gray.width * scale), int(gray.height * scale)), Image.BICUBIC
        )
    binary = gray.point(lambda x: 255 if x > threshold else 0, mode="1")
    return binary.convert("RGB")


def extract_text(result) -> tuple[str, list[dict]]:
    if not result:
        return "", []

    lines: list[dict] = []
    texts: list[str] = []
    for block in result:
        if not block:
            continue
        for item in block:
            text, score = item[1]
            if text:
                lines.append({"text": text, "score": score})
                texts.append(text)

    return "\n".join(texts).strip(), lines


def build_ocr() -> PaddleOCR:
    lang = os.getenv("PADDLE_OCR_LANG", "ch")
    use_angle_cls = os.getenv("PADDLE_OCR_ANGLE", "true").lower() in {
        "1",
        "true",
        "yes",
        "on",
    }
    return PaddleOCR(use_angle_cls=use_angle_cls, lang=lang)


OCR_ENGINE = build_ocr()


@app.post("/ocr")
async def ocr_endpoint(file: UploadFile = File(...)) -> dict:
    if not file.content_type or not file.content_type.startswith("image/"):
        raise HTTPException(status_code=400, detail="file must be an image")

    data = await file.read()
    if not data:
        raise HTTPException(status_code=400, detail="empty file")

    try:
        image = Image.open(io.BytesIO(data)).convert("RGB")
    except Exception as exc:
        raise HTTPException(status_code=400, detail=f"invalid image: {exc}") from exc

    retry_max = max(0, get_env_int("OCR_RETRY_MAX", 1))
    min_text_len = max(1, get_env_int("OCR_MIN_TEXT_LEN", 2))
    preprocess_enabled = get_env_bool("OCR_PREPROCESS_ENABLED", True)
    threshold = max(0, min(255, get_env_int("OCR_PREPROCESS_THRESHOLD", 180)))
    scale = max(1.0, get_env_float("OCR_PREPROCESS_SCALE", 1.5))

    result = OCR_ENGINE.ocr(np.array(image), cls=True)
    text, lines = extract_text(result)
    retry_count = 0

    while (
        preprocess_enabled
        and retry_count < retry_max
        and len(text) < min_text_len
    ):
        retry_count += 1
        processed = preprocess_image(image, threshold, scale)
        result = OCR_ENGINE.ocr(np.array(processed), cls=True)
        text, lines = extract_text(result)

    return {"text": text, "lines": lines, "retry_count": retry_count}
