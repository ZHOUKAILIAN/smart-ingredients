# Integration Test Log (2026-01-17)

## Environment

- Backend: `http://127.0.0.1:3000`
- DB: Docker container `smart-ingredients-db`
- Image: `/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg`

## 1) Health check

```bash
curl -s http://127.0.0.1:3000/health
```

Output:

```
ok
```

## 2) Upload image

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/upload \
  -F "file=@/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg"
```

Output:

```json
{"id":"ec986822-0c8f-4acd-8fbc-6865c6e00d1e","status":"pending","image_url":"/uploads/926ccc31-5b49-42f9-9264-c254326e51ab.jpg"}
```

## 3) Trigger OCR + LLM

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/ec986822-0c8f-4acd-8fbc-6865c6e00d1e/analyze
```

Output:

```json
{"code":"LLM_ERROR","message":"expected value at line 1 column 1","details":null,"request_id":null}
```

## 4) Query analysis result

```bash
curl -s http://127.0.0.1:3000/api/v1/analysis/ec986822-0c8f-4acd-8fbc-6865c6e00d1e
```

Output:

```json
{"id":"ec986822-0c8f-4acd-8fbc-6865c6e00d1e","status":"failed","ocr_text":"营养成分表\n\n项目 每100毫升Im山 营养素参考\n能量              (0干焦(fd)                    =\n\n0毫冯(mg)\n茶多酚含量 > 2Omgkg\n\nwww.nongfuspring.com\n\n’           从全国服务热线:95077\nRTT TTT","result":null,"error_message":"expected value at line 1 column 1","created_at":"2026-01-17T04:26:05.588262+00:00"}
```

## 5) Verify DB record

```bash
docker exec -i smart-ingredients-db psql -U smart_ingredients -d smart_ingredients \
  -c "select id, status, image_url, text, result, created_at, updated_at from analyses order by created_at desc limit 5;"
```

Output:

```
                  id                  | status |                     image_url                     |                       text                        | result |          created_at           |          updated_at
--------------------------------------+--------+---------------------------------------------------+---------------------------------------------------+--------+-------------------------------+-------------------------------
 ec986822-0c8f-4acd-8fbc-6865c6e00d1e | failed | /uploads/926ccc31-5b49-42f9-9264-c254326e51ab.jpg | 营养成分表                                       +|        | 2026-01-17 04:26:05.588262+00 | 2026-01-17 04:26:51.281883+00
                                      |        |                                                   |                                                  +|        |
                                      |        |                                                   | 项目 每100毫升Im山 营养素参考                    +|        |
                                      |        |                                                   | 能量              (0干焦(fd)                    =+|        |
                                      |        |                                                   |                                                  +|        |
                                      |        |                                                   | 0毫冯(mg)                                        +|        |
                                      |        |                                                   | 茶多酚含量 > 2Omgkg                              +|        |
                                      |        |                                                   |                                                  +|        |
                                      |        |                                                   | www.nongfuspring.com                             +|        |
                                      |        |                                                   |                                                  +|        |
                                      |        |                                                   | ’           从全国服务热线:95077                 +|        |
                                      |        |                                                   | RTT TTT                                           |        |
(1 row)
```

## Notes

- OCR successfully extracted text.
- LLM analysis failed with `LLM_ERROR` (`expected value at line 1 column 1`).

---

# Integration Test Log (2026-01-17, Run 2)

## Environment

- Backend: `http://127.0.0.1:3000`
- DB: Docker container `smart-ingredients-db`
- Image: `/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg`

## 1) Health check

```bash
curl -s http://127.0.0.1:3000/health
```

Output:

```
ok
```

## 2) Upload image

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/upload \
  -F "file=@/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg"
```

Output:

```json
{"id":"9a915474-f12b-4dca-9b38-c58ed4b14374","status":"pending","image_url":"/uploads/a56ab86f-7e45-4365-862d-763425c5b6c4.jpg"}
```

## 3) Trigger OCR + LLM

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/9a915474-f12b-4dca-9b38-c58ed4b14374/analyze
```

Output:

```json
{"id":"9a915474-f12b-4dca-9b38-c58ed4b14374","status":"completed","ocr_text":"营养成分表\n\n项目 每100毫升Im山 营养素参考\n能量              (0干焦(fd)                    =\n\n0毫冯(mg)\n茶多酚含量 > 2Omgkg\n\nwww.nongfuspring.com\n\n’           从全国服务热线:95077\nRTT TTT","result":{"health_score":85,"ingredients":[{"name":"茶多酚","category":"nutrition","risk_level":"low","description":"天然抗氧化剂，具有潜在的健康益处"}],"warnings":[],"recommendation":"该产品为茶饮料，配料表信息不完整，仅显示含有茶多酚（>20mg/kg）。从现有信息看，产品相对简单，但缺乏完整的营养成分和添加剂信息。建议：1) 联系厂家获取完整配料表；2) 若为无糖茶饮，可适量饮用；3) 注意查看是否有添加糖、防腐剂等未列明成分；4) 对茶多酚敏感者需谨慎饮用。"},"error_message":null,"created_at":"2026-01-17T04:36:34.522513+00:00"}
```

## 4) Query analysis result

```bash
curl -s http://127.0.0.1:3000/api/v1/analysis/9a915474-f12b-4dca-9b38-c58ed4b14374
```

Output:

```json
{"id":"9a915474-f12b-4dca-9b38-c58ed4b14374","status":"completed","ocr_text":"营养成分表\n\n项目 每100毫升Im山 营养素参考\n能量              (0干焦(fd)                    =\n\n0毫冯(mg)\n茶多酚含量 > 2Omgkg\n\nwww.nongfuspring.com\n\n’           从全国服务热线:95077\nRTT TTT","result":{"health_score":85,"ingredients":[{"name":"茶多酚","category":"nutrition","risk_level":"low","description":"天然抗氧化剂，具有潜在的健康益处"}],"warnings":[],"recommendation":"该产品为茶饮料，配料表信息不完整，仅显示含有茶多酚（>20mg/kg）。从现有信息看，产品相对简单，但缺乏完整的营养成分和添加剂信息。建议：1) 联系厂家获取完整配料表；2) 若为无糖茶饮，可适量饮用；3) 注意查看是否有添加糖、防腐剂等未列明成分；4) 对茶多酚敏感者需谨慎饮用。"},"error_message":null,"created_at":"2026-01-17T04:36:34.522513+00:00"}
```

## 5) Verify DB record

```bash
docker exec -i smart-ingredients-db psql -U smart_ingredients -d smart_ingredients \
  -c "select id, status, image_url, text, result, created_at, updated_at from analyses order by created_at desc limit 1;"
```

Output:

```
                  id                  |  status   |                     image_url                     |                       text                        |                                                                                                                                                                                                                                 result                                                                                                                                                                                                                                 |          created_at           |          updated_at
--------------------------------------+-----------+---------------------------------------------------+---------------------------------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+-------------------------------+-------------------------------
 9a915474-f12b-4dca-9b38-c58ed4b14374 | completed | /uploads/a56ab86f-7e45-4365-862d-763425c5b6c4.jpg | 营养成分表                                       +| {"warnings": [], "ingredients": [{"name": "茶多酚", "category": "nutrition", "risk_level": "low", "description": "天然抗氧化剂，具有潜在的健康益处"}], "health_score": 85, "recommendation": "该产品为茶饮料，配料表信息不完整，仅显示含有茶多酚（>20mg/kg）。从现有信息看，产品相对简单，但缺乏完整的营养成分和添加剂信息。建议：1) 联系厂家获取完整配料表；2) 若为无糖茶饮，可适量饮用；3) 注意查看是否有添加糖、防腐剂等未列明成分；4) 对茶多酚敏感者需谨慎饮用。"} | 2026-01-17 04:36:34.522513+00 | 2026-01-17 04:37:28.070091+00
                                      |           |                                                   |                                                  +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | 项目 每100毫升Im山 营养素参考                    +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | 能量              (0干焦(fd)                    =+|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   |                                                  +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | 0毫冯(mg)                                        +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | 茶多酚含量 > 2Omgkg                              +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   |                                                  +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | www.nongfuspring.com                             +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   |                                                  +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | ’           从全国服务热线:95077                 +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
                                      |           |                                                   | RTT TTT                                           |                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                               |
(1 row)
```

## Notes

- OCR text仍然不完整，未识别出配料行。
- LLM 解析成功，但基于 OCR 文本内容有限。
