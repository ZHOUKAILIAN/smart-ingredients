from typing import Any, List, Tuple


def parse_ocr_result(result: Any, min_text_len: int) -> Tuple[str, List[dict]]:
    lines: List[dict] = []
    texts: List[str] = []

    if not result:
        return "", lines

    for block in result:
        if not block:
            continue
        for item in block:
            if not item or len(item) < 2:
                continue
            text, score = item[1]
            if text:
                lines.append({"text": text, "score": score})
                texts.append(text)

    joined = "\n".join(texts).strip()
    if len(joined) < min_text_len:
        return "", lines

    return joined, lines


def build_ocr_response(
    result: Any,
    min_text_len: int,
    empty_message: str,
) -> Tuple[int, dict]:
    text, lines = parse_ocr_result(result, min_text_len)
    if not text:
        return 422, {"message": empty_message}
    return 200, {"text": text, "lines": lines}
