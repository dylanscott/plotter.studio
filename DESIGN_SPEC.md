# Pen Plotter Web Application - Design Specification

## Project Overview

A web application for plotting SVG files with a pen plotter using the WebSerial API. The application provides a technical, blueprint-inspired interface for uploading SVGs, configuring plot parameters, assigning paths to layers each of which has pen settings (color and size), and executing plots with real-time preview.

**Target Platform**: Desktop Chrome (WebSerial requirement)
**Inspired By**: Teenage Engineering (minimal, technical, utilitarian), Blueprint aesthetic (precision, technical drawings)

---

## Visual Aesthetic

### Core Design Direction

**Blueprint/Technical Theme with Light/Dark Modes**:

The application supports both light and dark themes to match different plotting scenarios:
- **Light theme** (default): Subtle blue-tinted white background to simulate the common case of plotting dark pens on white paper
- **Dark theme** : Deep navy background for classic blueprint aesthetic - ideal for visualizing inverted plots on specialized black paper
- Subtle grid pattern in preview area maintains blueprint aesthetic in both themes
- Sharp, precise UI elements with thin borders
- Technical, monospaced typography throughout
- Grid-aligned, precisely spaced layouts

**Teenage Engineering Influence**:
- Extreme minimalism in controls
- Functional, no-nonsense design
- Monospaced fonts for technical information
- Utilitarian color coding (only where functionally necessary)
- Small, efficient use of screen space
- Clean, undecorated UI elements

### Typography

- **Primary Font**: IBM Plex Mono (monospaced, technical)
- **Font Sizes**:
  - Section headers: 11px, uppercase, letter-spacing: 1.2px
  - Body/controls: 13px
  - Technical data: 12px
  - Status info: 14px
- **Font Weight**: Regular (400) for most text, Medium (500) for emphasis
- **All caps for labels and headers** (technical drawing style)

### Color Palette

The application uses CSS custom properties for theming, allowing easy switching between light and dark modes.

**Light Theme** (default):
```css
/* Background & Surface */
--bg-primary: #f0f4f8;        /* Subtle blue-tinted white */
--bg-secondary: #e8eef4;      /* Slightly darker surface */
--bg-surface: #ffffff;        /* Pure white for panels */

/* Accent & Interactive */
--accent-primary: #0066cc;    /* Darker blue for contrast */
--accent-bright: #003d7a;     /* Deep blue for emphasis */
--accent-attention: #cc8800;  /* Darker gold for warnings */

/* Grid & Borders */
--grid-line: rgba(0, 102, 204, 0.08);   /* Very subtle grid */
--border-subtle: rgba(0, 60, 122, 0.15);
--border-emphasis: rgba(0, 102, 204, 0.3);

/* Text */
--text-primary: #1a2332;
--text-secondary: rgba(26, 35, 50, 0.7);
--text-tertiary: rgba(26, 35, 50, 0.5);

/* Canvas */
--canvas-bg: #f8fbff;         /* Very subtle blue tint (paper simulation) */
--canvas-frame: #0066cc;      /* Frame outline */
--canvas-path-default: #1a2332; /* Dark paths on light background */

/* States */
--state-plotting: #00aa55;    /* Darker green for light theme */
--state-paused: #ff8800;      /* Orange */
--state-error: #cc0033;       /* Deep red */
```

**Dark Theme**:
```css
/* Background & Surface */
--bg-primary: #0a1628;        /* Deep blueprint navy */
--bg-secondary: #0d1b2a;      /* Slightly lighter panel */
--bg-surface: #1a2332;        /* Control panel sections */

/* Accent & Interactive */
--accent-primary: #00d9ff;    /* Primary blueprint cyan */
--accent-bright: #e0f7ff;     /* Bright white for emphasis */
--accent-attention: #ffd700;  /* Warning/attention (sparingly) */

/* Grid & Borders */
--grid-line: rgba(0, 217, 255, 0.1);    /* Subtle blueprint grid */
--border-subtle: rgba(224, 247, 255, 0.15);
--border-emphasis: rgba(0, 217, 255, 0.4);

/* Text */
--text-primary: #e0f7ff;
--text-secondary: rgba(224, 247, 255, 0.6);
--text-tertiary: rgba(224, 247, 255, 0.4);

/* Canvas */
--canvas-bg: #0a1628;         /* Same as bg-primary */
--canvas-frame: #e0f7ff;      /* Frame outline */
--canvas-path-default: #00d9ff; /* Default path color */

/* States */
--state-plotting: #00ff88;    /* Active plotting green */
--state-paused: #ffaa00;      /* Paused orange */
--state-error: #ff3366;       /* Error red */
```

**Theme Toggle**:
- Add theme toggle button in top-right corner of control panel
- Persist theme preference to localStorage
- Initial theme should _not_ be inherited from the OS preference (e.g. CSS `light-dark()`), as the theme is intended to help preview the plot rather than being purely aesthetic
- Smooth transition between themes (200ms for backgrounds, instant for text)

### Visual Details

- **Borders**: 1px solid, thin and precise
- **Border Radius**: 0px (sharp, technical) or minimal (2px maximum)
- **Shadows**: Minimal, only for floating elements (subtle cyan glow on focus)
- **Grid Pattern**: Subtle blueprint grid (5-10px squares, very faint lines)
- **Animations**: Subtle, fast (150-200ms), emphasize state changes

---

## Layout Structure

### Overall Layout

Full-screen application with fixed two-column layout:

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  PREVIEW CANVAS (60-70% width)    │  CONTROL PANEL (30-40%) │
│                                   │                         │
│  [PixiJS WebGL Canvas]            │  [Svelte Components]    │
│                                   │                         │
│                                   │                         │
│                                   │                         │
└─────────────────────────────────────────────────────────────┘
```

**Split Ratio**: 65% preview / 35% controls (adjustable via draggable divider?)

### Preview Canvas (Left Side)

- **Grid**: Subtle blueprint grid pattern overlay
- **Frame**: Rectangle representing plottable area
  - Border: 2px solid `--accent-white`
  - Position: Centered in canvas initially
- **Path Rendering**: Paths rendered based on the layer they are assigned to
- **During Plotting**:
  - Unplotted paths: Dimmed (40% opacity)
  - Current stroke: Bright, animated
  - Completed strokes: Full color
- **Controls** (subtle, corner-positioned):
  - Theme toggle
  - Zoom controls (bottom-right corner)
  - Pan hint (when needed)
  - Grid toggle (optional)

### Control Panel (Right Side)

Vertical scrolling panel with collapsible sections.

**Structure**:
1. **Always Visible (Pinned Top)**:
   - Plotter Connection
   - File/Plot Status

2. **Collapsible Sections** (one expanded at a time):
   - Position & Scale
   - Layers
   - Plot Settings

---

## Control Panel Sections - Detailed Specification

### 1. Plotter Connection (Always Visible)

**When Disconnected**:
```
┌─────────────────────────────────┐
│ PLOTTER                         │
├─────────────────────────────────┤
│ Status: Not Connected           │
│                                 │
│ [CONNECT PLOTTER]               │
└─────────────────────────────────┘
```

**When Connected (Idle)**:
```
┌─────────────────────────────────┐
│ PLOTTER                         │
├─────────────────────────────────┤
│ Model: NextDraw 8511            │
│ Size: 8.5 × 11 in               │
│                                 │
│ ALIGNMENT                       │
│ [MANUAL POSITIONING]            │
│ [AUTO HOME]                     │
│                                 │
│ [DISCONNECT]                    │
└─────────────────────────────────┘
```

**When Connected (Manual Positioning Active)**:
```
┌─────────────────────────────────┐
│ PLOTTER                         │
├─────────────────────────────────┤
│ Model: NextDraw 8511            │
│ Size: 8.5 × 11 in               │
│                                 │
│ ALIGNMENT                       │
│ [MANUAL POSITIONING] ✓          │
│ Pen raised, motors disabled     │
│ Position head, then click again │
│                                 │
│ [DISCONNECT]                    │
└─────────────────────────────────┘
```

**When Auto-Homing**:
```
┌─────────────────────────────────┐
│ PLOTTER                         │
├─────────────────────────────────┤
│ Model: NextDraw 8511            │
│ Status: Homing...               │
│                                 │
│ ALIGNMENT                       │
│ [MANUAL POSITIONING]            │
│ [AUTO HOME] ⟳                   │
│                                 │
│ [DISCONNECT]                    │
└─────────────────────────────────┘
```

**When Plotting**:
```
┌─────────────────────────────────┐
│ PLOTTER                         │
├─────────────────────────────────┤
│ Model: NextDraw 8511            │
│ Status: Plotting... (Group 2/3) │
│                                 │
│ (alignment controls hidden)     │
│                                 │
│ [DISCONNECT] ⚠                  │
└─────────────────────────────────┘
```

**Elements**:

**Section Header**:
- "PLOTTER" (11px, uppercase, `--text-secondary`)

**Model & Status Info**:
- Model name: 12px monospace, `--text-primary`
- Size: 12px monospace, `--text-secondary`
- Status line (only shown when relevant):
  - Not connected: `--text-tertiary`
  - Homing...: `--accent-primary`
  - Plotting...: `--state-plotting`
  - Error: `--state-error`

**Alignment Controls Group** (only when connected and not plotting):
- "ALIGNMENT" label: 10px uppercase, `--text-tertiary`, subtle divider
- Manual Positioning button:
  - Toggle button styled like regular button
  - Shows ✓ checkmark when active
  - Active state: `--accent-attention` border/text
  - When active, help text appears below:
    - "Pen raised, motors disabled"
    - "Position head, then click again"
    - Text size: 11px, color: `--text-secondary`
- Auto Home button (only if model supports):
  - Regular bordered button
  - Shows ⟳ spinner when homing in progress
  - Disabled during manual positioning or homing
- Both buttons: Full-width, stacked vertically, 8px gap

**Disconnect Button**:
- Full-width bordered button
- Subtle styling (secondary action)
- When plotting: Shows ⚠ warning icon
- Triggers confirmation dialog if plot in progress:
  - "Stop plot and disconnect plotter?"
  - "This will immediately halt the plotter. Continue?"
  - [CANCEL] [DISCONNECT]


**State Management**:

```typescript
type PlotterState =
  | 'disconnected'
  | 'idle'               // Connected, ready
  | 'manual-positioning' // Motors off, user aligning
  | 'homing'             // Auto-homing routine running
  | 'plotting'           // Active plot
  | 'paused'             // Plot paused
  | 'error';

interface PlotterConnection {
  // ... existing fields ...
  state: PlotterState;
  supportsAutoHome: boolean;
  manualPositioningEnabled: boolean;  // Queried from firmware on connect
}
```

**Control Availability**:
```
State              | Manual Pos | Auto Home | Disconnect | Plot Button
-------------------|------------|-----------|------------|-------------
disconnected       | hidden     | hidden    | hidden     | disabled
idle               | enabled    | enabled*  | enabled    | enabled
manual-positioning | enabled†   | disabled  | enabled    | disabled
homing             | disabled   | disabled‡ | enabled    | disabled
plotting           | hidden     | hidden    | enabled§   | enabled
paused             | hidden     | hidden    | enabled§   | enabled
error              | disabled   | disabled  | enabled    | disabled

* Only shown if supportsAutoHome === true
† Button is active/checked state
‡ Button shows spinner
§ Shows warning icon, triggers confirmation
```

**Behavior Details**:

1. **On Connect**:
   - Query firmware for motor state
   - Set `manualPositioningEnabled` based on firmware response
   - Show Manual Positioning button as active if motors are disabled
   - Query if auto-home is supported, hide button if not

2. **Manual Positioning Toggle**:
   - Click to enable: Send firmware command to raise pen and disable motors
   - Click to disable: Send firmware command to re-engage motors
   - Help text only shown when active
   - Button visual state changes (checkmark, accent color)

3. **Auto Home**:
   - Click: Send firmware homing command
   - State changes to 'homing'
   - Status text shows "Homing..."
   - Button shows spinner
   - When complete, state returns to 'idle'
   - Typical duration: 2-5 seconds

4. **Disconnect While Plotting**:
   - Button remains visible but shows warning icon
   - Click triggers modal confirmation
   - If confirmed: Immediately close serial connection, stop plot
   - UI handles disconnection gracefully (doesn't crash if physical disconnect)

5. **Physical Disconnection Handling**:
   - Serial disconnect event detected
   - If plotting: Stop plot, show error state
   - Clean up resources
   - Show reconnection prompt

---

### 2. Plot Status (Always Visible)

```
┌─────────────────────────────────┐
│ PLOT STATUS                     │
├─────────────────────────────────┤
│ File: drawing.svg               │
│ Dimensions: 8.3 × 10.7 in       │
│ Paths: 1,247                    │
│ Est. Time: 23m 45s              │
│                                 │
│ ☐ Dry Run                       │
│                                 │
│ [PLOT] [PAUSE] [STOP]           │
│                                 │
│ Progress: ████████░░░░ 67%      │
│ Time Remaining: 7m 52s          │
└─────────────────────────────────┘
```

**Elements**:
- File info: File name, dimensions, path count (12px, `--text-primary`)
- Estimated time: Pre-plot calculation (12px, `--text-secondary`)
- Dry Run checkbox:
  - Standard checkbox with label
  - Forced ON (disabled) when no plotter connected
- Action buttons:
  - PLOT: Primary action, `--accent-cyan` border, filled on hover
  - PAUSE: Only visible during plotting, `--accent-yellow` border
  - STOP: Only visible during plotting, `--state-error` border
  - Buttons: 13px uppercase, monospace, bordered
- Progress bar (only visible during plotting):
  - Custom progress bar with blueprint aesthetic
  - Cyan fill, subtle animation
  - Percentage and time remaining below

---

### 3. Position & Scale (Collapsible)

```
┌─────────────────────────────────┐
│ ▼ POSITION & SCALE              │
├─────────────────────────────────┤
│ X Offset: [1.75] in   [+] [-]   │
│ Y Offset: [1.35] in   [+] [-]   │
│                                 │
│ Scale: [1.00]         [+] [-]   │
│                                 │
│ [CENTER] [RESET]                │
└─────────────────────────────────┘
```

**Elements**:
- Section header: Collapsible (▼ when expanded, ▶ when collapsed)
- Numeric inputs: Monospace, right-aligned, units appended
- +/- buttons: Small, square, precise increments (0.1 in, 0.1 scale)
- Action buttons:
  - CENTER: Sets the X and Y offsets to center the plot geometry in the frame.
  - RESET: Resets the X and Y offsets to zero, and the scale to 1.

**Behavior**:
- Real-time preview updates as values change (debounced)
- +/- buttons increment by sensible amounts
- Changes trigger re-digest of SVG via WASM module

---

### 4. Layers (Collapsible)

Most complex section. Allows the plot to be split into multiple layers, which are plotted separately to allow the pen to be swapped in between.

```
┌─────────────────────────────────┐
│ ▼ LAYERS                        │
├─────────────────────────────────┤
│ [MANAGE PEN LIBRARY]            │
│                                 │
│ ┌─ Layer  1 ──────────────────┐ │
│ │ Pen: [Black 0.5mm ▼]    👁 │ │
│ │ Paths: 847                  │ │
│ │ ≡                           │ │
│ └─────────────────────────────┘ │
│                                 │
│ ┌─ Layer 2 ───────────────────┐ │
│ │ Pen: [Red 0.3mm ▼]      👁 │ │
│ │ Paths: 400                  │ │
│ │ ≡                           │ │
│ └─────────────────────────────┘ │
│                                 │
│ [+ ADD LAYER]                   │
│                                 │
│ ─ PATH ASSIGNMENT ────────────  │
│                                 │
│ ▼ group1 (847)         [Layer 1]│
│   ▶ shapes (420)       [Layer 1]│
│   ▼ text (427)         [Layer 2]│
│     • path1            [Layer 2]│
│     • path2            [Layer 2]│
└─────────────────────────────────┘
```

**Pen Library Management**:
- Button opens modal/expanded section
- List of saved pens:
  - Name (editable)
  - Color swatch (clickable color picker)
  - Size (numeric input, mm)
  - Delete button
- Add new pen button
- Stored in localStorage

**Layer List**:
- Each layer card shows:
  - Layer number/label
  - Pen dropdown (from library), or manual pen configuration (not persisted to library)
  - Path count
  - Visibility toggle (👁 icon) - show/hide in preview, hidden layers are not plotted
  - Reorder handle (≡ drag handle)
- Cards draggable to reorder plot sequence
- Add Layer button

**Path Assignment Tree**:
- Expandable tree view of SVG hierarchy (which can be arbitrarily nested, though in the common case the user will already have organized into top-level groups to be assigned to layers)
- Each node shows:
  - Expand/collapse arrow (if has children)
  - Node name (from SVG id/label if available)
  - Path count in node
  - Layer assignment dropdown (on right)
- Visual indicator for inherited vs explicit assignment:
  - Explicit: Bold layer name
  - Inherited: Italic, dimmed
- Children inherit parent's layer unless overridden
- Collapsed by default, showing only top-level groups

**Behavior**:
- Changing layer assignment updates preview immediately
- Changing pen color/size updates preview
- Visibility toggle hides/shows paths in preview
- Reordering layers changes plot sequence

---

### 5. Plot Settings (Collapsible)

Two sub-sections within this collapsible section:

#### 5a. Speed & Quality

```
┌─────────────────────────────────┐
│ ▼ PLOT SETTINGS                 │
├─────────────────────────────────┤
│ SPEED & QUALITY                 │
│                                 │
│ Handling Mode:                  │
│ [Technical Drawing    ▼]        │
│                                 │
│ Acceleration:                   │
│ [Standard             ▼]        │
│                                 │
│ Pen Raising Speed:              │
│ [Standard             ▼]        │
│                                 │
│ Pen Lowering Speed:             │
│ [Standard             ▼]        │
└─────────────────────────────────┘
```

**Handling Mode Options**:
- Technical Drawing (default)
- Handwriting
- Sketching
- Constant Speed

**Acceleration Options**:
- Maximum
- High
- Standard (default)
- Slow

**Pen Speed Options** (both raising and lowering):
- Maximum
- Standard (default)
- Slow
- Very Slow
- Dead Slow

#### 5b. Optimization

```
┌──────────────────────────────────┐
│ OPTIMIZATION                     │
│                                  │
│ ☑ Randomize Closed Path Start    │
│                                  │
│ Path Optimization:               │
│ ○ None: Preserve path order      │
│ ● Least: Connect adjoining paths │
│ ○ Basic: Allow path reordering   │
│ ○ Full: Allow path reversal      │
└──────────────────────────────────┘
```

**Elements**:
- Checkbox for randomize closed path start
- Radio button group for optimization level (default: Full):
  - None: Preserve path order
  - Least: Connect adjoining paths
  - Basic: Allow path reordering
  - Full: Allow path reversal

**Persistence**:
- All plot settings persist to localStorage
- Restored on next session

---

## Component Architecture (Svelte)

### Component Tree

```
App.svelte
├── CanvasContainer.svelte (PixiJS wrapper)
│   └── PlotCanvas.ts (PixiJS application)
├── ControlPanel.svelte
    ├── PlotterConnection.svelte
    ├── PlotStatus.svelte
    ├── CollapsibleSection.svelte (reusable)
    │   ├── PositionScale.svelte
    │   ├── Layers.svelte
    │   │   ├── PenLibrary.svelte (modal)
    │   │   ├── LayerCard.svelte (repeated)
    │   │   └── PathTree.svelte
    │   │       └── PathTreeNode.svelte (recursive)
    │   └── PlotSettings.svelte
    │       ├── SpeedQuality.svelte
    │       └── Optimization.svelte
```

### Key Svelte Patterns

- **Stores** for shared state:
  - `plotterStore` (connection status, model info)
  - `fileStore` (SVG data, paths, tree structure)
  - `penStore` (pen library, layers, assignments)
  - `plotStore` (plot status, progress, settings)
  - `uiStore` (expanded sections, visibility toggles)

- **Two-way bindings** for form inputs
- **Reactive statements** for computed values (path counts, estimated time)
- **Component composition** for reusability (CollapsibleSection, buttons, inputs)

---

## PixiJS Preview Implementation

### Canvas Structure

```
PIXI.Application
├── backgroundContainer (grid pattern)
├── frameContainer (plottable area rectangle)
├── layersContainer
│   ├── layer1Container
│   │   └── PIXI.Graphics (paths)
│   ├── layer2Container
│   │   └── PIXI.Graphics (paths)
│   └── ...
└── progressContainer (during plotting)
    └── PIXI.Graphics (current stroke highlight)
```

### Rendering Details

**Path Rendering**:
- Each layer gets its own container
- Paths rendered as `PIXI.Graphics` lines
- Line color from pen configuration
- Line width from pen size (scaled appropriately)

**Grid Pattern**:
- Subtle background grid (5-10px squares)
- Very low opacity (`--grid-line`)
- Consider using PIXI.Graphics or texture

**Frame**:
- Rectangle outline representing plottable area
- White/cyan border, no fill
- Position/scale updated based on position controls

**Progress Animation** (during plotting):
- Dim unplotted paths (40% opacity)
- Highlight current stroke:
  - Brighter color
  - Subtle glow effect
  - Animated drawing (reveal along path)
- Completed strokes at full color

**Performance Considerations**:
- Use `PIXI.Graphics` for vector paths
- Consider caching for static elements
- Batch rendering where possible
- Optimize for potentially thousands of paths

---

## Interaction Behaviors

### File Upload
1. User uploads SVG file
2. WASM module digests SVG with default transform → DigestResult
3. All paths initially assigned to default layer
5. Preview renders polylines in default pen color
6. Estimated time calculated (future: once motion planning API exists)

### Transform Changes (Position/Scale)
1. User adjusts offset or scale values
2. Changes are debounced (150ms)
3. WASM re-digests SVG with new transform
4. Preview updates with new polyline data
5. Note: This is relatively expensive, hence the debouncing

### Handling Mode Changes
1. User changes handling mode in plot settings
2. WASM re-digests SVG (mode affects curve tolerance)
3. Preview updates with new polyline approximations

### Layer Management
1. User can add/remove layers
2. Assign SVG tree nodes to layers
3. Children inherit parent assignment unless overridden
4. Preview updates in real-time with pen colors
5. Reorder groups to change plot sequence
6. Does NOT trigger re-digestion (works on existing polylines)

### Plot Execution
1. User clicks PLOT (or dry run enabled)
2. Motion planning API generates commands (future work)
3. If real plot: Send commands via WebSerial
4. If dry run: Simulate execution
5. Progress updates stream back
6. Preview animates current stroke
7. Pause/Stop available during execution

---

## Accessibility Considerations

- Keyboard navigation for all controls
- Focus visible states (subtle cyan outline)
- ARIA labels for icon-only buttons
- Proper semantic HTML in Svelte components
- Screen reader announcements for status changes
- High contrast (blueprint theme naturally high contrast)

---

## Technical Implementation Notes

### API sketch

I've started to sketch out some TypeScript APIs for representing the simplified SVG geometry that will be returned by the `digestSVG` function exported from the WASM module. The package `plotter-api` in the repository root contains this code.

### Unit-Safe Types with Dimensional Analysis

In both the UI and the Rust side, the type system will be used to ensure unit safety and prevent incompatible measurements. In TypeScript, branded types will be used for this:

```typescript
// ============================================================================
// Unit-Safe Branded Types
// ============================================================================

/**
 * Branded type for lengths with explicit units.
 * Prevents mixing incompatible units at compile time.
 */
type Length<Unit extends string> = number & { __unit: Unit };

/**
 * 2D point in inches
 */
type Point = [x: Length<"in">, y: Length<"in">];
```

This is especially important for the motion planning API (future work, see below).

### Motion Planning API

The motion planning API (which will take the simplified polyline tree, layer assignments, and plot settings, and generate motor commands to plot each layer) is intentionally omitted from this design specification. It will be designed separately and will introduce additional branded types for:
- Motor steps (different unit from inches)
- Velocity (steps/second)
- Acceleration (steps/second²)
- Jerk (steps/second³)

This deferred design allows us to focus on the UI and SVG parsing workflow first.


### Theme System Implementation

The theme system uses CSS custom properties for all colors and switches between light/dark values:

```css
/* Root element class-based theming */
:root,
:root.theme-dark {
  /* Dark theme variables (default) */
  --bg-primary: #0a1628;
  /* ... other dark theme variables */
}

:root.theme-light {
  /* Light theme variables */
  --bg-primary: #f0f4f8;
  /* ... other light theme variables */
}

/* Smooth transitions for theme changes */
* {
  transition: background-color 200ms ease,
              color 0ms;  /* Instant text color change for readability */
}
```

**Implementation**:
- Theme preference stored in localStorage
- Applied by adding/removing `theme-light` class on root element
- All colors reference CSS variables
- PixiJS canvas background updates via JavaScript
- Theme toggle button in PlotterConnection header

### WebSerial Communication
- Request port on Connect button click (user activation required)
- Handle disconnect/errors gracefully
- Status updates reflected in PlotterConnection section

### Code Splitting & Lazy Loading

To optimize initial load time, the application should be split into two main bundles:

**Critical Path Bundle** (loads immediately):
- File upload UI
- Control panel components
- State management (stores)
- WASM module (ideally this could be loaded while the control UI renders, since it is not invoked until an SVG is uploaded)
- Basic theme and styling

**Preview Bundle** (lazy loaded):
- PixiJS library (~500KB)
- PlotCanvas implementation
- Preview rendering logic
- Grid and animation utilities

**Benefits**:
- Initial bundle ~100-150KB (controls, WASM, state)
- Preview bundle loads in background (~500KB)
- User can start uploading/configuring immediately
- Preview appears once ready (typically <1 second on good connection)

**Loading States**:
- Show subtle loading indicator in preview area
- Maintain blueprint aesthetic (animated grid fade-in?)
- Don't block user from interacting with controls

### Performance Optimizations (continued)

- Debounce position/scale inputs (150ms) - triggers re-digestion
- Debounce handling mode changes (150ms) - triggers re-digestion
- Throttle preview updates during animation
- Virtual scrolling for large path trees (if needed)
- PixiJS render loop only when needed (pause when not plotting)
- Cache digest results when possible (same SVG + same options = same result)

---

## Design Inspiration References

- Teenage Engineering's firmware updater (minimal, technical UI)
- Blueprint/technical drawing aesthetic
- Monospaced terminal interfaces
- CAD software (precision, grid-based)
- Music production software (grouped controls, collapsible sections)

---

## Open Questions / Future Enhancements

1. **Draggable divider** between preview and control panel?
2. **Zoom/pan controls** in preview - mouse wheel? Touch gestures?
3. **Save/load plot configurations** (beyond just pens and settings)?

---

## Success Criteria

The UI is successful if:
- It feels precise, technical, and minimal (Teenage Engineering vibes)
- The blueprint aesthetic is consistent and atmospheric
- Pen grouping workflow is intuitive despite complexity
- Preview updates feel instantaneous
- Plot progress visualization is clear and informative
- All controls are keyboard-accessible
- The application feels fast and responsive
- Code is maintainable and well-typed

