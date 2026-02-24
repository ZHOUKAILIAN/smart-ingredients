# UI Guidelines Fixes Technical Plan

## Overview
Implement minimal CSS adjustments to satisfy Web Interface Guidelines without changing layout, structure, or copy. All fixes align with the current Figma-derived style (glassmorphism, emerald accents).

## Design Decisions
- CSS-only changes to keep layout and components unchanged.
- Use :focus-visible for keyboard focus while preserving the global focus ring.
- Use subtle hover feedback to avoid visual drift from Figma style.
- Prefer line-clamp for summaries to prevent overflow while keeping consistent card height.
- Use overflow-wrap for long ingredient text in detail view.
- Apply tabular-nums to numeric scores for stable alignment.

## Changes by Area
1) Focus visibility
- Update skip-link to use :focus-visible and visible outline/box-shadow.

2) Hover feedback
- Add hover style for community list card link.
- Add hover style for community delete button.
- Add hover style for icon-button.

3) Long text handling
- Clamp community list summary to 2 lines.
- Apply break-word/overflow-wrap to community detail summary and ingredients text.
- Apply text-wrap to community page title to improve wrapping.

4) Numeric alignment
- Add font-variant-numeric: tabular-nums to community and history score values.

## Files to Modify
- `frontend/src/styles/app.css`

## Verification
- Re-run web-design-guidelines review on the same files.
- Visual spot-check key screens (Community list/detail, History) for regressions.
- Follow the Execution Finish Checklist after implementation.
