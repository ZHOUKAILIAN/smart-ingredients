# Figma Design System Rules - Smart Ingredients

> **Generated for Claude Code Figma Integration**
> This document provides comprehensive design system guidelines for implementing Figma designs into the Smart Ingredients codebase.

---

## Project Overview

**Smart Ingredients** is a Tauri desktop application built with Rust + Leptos for analyzing food ingredient labels using OCR and AI.

**Tech Stack:**
- **Frontend Framework:** Tauri 2.x + Leptos 0.7.x (Rust-based reactive UI)
- **Styling:** Vanilla CSS with CSS Custom Properties
- **Build System:** Trunk (WASM bundler)
- **Component Architecture:** Leptos components with reactive signals

---

## 1. Design Token Definitions

### Location
Design tokens are defined in CSS custom properties at:
```
frontend/src/styles/app.css (lines 7-40)
```

### Color Tokens

```css
:root {
  /* Brand Colors */
  --bg: #f5f7f7;                    /* Background - light gray-green */
  --text: #1f2d2d;                  /* Primary text - dark green */
  --muted: #5f6b6b;                 /* Secondary text - muted gray */
  --primary: #26a36a;               /* Primary green */
  --primary-dark: #1f8a59;          /* Dark green for active states */
  --card: #ffffff;                  /* Card background */
  --border: #e3e8e7;                /* Border color */

  /* Risk Level Colors */
  --risk-low: #10b981;              /* Green - healthy */
  --risk-low-bg: #d1fae5;           /* Light green background */
  --risk-medium: #f59e0b;           /* Amber - caution */
  --risk-medium-bg: #fef3c7;        /* Light amber background */
  --risk-high: #ef4444;             /* Red - warning */
  --risk-high-bg: #fee2e2;          /* Light red background */
}
```

### Shadow Tokens

```css
:root {
  /* Layered shadows for depth */
  --shadow-sm: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
}
```

### Transition Tokens

```css
:root {
  --transition-fast: 150ms ease;    /* Quick interactions */
  --transition-base: 250ms ease;    /* Standard animations */
}
```

### Layout Tokens

```css
:root {
  /* Safe area for mobile devices */
  --statusbar-height: max(32px, env(safe-area-inset-top, 32px));
}
```

### Token Usage Pattern
When implementing Figma designs, **always use CSS custom properties** instead of hardcoded values:

```css
/* ✅ CORRECT */
.button {
  background: var(--primary);
  color: var(--card);
  box-shadow: var(--shadow-md);
  transition: transform var(--transition-fast);
}

/* ❌ INCORRECT */
.button {
  background: #26a36a;
  color: #ffffff;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition: transform 150ms ease;
}
```

---

## 2. Component Library

### Component Location
All reusable components are located at:
```
frontend/src/components/
```

### Component Architecture

**Leptos Component Pattern:**
```rust
use leptos::prelude::*;

#[component]
pub fn ComponentName(
    // Props with type annotations
    prop_name: String,
    optional_prop: Option<String>,
) -> impl IntoView {
    // Reactive signals for state
    let state = create_rw_signal(initial_value);

    // View template with HTML-like syntax
    view! {
        <div class="component-class">
            {/* JSX-like syntax */}
            <p>{prop_name}</p>
            <Show when=move || state.get() > 0>
                <span>"Conditional content"</span>
            </Show>
        </div>
    }
}
```

### Core Components

#### 1. **RiskBadge** (`components/risk_badge.rs`)
Displays health risk level with color coding.

```rust
#[component]
pub fn RiskBadge(level: String) -> impl IntoView {
    // Maps level to CSS class and label
    view! {
        <span class={badge_class}>{label}</span>
    }
}
```

**CSS Classes:**
```css
.risk-badge-low     /* Green badge for healthy items */
.risk-badge-medium  /* Amber badge for caution */
.risk-badge-high    /* Red badge for warnings */
```

**Usage:**
```rust
<RiskBadge level="low".to_string() />
```

#### 2. **IngredientCard** (`components/ingredient_card.rs`)
Card component for displaying ingredient information.

```rust
#[component]
pub fn IngredientCard(
    name: String,
    category: String,
    function: String,
    risk_level: String,
    note: String,
) -> impl IntoView
```

**Structure:**
```html
<div class="ingredient-card-compact">
  <div class="card-header">
    <h3 class="ingredient-name">Name</h3>
    <RiskBadge />
  </div>
  <div class="tags-row">
    <span class="tag tag-category">Category</span>
    <span class="tag tag-function">Function</span>
  </div>
  <p class="ingredient-note">Note</p>
</div>
```

#### 3. **ToastHost** (`components/toast.rs`)
Toast notification system with auto-dismiss.

**Toast Types:**
- `.toast-success` - Green, for success messages
- `.toast-error` - Red, for errors
- `.toast-warning` - Amber, for warnings
- `.toast-info` - Blue, for information

#### 4. **BottomNav** (`components/bottom_nav.rs`)
Fixed bottom navigation bar with tab items.

```rust
#[component]
pub fn BottomNav() -> impl IntoView {
    view! {
        <nav class="bottom-nav">
            <button class="tab-item active">
                <span class="tab-icon">Icon</span>
                <span class="tab-label">Label</span>
            </button>
        </nav>
    }
}
```

#### 5. **Icons** (`components/icons.rs`)
SVG icon components following consistent patterns.

```rust
#[component]
pub fn IconArrowLeft() -> impl IntoView {
    view! {
        <svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            {/* SVG paths */}
        </svg>
    }
}
```

**Icon Components:**
- `IconArrowLeft` - Back navigation
- `IconCamera` - Camera/capture
- `IconChart` - Analytics/stats
- `IconCheckBadge` - Success/verified

### Component Naming Convention

| Type | Convention | Example |
|------|------------|---------|
| Component files | `snake_case.rs` | `ingredient_card.rs` |
| Component names | `PascalCase` | `IngredientCard` |
| CSS classes | `kebab-case` | `.ingredient-card-compact` |
| Props | `snake_case` | `risk_level: String` |

---

## 3. Frameworks & Libraries

### UI Framework: Leptos 0.7.x

**Reactive Primitives:**
```rust
// Read-write signal (reactive state)
let count = create_rw_signal(0);
count.set(5);                    // Update
let value = count.get();         // Read

// Derived signal (computed value)
let doubled = move || count.get() * 2;

// Effect (side effects)
create_effect(move |_| {
    println!("Count: {}", count.get());
});
```

**Conditional Rendering:**
```rust
<Show when=move || condition.get()>
    <p>"Shown when true"</p>
</Show>

<Show
    when=move || user.get().is_some()
    fallback=|| view! { <p>"Not logged in"</p> }
>
    <p>"Welcome!"</p>
</Show>
```

**List Rendering:**
```rust
<For
    each=move || items.get()
    key=|item| item.id
    children=move |item| {
        view! { <li>{item.name}</li> }
    }
/>
```

### Routing: Leptos Router 0.7

**Route Definition:**
```rust
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

<Router>
    <Routes fallback=|| view! { <p>"Not found"</p> }>
        <Route path=path!("/") view=HomePage />
        <Route path=path!("/profile") view=ProfilePage />
    </Routes>
</Router>
```

**Navigation:**
```rust
use leptos_router::hooks::use_navigate;

let navigate = use_navigate();
navigate("/result", Default::default());
```

### State Management

**Global State Pattern:**
```rust
// Define state struct
pub struct AppState {
    pub analysis_id: RwSignal<Option<Uuid>>,
    pub loading: RwSignal<bool>,
    pub toasts: RwSignal<Vec<Toast>>,
}

// Provide context at root
provide_context(AppState { /* ... */ });

// Consume in child components
let state = use_context::<AppState>().expect("AppState not found");
state.loading.set(true);
```

### Build System: Trunk

**HTML Entry Point:** `frontend/index.html`
```html
<!doctype html>
<html lang="zh-CN">
  <head>
    <link data-trunk rel="css" href="src/styles/app.css" />
    <link data-trunk rel="css" href="src/styles/bottom-nav.css" />
  </head>
  <body></body>
</html>
```

**Build Commands:**
```bash
# Development (hot reload)
trunk serve

# Production build
trunk build --release
```

---

## 4. Styling Approach

### CSS Methodology: Vanilla CSS with BEM-inspired naming

**File Organization:**
```
frontend/src/styles/
├── app.css          # Main styles (43KB)
└── bottom-nav.css   # Bottom navigation styles (3KB)
```

### CSS Architecture Pattern

**1. Global Resets & Base Styles**
```css
*,
*::before,
*::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: "PingFang SC", "Microsoft YaHei", sans-serif;
  background: var(--bg);
  color: var(--text);
}
```

**2. Layout Classes**
```css
.app-shell {
  height: 100vh;
  width: 100vw;
  padding-top: var(--statusbar-height);
  background: linear-gradient(to bottom, #d1fae5 0, var(--bg) 120px);
}

.page {
  width: 100%;
  max-width: 100%;
  background: var(--card);
  padding: 24px 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
```

**3. Component-Scoped Classes**
```css
/* Component: Card */
.card {
  background: var(--card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-sm);
}

/* Component: Button */
.primary-button {
  height: 48px;
  border-radius: 14px;
  background: var(--primary);
  color: white;
  border: none;
  cursor: pointer;
}
```

**4. State Modifiers**
```css
.tab-item {
  color: #94a3b8;
}

.tab-item.active {
  color: #10b981;
}

.tab-item:active {
  transform: scale(0.95);
}
```

### Responsive Design Strategy

**Mobile-First Approach:**
- Base styles target mobile (320px+)
- No media queries needed (desktop app with fixed viewport)
- Safe area insets for notches/rounded corners

```css
/* Safe area support */
.bottom-nav {
  padding: 4px env(safe-area-inset-right)
              env(safe-area-inset-bottom)
              env(safe-area-inset-left);
}
```

### Animation Patterns

**Micro-interactions:**
```css
.button {
  transition: transform var(--transition-fast);
}

.button:active {
  transform: scale(0.98);
}
```

**Loading Animations:**
```css
.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
```

**Entrance Animations:**
```css
.toast {
  animation: toastSlideIn 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
}

@keyframes toastSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
```

### Gradient Patterns

**Brand Gradients:**
```css
/* Primary gradient (buttons, highlights) */
background: linear-gradient(135deg, #10b981, #059669);

/* Background gradient (pages) */
background: linear-gradient(
  135deg,
  rgba(236, 253, 245, 0.7) 0%,
  #ffffff 45%,
  rgba(240, 253, 250, 0.7) 100%
);

/* Glass morphism */
background: rgba(255, 255, 255, 0.92);
backdrop-filter: blur(14px);
```

---

## 5. Asset Management

### Asset Storage
Currently, assets are embedded as inline SVG in Rust components.

**Pattern:**
```rust
#[component]
pub fn IconCamera() -> impl IntoView {
    view! {
        <svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="..."></path>
        </svg>
    }
}
```

### Image Handling

**User-Uploaded Images:**
- Stored locally or cloud storage (configured via env)
- Accessed via URL strings
- Displayed with `<img>` tags

```rust
<img
    src={image_url.get()}
    alt="Ingredient label"
    style="max-height: 360px; object-fit: contain;"
/>
```

### Asset Optimization

**SVG Icons:**
- Inline in components (no HTTP requests)
- Use `currentColor` for dynamic theming
- Remove unnecessary attributes

```rust
view! {
    <svg
        class="icon"
        fill="none"
        stroke="currentColor"  // Inherits parent color
        stroke-width="2"
        aria-hidden="true"
        focusable="false"
    >
        <path d="..."></path>
    </svg>
}
```

---

## 6. Icon System

### Icon Storage
Icons are defined as Leptos components in:
```
frontend/src/components/icons.rs
```

### Icon Architecture

**Standard SVG Template:**
```rust
#[component]
pub fn IconName() -> impl IntoView {
    view! {
        <svg
            class="icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <path d="..."></path>
        </svg>
    }
}
```

### Icon Usage Pattern

**In Components:**
```rust
use crate::components::IconCamera;

view! {
    <button class="icon-button">
        <IconCamera />
        <span>"Take Photo"</span>
    </button>
}
```

**CSS Sizing:**
```css
.icon {
  width: 20px;
  height: 20px;
}

.tab-icon .icon {
  width: 22px;
  height: 22px;
}
```

### Adding New Icons from Figma

**Step 1: Export SVG from Figma**
- Select icon
- Export as SVG
- Optimize with SVGO (remove unnecessary attributes)

**Step 2: Create Rust Component**
```rust
// In frontend/src/components/icons.rs

#[component]
pub fn IconNewIcon() -> impl IntoView {
    view! {
        <svg
            class="icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            {/* Paste optimized SVG paths here */}
            <path d="..."></path>
        </svg>
    }
}
```

**Step 3: Export in mod.rs**
```rust
// In frontend/src/components/mod.rs
pub use icons::{IconArrowLeft, IconCamera, IconNewIcon};
```

### Icon Naming Convention
- Component name: `Icon{PascalCase}`
- File location: `icons.rs` (all icons in one file)
- Example: `IconCheckBadge`, `IconArrowLeft`

---

## 7. Project Structure

### Monorepo Layout

```
smart-ingredients/
├── frontend/              # Tauri + Leptos app
│   ├── src/
│   │   ├── components/    # Reusable UI components
│   │   ├── pages/         # Page-level components
│   │   ├── services/      # API client
│   │   ├── stores/        # State management
│   │   ├── styles/        # CSS files
│   │   ├── types/         # Frontend-specific types
│   │   ├── utils/         # Helper functions
│   │   ├── lib.rs         # App entry point
│   │   └── main.rs        # WASM entry
│   ├── src-tauri/         # Tauri native layer
│   ├── index.html         # HTML entry point
│   └── Cargo.toml         # Frontend dependencies
├── backend/               # Axum API server
│   └── src/
├── shared/                # Shared types
│   └── src/
│       ├── analysis.rs    # Analysis types
│       ├── ingredient.rs  # Ingredient types
│       ├── auth.rs        # Auth types
│       └── lib.rs
├── docs/                  # Documentation
│   ├── requirements/      # Feature specs
│   ├── design/            # Technical designs
│   ├── api/               # API docs
│   └── standards/         # Coding standards
└── Cargo.toml             # Workspace config
```

### Component Organization Pattern

```
frontend/src/components/
├── mod.rs                 # Public exports
├── ingredient_card.rs     # Single component per file
├── risk_badge.rs
├── toast.rs
├── icons.rs               # All icons in one file
└── bottom_nav.rs
```

**Module Pattern:**
```rust
// mod.rs - Central export point
mod ingredient_card;
mod risk_badge;

pub use ingredient_card::IngredientCard;
pub use risk_badge::RiskBadge;
```

### Page Organization Pattern

```
frontend/src/pages/
├── mod.rs                 # Page exports
├── capture.rs             # Home/capture page
├── result.rs              # Results page
├── login.rs               # Login page
└── profile.rs             # Profile page
```

**Page Component Pattern:**
```rust
// pages/capture.rs
use leptos::prelude::*;
use crate::components::{BottomNav, ToastHost};

#[component]
pub fn CapturePage() -> impl IntoView {
    view! {
        <div class="page page-capture figma">
            {/* Page content */}
        </div>
    }
}
```

---

## 8. Type System & Data Flow

### Shared Types Location
All API-shared types are in:
```
shared/src/
├── analysis.rs      # Analysis request/response types
├── ingredient.rs    # Ingredient types
├── auth.rs          # Authentication types
└── user.rs          # User types
```

### Type Definition Pattern

**All shared types must derive:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeName {
    pub field: String,
}
```

### Key Data Structures

**Analysis Result:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub health_score: i32,
    pub summary: String,
    pub table: Vec<TableRow>,
    pub ingredients: Vec<IngredientInfo>,
    pub warnings: Vec<Warning>,
    pub recommendation: String,
}
```

**Table Row (for UI rendering):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub name: String,
    pub category: String,
    pub function: String,
    pub risk_level: String,  // "low" | "medium" | "high"
    pub note: String,
}
```

### Data Flow Pattern

**1. API Request (Frontend → Backend):**
```rust
// Frontend service call
use reqwest::Client;

let response = Client::new()
    .post(format!("{}/api/v1/analysis/upload", api_url))
    .multipart(form)
    .send()
    .await?;

let result: AnalysisResponse = response.json().await?;
```

**2. State Update (Reactive):**
```rust
// Update reactive signal
let state = use_context::<AppState>().unwrap();
state.analysis_result.set(Some(result));
```

**3. UI Rendering (Reactive):**
```rust
view! {
    <Show when=move || state.analysis_result.get().is_some()>
        {move || {
            let result = state.analysis_result.get().unwrap();
            view! {
                <IngredientCard
                    name={result.name}
                    risk_level={result.risk_level}
                />
            }
        }}
    </Show>
}
```

---

## 9. Figma-to-Code Workflow

### Step 1: Analyze Figma Design

**Extract Design Tokens:**
1. Identify colors → Map to CSS custom properties
2. Identify spacing → Use consistent values (4px grid)
3. Identify typography → Use existing font stack
4. Identify shadows → Use existing shadow tokens

**Example Mapping:**
```
Figma: #10B981 → CSS: var(--risk-low)
Figma: 16px border-radius → CSS: border-radius: 16px;
Figma: Drop shadow → CSS: box-shadow: var(--shadow-md);
```

### Step 2: Create Component Structure

**Rust Component Template:**
```rust
use leptos::prelude::*;

#[component]
pub fn NewComponent(
    // Props from Figma design
    title: String,
    description: Option<String>,
    on_click: Callback<()>,
) -> impl IntoView {
    // Reactive state if needed
    let is_active = create_rw_signal(false);

    view! {
        <div class="new-component">
            <h2 class="component-title">{title}</h2>
            <Show when=move || description.is_some()>
                <p class="component-description">
                    {description.clone().unwrap()}
                </p>
            </Show>
            <button
                class="component-button"
                on:click=move |_| {
                    is_active.set(true);
                    on_click.call(());
                }
            >
                "Click Me"
            </button>
        </div>
    }
}
```

### Step 3: Write CSS Styles

**CSS Template:**
```css
/* Component: NewComponent */
.new-component {
  background: var(--card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-md);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.component-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
}

.component-description {
  margin: 0;
  font-size: 14px;
  color: var(--muted);
  line-height: 1.5;
}

.component-button {
  width: 100%;
  height: 48px;
  border-radius: 14px;
  background: var(--primary);
  color: white;
  border: none;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform var(--transition-fast);
}

.component-button:active {
  transform: scale(0.98);
}
```

### Step 4: Export and Use Component

**1. Add to mod.rs:**
```rust
// In frontend/src/components/mod.rs
mod new_component;
pub use new_component::NewComponent;
```

**2. Use in Page:**
```rust
use crate::components::NewComponent;

view! {
    <NewComponent
        title="Hello".to_string()
        description=Some("Description".to_string())
        on_click=Callback::new(move |_| {
            // Handle click
        })
    />
}
```

---

## 10. Design System Best Practices

### ✅ DO

1. **Use CSS Custom Properties**
   ```css
   background: var(--primary);  /* ✅ */
   ```

2. **Follow Existing Patterns**
   ```rust
   #[component]
   pub fn MyComponent(prop: String) -> impl IntoView {
       view! { <div>{prop}</div> }
   }
   ```

3. **Use Reactive Signals for State**
   ```rust
   let count = create_rw_signal(0);
   count.set(count.get() + 1);
   ```

4. **Leverage Existing Components**
   ```rust
   <RiskBadge level="low".to_string() />
   <ToastHost />
   ```

5. **Use Consistent Spacing (4px grid)**
   ```css
   padding: 20px;    /* 5 × 4px */
   gap: 12px;        /* 3 × 4px */
   margin: 16px;     /* 4 × 4px */
   ```

6. **Add Transitions for Interactions**
   ```css
   .button {
     transition: transform var(--transition-fast);
   }
   .button:active {
     transform: scale(0.98);
   }
   ```

### ❌ DON'T

1. **Don't Hardcode Colors**
   ```css
   background: #26a36a;  /* ❌ */
   ```

2. **Don't Use Inline Styles (unless dynamic)**
   ```rust
   // ❌ Avoid
   <div style="color: red;">

   // ✅ Use CSS classes
   <div class="error-text">
   ```

3. **Don't Create Redundant Components**
   - Check existing components first
   - Extend existing patterns

4. **Don't Mix State Management Approaches**
   - Use Leptos signals consistently
   - Use context for global state

5. **Don't Ignore Accessibility**
   ```rust
   // ✅ Add ARIA attributes
   <svg aria-hidden="true" focusable="false">

   // ✅ Add alt text
   <img alt="Ingredient label" />
   ```

---

## 11. Quick Reference

### Component Checklist

When implementing a Figma design as a component:

- [ ] Extract design tokens (colors, spacing, shadows)
- [ ] Map to existing CSS custom properties
- [ ] Create Rust component with typed props
- [ ] Write CSS with BEM-inspired naming
- [ ] Use reactive signals for dynamic state
- [ ] Add transitions for interactions
- [ ] Export component in `mod.rs`
- [ ] Test with different prop values
- [ ] Ensure mobile responsiveness
- [ ] Add accessibility attributes

### CSS Class Naming Pattern

```
.component-name            # Block
.component-name__element   # Element (rare, prefer flat)
.component-name--modifier  # Modifier (rare, prefer separate class)
.component-name.active     # State class
```

**Example:**
```css
.ingredient-card           /* Block */
.card-header               /* Element (flat) */
.ingredient-name           /* Element (flat) */
.risk-badge-low            /* Component with modifier */
.tab-item.active           /* State */
```

### Common Patterns

**Button Styles:**
```css
.primary-button {
  height: 48px;
  border-radius: 14px;
  background: linear-gradient(135deg, #10b981, #059669);
  color: white;
  font-size: 16px;
  font-weight: 600;
  border: none;
  cursor: pointer;
  box-shadow: 0 12px 22px rgba(16, 185, 129, 0.3);
  transition: transform var(--transition-fast);
}

.primary-button:active {
  transform: scale(0.98);
}
```

**Card Styles:**
```css
.card {
  background: var(--card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-sm);
}
```

**Glass Morphism:**
```css
.glass-card {
  background: rgba(255, 255, 255, 0.92);
  backdrop-filter: blur(14px);
  border-radius: 24px;
  border: 1px solid rgba(255, 255, 255, 0.4);
}
```

---

## 12. Resources & Documentation

### Internal Documentation
- **Project Conventions:** `docs/standards/project-conventions.md`
- **Coding Standards:** `docs/standards/coding-standards.md`
- **API Reference:** `docs/api/api-reference.md`
- **Technical Design:** `docs/design/technical-design.md`

### External Resources
- **Leptos Documentation:** https://leptos.dev
- **Tauri Documentation:** https://tauri.app
- **Rust Book:** https://doc.rust-lang.org/book/

### Key Files
- **Main App:** `frontend/src/lib.rs`
- **Styles:** `frontend/src/styles/app.css`
- **Components:** `frontend/src/components/`
- **Shared Types:** `shared/src/`

---

## Summary

This design system is built on:
- **CSS Custom Properties** for consistent theming
- **Leptos Components** for reactive UI
- **Vanilla CSS** with BEM-inspired naming
- **Inline SVG Icons** as Rust components
- **Mobile-first** responsive design
- **Glass morphism** and gradient aesthetics

When implementing Figma designs:
1. Map design tokens to existing CSS variables
2. Create Leptos components with typed props
3. Write scoped CSS with flat class names
4. Use reactive signals for state
5. Follow existing patterns and conventions
