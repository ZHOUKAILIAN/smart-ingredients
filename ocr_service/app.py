import io
import os

import numpy as np
from fastapi import FastAPI, File, HTTPException, UploadFile
from paddleocr import PaddleOCR
from PIL import Image

app = FastAPI()


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

    result = OCR_ENGINE.ocr(np.array(image), cls=True)
    if not result:
        return {"text": "", "lines": []}

    lines = []
    texts = []
    for block in result:
        if not block:
            continue
        for item in block:
            text, score = item[1]
            if text:
                lines.append({"text": text, "score": score})
                texts.append(text)

    return {"text": "\n".join(texts).strip(), "lines": lines}
