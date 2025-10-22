# TERM_1: Implement Terminal Dimension Padding Support

**Prefix:** TERM  
**Severity:** MEDIUM  
**Priority:** MEDIUM (Within 2 sprints)  
**Estimated Effort:** Single focused session (60-120 minutes)

---

## OBJECTIVE

Fix the incomplete terminal dimension calculation in `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs` to properly account for padding values, ensuring accurate terminal grid sizing.

Why this matters: The current simplified calculation ignores padding, causing rendering inconsistencies between calculated terminal dimensions and actual available rendering space. This results in text clipping or incorrect cursor positioning.

---

## LOOK AROUND

- Path to terminal module:
  - [terminal/mod.rs](../packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs)
- Relevant vendor and tmp sources already present (no new clones required):
  - Rio backend config: [rio-backend/src/config/mod.rs](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/mod.rs)
  - Rio defaults: [rio-backend/src/config/defaults.rs](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/defaults.rs)
  - Sugarloaf layout: [sugarloaf/src/layout/mod.rs](../packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/layout/mod.rs)
  - tmp mirror (for citation parity):
    - [tmp/rio-backend/src/config/mod.rs](../packages/sweetmcp/packages/sixel6vt/tmp/rio/rio-backend/src/config/mod.rs)
    - [tmp/sugarloaf/src/layout/mod.rs](../packages/sweetmcp/packages/sixel6vt/tmp/rio/sugarloaf/src/layout/mod.rs)

Observation summary:
- `TerminalPane::new()` computes cols/lines using total width/height and ignores padding.
- `TerminalPane::resize()` uses hardcoded `font_size` and `line_height`, and ignores padding.
- Sugarloaf is initialized with `RootStyle::new(scale, config.fonts.size, config.line_height)`, so font metrics are available at creation and should be persisted for resize.

---

## RESEARCH FINDINGS

- **Rio config padding fields**:
  - Source: [rio-backend/src/config/mod.rs](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/mod.rs)
  - Relevant excerpt:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    // ...
    #[serde(rename = "padding-x", default = "f32::default")]
    pub padding_x: f32,
    #[serde(rename = "padding-y", default = "default_padding_y")]
    pub padding_y: [f32; 2],
    // ...
}
```
  - Defaults: `../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/defaults.rs`
```rust
#[inline]
pub fn default_padding_y() -> [f32; 2] {
    [0., 0.]
}
```
- **Sugarloaf root style (font metrics)**:
  - Source: `../packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/layout/mod.rs`
```rust
pub struct RootStyle {
    pub scale_factor: f32,
    pub font_size: f32,
    pub line_height: f32,
}
```
- Mirror citations in tmp (same semantics):
  - `../packages/sweetmcp/packages/sixel6vt/tmp/rio/rio-backend/src/config/mod.rs`
  - `../packages/sweetmcp/packages/sixel6vt/tmp/rio/sugarloaf/src/layout/mod.rs`

---

## SEQUENTIAL THINKING (ULTRATHINK)

- **Core objective**: Ensure terminal grid dimensions reflect the actual drawable area by subtracting configured padding before dividing by font metrics.
- **What changes**:
  - Persist `font_size`, `line_height`, `padding_x`, `padding_y` in `TerminalPane` so `resize()` can compute consistently with `new()`.
  - Replace simplified/incorrect calculations in both `new()` and `resize()` with padding-aware versions and minimum guards.
  - Remove hardcoded font metrics and misleading comments.
- **Why existing code is close**:
  - Config and fonts are already wired into Sugarloaf via `RootStyle::new(...)`.
  - Terminal resizing pipeline (`self.sugarloaf.resize` → compute cols/lines → `CrosswordsSize::new_with_dimensions` → PTY ESC sequence) is present; it just needs correct numbers.
- **Questions**:
  - Do paddings ever exceed window bounds (tiny windows)? We will clamp available sizes to a minimum > 0 before division.
  - DPI scaling: current grid uses device px; Sugarloaf handles scaling internally. We keep computation in the same space as current code (device px) for consistency.

---

## CHANGES REQUIRED (precise edits in src)

- **File**: `../packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs`

1) **Add fields to `TerminalPane<...>` struct** (place with other fields; private):
```rust
// Configuration values needed for correct dimension calculations
font_size: f32,
line_height: f32,
padding_x: f32,
padding_y: [f32; 2],
```

2) **Update `new(...)` dimension calc to honor padding**
- Find the block with comment starting `// Calculate terminal dimensions manually`.
- Replace the simplified calc with padding-aware calc:
```rust
let padding_w = config.padding_x * 2.0; // left + right
let padding_h = config.padding_y[0] + config.padding_y[1]; // top + bottom
let avail_w = (width - padding_w).max(0.0);
let avail_h = (height - padding_h).max(0.0);

let cols = (avail_w / config.fonts.size).floor() as usize;
let lines = (avail_h / (config.fonts.size * config.line_height)).floor() as usize;
let cols = cols.max(1);
let lines = lines.max(1);
let terminal_size = (cols, lines);
```
- Keep `CrosswordsSize::new_with_dimensions(cols, lines, width_u, height_u, 0, 0)` as-is. The grid dims change; the raw window size passed through remains window pixels.

3) **Persist config values in `Ok(Self { ... })`**
- In the `Ok(Self { ... })` initializer, add:
```rust
font_size: config.fonts.size,
line_height: config.line_height,
padding_x: config.padding_x,
padding_y: config.padding_y,
```

4) **Fix `resize(&mut self, width: u32, height: u32)`**
- Remove the hardcoded font metrics and compute using stored fields with padding:
```rust
self.sugarloaf.resize(width, height);
let padding_w = self.padding_x * 2.0;
let padding_h = self.padding_y[0] + self.padding_y[1];
let avail_w = (width as f32 - padding_w).max(0.0);
let avail_h = (height as f32 - padding_h).max(0.0);

let cols = (avail_w / self.font_size).floor() as usize;
let lines = (avail_h / (self.font_size * self.line_height)).floor() as usize;
let cols = cols.max(1);
let lines = lines.max(1);

let cross_size = rio_backend::crosswords::CrosswordsSize::new_with_dimensions(
    cols,
    lines,
    width,
    height,
    0,
    0,
);
self.terminal.resize(cross_size);

let ws_cols = cols.min(u16::MAX as usize) as u16;
let ws_rows = lines.min(u16::MAX as usize) as u16;
let ws_w = width.min(u16::MAX as u32) as u16;
let ws_h = height.min(u16::MAX as u32) as u16;
let resize_info = WinsizeBuilder {
    cols: ws_cols,
    rows: ws_rows,
    width: ws_w,
    height: ws_h,
};
let resize_sequence = format!("\x1b[8;{};{}t", resize_info.rows, resize_info.cols).into_bytes();
let _ = self.pty_tx.send(resize_sequence);
```

5) **Housekeeping**
- Remove or update any comments implying hardcoded defaults (e.g., "should match config", "for now").

Notes:
- We do not change Sugarloaf `RootStyle` initialization; it already uses `config.fonts.size` and `config.line_height`.
- Available sizes are clamped at 0.0 before division; cols/lines are clamped to at least 1 after division to keep the terminal valid under tiny windows.

---

## CODE PATTERN REFERENCES

- **Rio Config with padding**  
  `../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/mod.rs`

- **Default padding values**  
  `../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/defaults.rs`

- **Sugarloaf layout (font metrics used during init)**  
  `../packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/layout/mod.rs`

- **tmp mirrors for reference**  
  `../packages/sweetmcp/packages/sixel6vt/tmp/rio/rio-backend/src/config/mod.rs`  
  `../packages/sweetmcp/packages/sixel6vt/tmp/rio/sugarloaf/src/layout/mod.rs`

---

## DEFINITION OF DONE

- **[struct]** `TerminalPane` includes `font_size`, `line_height`, `padding_x`, `padding_y` fields.
- **[new()]** Calculates `cols`/`lines` using width/height minus padding; guards applied; no placeholder comments.
- **[persist]** Stores config values into the new struct fields in `Ok(Self { ... })`.
- **[resize()]** Uses stored fields; computes with padding; guards applied; no hardcoded font metrics.
- **[search checks]** Grep shows no hardcoded `font_size = 12.0` or `line_height = 1.2` in `terminal/mod.rs`.
- **[build]** Crate compiles successfully from its own directory.

---

## VERIFICATION (no tests; quick manual checks)

From the crate directory `../packages/sweetmcp/packages/sixel6vt/packages/sixel6vt`:

```bash
# Confirm padding-aware vars used
rg "padding_w|padding_h|avail_w|avail_h" src/components/terminal/mod.rs

# Confirm no hardcoded metrics remain
rg "font_size = 12.0|line_height = 1.2" src/components/terminal/mod.rs -n

# Build
cargo check
```

Optional runtime sanity: resize the window with non-zero `padding-x` and `padding-y` in Rio config and confirm the grid reduces accordingly (no text in padded margins).

---

## OUT OF SCOPE

- No new unit tests, functional tests, benchmarks, or extensive documentation.
- No scope changes beyond padding-aware grid sizing using existing config and renderer wiring.
