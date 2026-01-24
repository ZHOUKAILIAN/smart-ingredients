# Backend Integration Test (Server API)

## Environment

- Backend: `http://127.0.0.1:3000`
- DB: Docker container `smart-ingredients-db`
- Image: `/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg`

## 1) Health check

```bash
curl -s http://127.0.0.1:3000/health
```

Expected:

```
ok
```

## 2) Upload image (triggers OCR async)

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/upload \
  -F "file=@/Users/zhoukailian/Desktop/mySelf/smart-ingredients/1241768623551_.pic.jpg"
```

Output:

```json
{"id":"902110fa-a109-48d9-a13c-bd4d8513e63d","status":"ocr_pending","image_url":"/uploads/1241768623551_.pic_1769241597_0b22a743-c0f0-41e5-8178-35cced0aeb55.jpg"}
```

## 3) Query analysis (wait OCR)

```bash
curl -s http://127.0.0.1:3000/api/v1/analysis/902110fa-a109-48d9-a13c-bd4d8513e63d
```

Output (example):

```json
{"id":"902110fa-a109-48d9-a13c-bd4d8513e63d","status":"ocr_completed","ocr_status":"completed","llm_status":"pending","ocr_text":"...","result":null,"error_message":null}
```

## 4) Confirm OCR text (trigger LLM)

```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/902110fa-a109-48d9-a13c-bd4d8513e63d/confirm \
  -H "Content-Type: application/json" \
  -d '{"confirmed_text":"<填入上一步返回的 ocr_text>"}'
```

Output:

```json
{"id":"902110fa-a109-48d9-a13c-bd4d8513e63d","status":"llm_pending","llm_status":"pending","result":null}
```

## 5) Query analysis result (LLM completed)

```bash
curl -s http://127.0.0.1:3000/api/v1/analysis/902110fa-a109-48d9-a13c-bd4d8513e63d
```

Output (example):

```json
{"id":"902110fa-a109-48d9-a13c-bd4d8513e63d","status":"completed","llm_status":"completed","result":{"health_score":85,"summary":"...","ingredients":[...],"warnings":[...],"recommendation":"..."}}
```

## 6) Verify DB record

```bash
docker exec -i smart-ingredients-db psql -U smart_ingredients -d smart_ingredients \
  -c "select id, status, image_url, text, result, created_at, updated_at from analyses order by created_at desc limit 1;"
```

Expected: `status = completed` and `result` not null.

## Notes

- 新流程不再使用 `/analyze` 接口。
- OCR 完成后需通过 `/confirm` 才会触发 LLM。

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
