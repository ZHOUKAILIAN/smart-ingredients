# UI Guidelines Fixes Requirements

## Background
UI style was unified to the Figma redesign, but a Web Interface Guidelines review surfaced a small set of compliance gaps (focus visibility, hover feedback, long text handling, numeric alignment). The user asked for a minimal fix that keeps the current Figma style and layout intact.

## Goals
- Fix all guideline findings with minimal visual impact.
- Keep the current Figma-based styling and layout unchanged.
- Limit changes to CSS (no structural or copy changes).

## In Scope
- Make skip-link focus visible using :focus-visible.
- Add subtle hover states for items that lacked them (community card link, delete button, icon button).
- Add text wrapping or line clamping for long text in community list and detail.
- Add numeric alignment via tabular-nums for score values.
- Improve title wrapping for community page title (text-wrap).

## Out of Scope
- New UI components or layout changes.
- Content copy changes.
- Broad refactoring of shared button/link styles.

## Success Criteria
- Web Interface Guidelines review returns no findings for the targeted files.
- No layout shifts or visual regressions from the current Figma style.
- Interaction feedback remains consistent with the existing palette and glassmorphism style.

## Constraints
- Keep changes minimal and documented.
- Stay within the existing Figma styling direction.
- All changes must be in the worktree.
