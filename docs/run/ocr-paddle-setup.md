# PaddleOCR 本地服务使用说明

## 启动

```bash
docker compose up -d ocr backend
```

## 检查 OCR 服务

```bash
curl -s -X POST http://127.0.0.1:8000/ocr \
  -F "file=@/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg"
```

返回字段：

- `text`: 拼接后的全文
- `lines`: 每行的识别结果（含 `text` 和 `score`）

## 后端联调（走完整链路）

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/upload \
  -F "file=@/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg"
```

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/<analysis_id>/analyze
```

```bash
curl -s http://127.0.0.1:3000/api/v1/analysis/<analysis_id>
```
