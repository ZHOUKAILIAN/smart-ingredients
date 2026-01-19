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

---

# Integration Test Log (2026-01-17, Run 3: OpenCV Preprocess)

## Environment

- Backend: `http://127.0.0.1:3000`
- DB: Docker container `smart-ingredients-db`
- Image: `/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg`
- OCR preprocess: enabled (`OCR_PREPROCESS_ENABLE=true`)

## 1) Health check

```bash
curl -s http://127.0.0.1:3000/health
```

---

# Integration Test Log (Remote Server Template)

## Environment

- Backend: `http://xxxxx:3000`
- Image (local): `/Users/zhoukailian/Desktop/mySelf/smart-ingredients/image.png`

## 1) Health check

```bash
curl -s http://xxxx:3000/health
```

Expected:

```
ok
```

## 2) Upload image (triggers OCR async)

```bash
curl -s -X POST http://xxxx:3000/api/v1/analysis/upload \
  -F "file=@/Users/zhoukailian/Desktop/mySelf/smart-ingredients/image.png"
```

Example response:

```json
{"id":"<analysis_id>","status":"ocr_pending","image_url":"/uploads/<file>.jpg"}
```

## 3) Poll OCR status

```bash
curl -s http://xxxx:3000/api/v1/analysis/<analysis_id>
```

Wait until:

- `status = ocr_completed` with `ocr_text` populated, or
- `status = ocr_failed` with `error_message`.

## 4) Confirm OCR and trigger LLM

```bash
curl -s -X POST http://xxxx:3000/api/v1/analysis/<analysis_id>/confirm \
  -H "Content-Type: application/json" \
  -d '{"text":"<ocr_text>"}'
```

Example response:

```json
{"id":"<analysis_id>","status":"llm_pending","ocr_text":"..."}
```

## 5) Poll LLM status

```bash
curl -s http://xxxxxxx:3000/api/v1/analysis/<analysis_id>
```

Wait until:

- `status = completed` with `result`, or
- `status = failed` with `error_message`.

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
{"id":"69ce2b4c-37e5-454a-9674-a257c7c6622b","status":"pending","image_url":"/uploads/572760c4-01db-4ded-9fa3-26ef87995d86.jpg"}
```

## 3) Trigger OCR + LLM

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/69ce2b4c-37e5-454a-9674-a257c7c6622b/analyze
```

Output:

```json
{"id":"69ce2b4c-37e5-454a-9674-a257c7c6622b","status":"completed","ocr_text":"oe Grae\npea                 \\\n(     i                、\n-一            . ee              ~\n-    | ~|        }   |\npe             ~~ %\nha      乌龙茶       _\noe      100% 茶叶自然抽出，\n”1   FOARALES™ RERROE Bak   |      一\n_     A)       营养成分表\na       |\n|      nf\nNae\n7      \\ fil\na |\na                 东方树叶乌龙茶原味茶饮料             1\n人                    青水 人和. 的人月 pores,                  ，\n1               、            82天-3和-由全    ij\noe                   Co             rer ebewacaarec aca | MIN\n     _   SAALRALeeLATD Pees fl\n.       .     AIDE aies emscue SCOSUAGIN ce\n.                             人 WL |\nBet Ealmaianedaiassmrages. ||\n10 Fe: |的RE市和有4本 B |\n全 es. acer fi\n|   DERHIE STE SG人\n|      fen a tra ¢ galginaltiaanremany fy\n|    flues pratsnanrgsiaseaiagt | || |\n.     FHPTESS: SCOURS Fe. 了放生\n |  《RPR了ET站让 test Bemtetis3, Ta  ~、\n    了    \"       ,\n|           Rea) |\n       I            fa         I |!             \\\n|            Www.nongfuspring.com           |             ll or i\nAt     SEAR HE:95077    中 |，  ange\nOd UNIT AEP as pte","result":{"health_score":85,"ingredients":[{"name":"乌龙茶","category":"nutrition","risk_level":"low","description":"天然茶叶提取物，含茶多酚等抗氧化成分"},{"name":"水","category":"nutrition","risk_level":"low","description":"饮料基础成分"}],"warnings":[],"recommendation":"这是一款成分简单的原味乌龙茶饮料，仅含茶叶和水，无糖、无添加剂、无防腐剂。适合日常饮用，但需注意：1) 含天然咖啡因，对咖啡因敏感者应适量；2) 茶多酚可能影响铁质吸收，建议与餐食间隔饮用；3) 包装信息显示部分文字识别不清，建议通过官方渠道核实完整配料表。"},"error_message":null,"created_at":"2026-01-17T04:58:11.051163+00:00"}
```

## 4) Query analysis result

```bash
curl -s http://127.0.0.1:3000/api/v1/analysis/69ce2b4c-37e5-454a-9674-a257c7c6622b
```

Output:

```json
{"id":"69ce2b4c-37e5-454a-9674-a257c7c6622b","status":"completed","ocr_text":"oe Grae\npea                 \\\n(     i                、\n-一            . ee              ~\n-    | ~|        }   |\npe             ~~ %\nha      乌龙茶       _\noe      100% 茶叶自然抽出，\n”1   FOARALES™ RERROE Bak   |      一\n_     A)       营养成分表\na       |\n|      nf\nNae\n7      \\ fil\na |\na                 东方树叶乌龙茶原味茶饮料             1\n人                    青水 人和. 的人月 pores,                  ，\n1               、            82天-3和-由全    ij\noe                   Co             rer ebewacaarec aca | MIN\n     _   SAALRALeeLATD Pees fl\n.       .     AIDE aies emscue SCOSUAGIN ce\n.                             人 WL |\nBet Ealmaianedaiassmrages. ||\n10 Fe: |的RE市和有4本 B |\n全 es. acer fi\n|   DERHIE STE SG人\n|      fen a tra ¢ galginaltiaanremany fy\n|    flues pratsnanrgsiaseaiagt | || |\n.     FHPTESS: SCOURS Fe. 了放生\n |  《RPR了ET站让 test Bemtetis3, Ta  ~、\n    了    \"       ,\n|           Rea) |\n       I            fa         I |!             \\\n|            Www.nongfuspring.com           |             ll or i\nAt     SEAR HE:95077    中 |，  ange\nOd UNIT AEP as pte","result":{"health_score":85,"ingredients":[{"name":"乌龙茶","category":"nutrition","risk_level":"low","description":"天然茶叶提取物，含茶多酚等抗氧化成分"},{"name":"水","category":"nutrition","risk_level":"low","description":"饮料基础成分"}],"warnings":[],"recommendation":"这是一款成分简单的原味乌龙茶饮料，仅含茶叶和水，无糖、无添加剂、无防腐剂。适合日常饮用，但需注意：1) 含天然咖啡因，对咖啡因敏感者应适量；2) 茶多酚可能影响铁质吸收，建议与餐食间隔饮用；3) 包装信息显示部分文字识别不清，建议通过官方渠道核实完整配料表。"},"error_message":null,"created_at":"2026-01-17T04:58:11.051163+00:00"}
```

## 5) Verify DB record

```bash
docker exec -i smart-ingredients-db psql -U smart_ingredients -d smart_ingredients \
  -c "select id, status, image_url, text, result, created_at, updated_at from analyses order by created_at desc limit 1;"
```

Output:

```
                  id                  |  status   |                     image_url                     |                                text                                |                                                                                                                                                                                                                                                                           result                                                                                                                                                                                                                                                                           |          created_at           |          updated_at
--------------------------------------+-----------+---------------------------------------------------+--------------------------------------------------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+-------------------------------+-------------------------------
 69ce2b4c-37e5-454a-9674-a257c7c6622b | completed | /uploads/572760c4-01db-4ded-9fa3-26ef87995d86.jpg | oe Grae                                                           +| {"warnings": [], "ingredients": [{"name": "乌龙茶", "category": "nutrition", "risk_level": "low", "description": "天然茶叶提取物，含茶多酚等抗氧化成分"}, {"name": "水", "category": "nutrition", "risk_level": "low", "description": "饮料基础成分"}], "health_score": 85, "recommendation": "这是一款成分简单的原味乌龙茶饮料，仅含茶叶和水，无糖、无添加剂、无防腐剂。适合日常饮用，但需注意：1) 含天然咖啡因，对咖啡因敏感者应适量；2) 茶多酚可能影响铁质吸收，建议与餐食间隔饮用；3) 包装信息显示部分文字识别不清，建议通过官方渠道核实完整配料表。"} | 2026-01-17 04:58:11.051163+00 | 2026-01-17 04:58:39.233044+00
                                      |           |                                                   | pea                 \                                             +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | (     i                、                                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | -一            . ee              ~                                +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | -    | ~|        }   |                                            +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | pe             ~~ %                                               +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | ha      乌龙茶       _                                            +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | oe      100% 茶叶自然抽出，                                       +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | ”1   FOARALES™ RERROE Bak   |      一                             +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | _     A)       营养成分表                                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | a       |                                                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | |      nf                                                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | Nae                                                               +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | 7      \ fil                                                      +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | a |                                                               +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | a                 东方树叶乌龙茶原味茶饮料             1          +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | 人                    青水 人和. 的人月 pores,                  ，+|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | 1               、            82天-3和-由全    ij                 +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | oe                   Co             rer ebewacaarec aca | MIN     +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   |      _   SAALRALeeLATD Pees fl                                    +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | .       .     AIDE aies emscue SCOSUAGIN ce                       +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | .                             人 WL |                             +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | Bet Ealmaianedaiassmrages. ||                                     +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | 10 Fe: |的RE市和有4本 B |                                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | 全 es. acer fi                                                    +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | |   DERHIE STE SG人                                               +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | |      fen a tra ¢ galginaltiaanremany fy                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | |    flues pratsnanrgsiaseaiagt | || |                            +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | .     FHPTESS: SCOURS Fe. 了放生                                  +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   |  |  《RPR了ET站让 test Bemtetis3, Ta  ~、                         +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   |     了    "       ,                                               +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | |           Rea) |                                                +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   |        I            fa         I |!             \                 +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | |            Www.nongfuspring.com           |             ll or i +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | At     SEAR HE:95077    中 |，  ange                              +|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
                                      |           |                                                   | Od UNIT AEP as pte                                                 |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                               |
(1 row)
```

## Notes

- OCR 预处理后能识别到 “乌龙茶” 与 “100% 茶叶自然抽出” 等信息，但整体仍较杂乱。
- LLM 根据 OCR 文本返回了更明确的配料建议。
