# SVG Animation Verification Process

## Overview

This document focuses on verifying that CSS animations **exactly match** the behavior defined in the original TSX files. The primary goal is ensuring 1:1 accuracy between TSX variants and CSS implementations.

## Core Verification Principle

**The CSS file must replicate the exact animation behavior from the TSX file.** Every property, timing, and transform must match precisely.

## TSX-to-CSS Verification Workflow

### 1. Locate Required Files for Specific Icon

#### Find the Three Files for the Icon Being Verified
- **SVG paths**: `ICONS_WIP/{icon_name}.txt`
- **TSX source**: `ICONS_WIP/{icon_name}.tsx`
- **CSS implementation**: `ICONS_WIP/{icon_name}.css`

#### Extract TSX Animation Data
- **Extract variants**: Identify all `Variants` objects (e.g., `letterVariants`, `arrowVariants`)
- **Map to SVG paths**: Understand which paths use which variants

#### Example TSX Analysis
```tsx
const letterVariants: Variants = {
  normal: { opacity: 1, scale: 1 },
  animate: {
    opacity: [0, 1],
    scale: [0.8, 1],
    transition: { duration: 0.3 },
  },
};

const arrowVariants: Variants = {
  normal: { opacity: 1, y: 0 },
  animate: {
    opacity: [0, 1],
    y: [-10, 0],
    transition: { duration: 0.3, delay: 0.2 },
  },
};
```

### 2. Verify CSS Implementation Accuracy

#### Animation Property Verification
For each TSX variant, verify the CSS matches **exactly**:

| TSX Property | TSX Value | Required CSS | Verification Status |
|--------------|-----------|--------------|-------------------|
| `duration` | `0.3` | `0.3s` | ✅ / ❌ |
| `delay` | `0.2` | `0.2s` | ✅ / ❌ |
| `opacity` | `[0, 1]` | `0% { opacity: 0 } 100% { opacity: 1 }` | ✅ / ❌ |
| `scale` | `[0.8, 1]` | `0% { transform: scale(0.8) } 100% { transform: scale(1) }` | ✅ / ❌ |
| `y` | `[-10, 0]` | `0% { transform: translateY(-10px) } 100% { transform: translateY(0) }` | ✅ / ❌ |

#### Path Mapping Verification
- **TSX component structure**: Which `<motion.path>` uses which variant
- **CSS selector targeting**: Which `path[d="..."]` gets which animation
- **Grouping accuracy**: Paths that should animate together are grouped correctly

### 3. Critical Verification Checklist

#### ✅ TSX-CSS Accuracy Requirements

- [ ] **All TSX variants converted**: Every variant has corresponding CSS
- [ ] **Exact timing match**: Duration and delay values identical
- [ ] **Transform properties match**: Scale, translate, rotate values identical
- [ ] **Opacity transitions match**: Opacity arrays converted correctly
- [ ] **Path targeting accurate**: Correct SVG paths receive correct animations
- [ ] **Animation fill mode**: `both` keyword added for delayed animations
- [ ] **Easing functions**: TSX easing converted to CSS equivalent

## Detailed TSX-CSS Mapping Verification

### Step-by-Step Mapping Process

#### 1. Extract TSX Animation Data
```tsx
// Example from arrow_down.tsx
const arrowVariants: Variants = {
  normal: { opacity: 1, pathLength: 1, pathOffset: 0 },
  animate: {
    opacity: [0, 1],
    pathLength: [0, 1],
    pathOffset: [1, 0],
    transition: { duration: 0.5, delay: 0.3 },
  },
};
```

#### 2. Create Equivalent CSS Animation
```css
/* MUST match TSX exactly */
[data-name="ArrowDownAnimate"]:hover path[d="specific-arrow-path"] {
    animation: arrowDraw 0.5s ease-in-out 0.3s both;
}

@keyframes arrowDraw {
    0% {
        opacity: 0;
        stroke-dasharray: 100;
        stroke-dashoffset: 100;
    }
    100% {
        opacity: 1;
        stroke-dasharray: 100;
        stroke-dashoffset: 0;
    }
}
```

#### 3. Verification Checklist for Each Property

| TSX Property | TSX Value | CSS Implementation | ✅ Verified |
|--------------|-----------|-------------------|------------|
| `duration: 0.5` | `0.5` | `0.5s` in animation | ☐ |
| `delay: 0.3` | `0.3` | `0.3s` in animation | ☐ |
| `opacity: [0, 1]` | `[0, 1]` | `0% { opacity: 0 } 100% { opacity: 1 }` | ☐ |
| `pathLength: [0, 1]` | `[0, 1]` | `stroke-dasharray` animation | ☐ |
| `pathOffset: [1, 0]` | `[1, 0]` | `stroke-dashoffset: 100 → 0` | ☐ |

## Critical Verification Failures to Avoid

### ❌ Timing Mismatches
```tsx
// TSX: duration: 0.3
transition: { duration: 0.3 }
```
```css
/* ❌ WRONG: Different duration */
animation: animate 0.5s ease-in-out;

/* ✅ CORRECT: Exact match */
animation: animate 0.3s ease-in-out;
```

### ❌ Property Value Mismatches
```tsx
// TSX: scale from 0.8 to 1
scale: [0.8, 1]
```
```css
/* ❌ WRONG: Different scale values */
0% { transform: scale(0.5); }
100% { transform: scale(1); }

/* ✅ CORRECT: Exact TSX values */
0% { transform: scale(0.8); }
100% { transform: scale(1); }
```

### ❌ Missing Delay Fill Mode
```tsx
// TSX: Has delay
transition: { duration: 0.3, delay: 0.2 }
```
```css
/* ❌ WRONG: Will flicker during delay */
animation: animate 0.3s ease-in-out 0.2s;

/* ✅ CORRECT: Maintains state during delay */
animation: animate 0.3s ease-in-out 0.2s both;
```

### ❌ Path Targeting Errors
```tsx
// TSX: Specific path has specific variant
<motion.path d="M3.5 13h6" variants={letterVariants} />
<motion.path d="M18 7v9" variants={arrowVariants} />
```
```css
/* ❌ WRONG: All paths get same animation */
[data-name="Icon"]:hover path {
    animation: letterAnimate 0.3s;
}

/* ✅ CORRECT: Specific path targeting */
[data-name="Icon"]:hover path[d="M3.5 13h6"] {
    animation: letterAnimate 0.3s;
}
[data-name="Icon"]:hover path[d="M18 7v9"] {
    animation: arrowAnimate 0.3s 0.2s both;
}
```

## TSX-CSS Conversion Reference

### Transform Properties
| TSX Property | TSX Syntax | CSS Equivalent |
|--------------|------------|----------------|
| Scale | `scale: [0.8, 1]` | `transform: scale(0.8)` → `scale(1)` |
| Translate Y | `y: [-10, 0]` | `transform: translateY(-10px)` → `translateY(0)` |
| Translate X | `x: [20, 0]` | `transform: translateX(20px)` → `translateX(0)` |
| Rotate | `rotate: [0, 180]` | `transform: rotate(0deg)` → `rotate(180deg)` |

### Path Drawing Properties
| TSX Property | TSX Syntax | CSS Equivalent |
|--------------|------------|----------------|
| Path Length | `pathLength: [0, 1]` | `stroke-dasharray: 100; stroke-dashoffset: 100` → `0` |
| Path Offset | `pathOffset: [1, 0]` | `stroke-dashoffset: 100` → `0` |

### Timing Properties
| TSX Property | TSX Syntax | CSS Equivalent |
|--------------|------------|----------------|
| Duration | `duration: 0.3` | `0.3s` |
| Delay | `delay: 0.2` | `0.2s` |
| Ease | `ease: "easeInOut"` | `ease-in-out` |

## File-Based Verification Process

### Required Files in ICONS_WIP Directory
```
ICONS_WIP/
├── {icon_name}.txt     # SVG path data
├── {icon_name}.tsx     # Original TSX animation source
└── {icon_name}.css     # CSS implementation to verify
```

### Verification Sign-Off Checklist

Before marking any animation as complete:

- [ ] **TSX file analyzed**: All variants identified and documented from `ICONS_WIP/{icon_name}.tsx`
- [ ] **Path mapping verified**: Each SVG path in `{icon_name}.txt` matches correct TSX motion.path
- [ ] **All properties converted**: Every TSX property has CSS equivalent in `{icon_name}.css`
- [ ] **Timing exactly matches**: Duration, delay, easing identical between TSX and CSS
- [ ] **Property values exact**: Transform values, opacity ranges match precisely
- [ ] **Path targeting accurate**: CSS selectors target correct SVG path `d` attributes
- [ ] **File formatting correct**: Proper selectors, trailing newlines in CSS file

### When CSS Doesn't Match TSX

**STOP and fix immediately.** Do not proceed until:
1. Identify the exact discrepancy
2. Correct the CSS to match TSX exactly
3. Re-verify with checklist
4. Confirm visual behavior is identical

**Remember: The goal is 100% accuracy, not "close enough".**