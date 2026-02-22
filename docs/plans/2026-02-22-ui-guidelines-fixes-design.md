# UI Guidelines Fixes Design

## Summary
This design delivers a minimal, CSS-only fix set to address a small group of Web Interface Guidelines findings while preserving the current Figma-derived UI style. The objective is to close compliance gaps without changing layout, structure, or copy. The updates focus on four areas: focus visibility, hover feedback, long text handling, and numeric alignment.

## Scope
- Make skip-link focus visible using :focus-visible so keyboard users can see it.
- Add subtle hover feedback for interactive elements that currently lack it (community list card link, delete button, icon button).
- Clamp long summaries in the community list and apply break-word handling in community detail summaries and ingredient text to prevent overflow.
- Use tabular-nums for score displays to stabilize numeric alignment.
- Improve community page title wrapping with text-wrap to avoid awkward line breaks.

## Constraints
- Keep the current Figma style (glassmorphism + emerald accents) intact.
- No structural changes to components or pages.
- CSS-only edits in `frontend/src/styles/app.css`.

## Success Criteria
- Web Interface Guidelines review returns no findings for targeted files.
- No layout shifts or visual regressions on Community list/detail or History pages.
- Hover and focus feedback remain subtle and consistent with the existing palette.
