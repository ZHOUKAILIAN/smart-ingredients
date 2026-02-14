# AGENTS.md

## Workflow

- 设计/代码变更必须走 "小周调研 → 小周计划 → 小周执行 → 小周验证"；验证通过后再汇报。
- 简单需求可直接执行。
- Write requirements and technical plan first; get user confirmation before implementation.
- Branch names must be `feat/*` and open a PR after pushing.

## Execution Finish Checklist

After any implementation work completes:

- Start local services (docker compose) and confirm they are healthy.
- Run the full API flow end-to-end to ensure it works.
- Run `cargo check` for the frontend to ensure no compile errors.

## Notes

- Prioritize rules-based conclusions and explainability.
- Keep changes minimal and documented.
