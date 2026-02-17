# Frontend Routing Guide

## Overview

Frontend routes are defined in `frontend/src/lib.rs` and map to page modules in `frontend/src/pages/`.

## How to Update Routes

1. Add or update a page component under `frontend/src/pages/`.
2. Register the route in `frontend/src/lib.rs`.
3. Update navigation entries in `frontend/src/components/` if needed.

## Notes

- Keep route paths stable to avoid breaking existing links.
- For new flows, document the route purpose in the corresponding requirements/design doc.
