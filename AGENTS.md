# AGENTS.md

## Workflow

- Follow "调研 → 计划 → 执行" for each iteration.
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
