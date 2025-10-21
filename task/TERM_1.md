# TERM_1: Implement Terminal Dimension Padding Support

**Prefix:** TERM  
**Severity:** MEDIUM  
**Priority:** MEDIUM (Within 2 sprints)  
**Estimated Effort:** Single focused session (60-120 minutes)

---

## OBJECTIVE

Fix the incomplete terminal dimension calculation in `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs` to properly account for padding values, ensuring accurate terminal grid sizing.

**Why this matters:** The current simplified calculation ignores padding, causing rendering inconsistencies between calculated terminal dimensions and actual available rendering space. This results in text clipping or incorrect cursor positioning.

---

## RESEARCH FINDINGS

### Config Structure (Rio Backend)

**Source:** [`vendor/rio/rio-backend/src/config/mod.rs:130-133`](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/mod.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    // ... other fields ...
    #[serde(rename = "padding-x", default = "f32::default")]
    pub padding_x: f32,
    #[serde(rename = "padding-y", default = "default_padding_y")]
    pub padding_y: [f32; 2],
    // ... other fields ...
}
```

**Padding Field Details:**
- `padding_x: f32` - Horizontal padding in pixels, applied to BOTH left and right sides
- `padding_y: [f32; 2]` - Vertical padding in pixels as `[top, bottom]` array
- Default values from [`vendor/rio/rio-backend/src/config/defaults.rs:29-31`](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/defaults.rs):
  ```rust
  fn default_padding_y() -> [f32; 2] {
      [0., 0.]
  }
  ```

### Problem Areas Identified

#### Problem 1: new() Function (Lines 188-194)

**Current Code:**
```rust
// Calculate terminal dimensions manually
// Note: This is a simplified calculation. A real implementation
// might need to account for padding_x/padding_y more precisely
// depending on how Sugarloaf handles it internally.
let cols = (width / config.fonts.size).floor() as usize;
let lines = (height / (config.fonts.size * config.line_height)).floor() as usize;
let terminal_size = (cols, lines); // Store as a tuple for now
```

**Issues:**
- Ignores `config.padding_x` for width calculation
- Ignores `config.padding_y` for height calculation
- Comment indicates this is temporary ("for now")
- Will cause text to render beyond intended boundaries

#### Problem 2: resize() Function (Lines 633-642)

**Current Code:**
```rust
pub fn resize(&mut self, width: u32, height: u32) {
    let dpr = self.window.scale_factor();
    let _scale = dpr as f32; // Keep but unused
    self.sugarloaf.resize(width, height);
    // Resize the terminal - needs a type implementing Dimensions
    // Calculate terminal dimensions based on window size
    // Use configuration values from Rio for accurate calculations
    // This avoids accessing private sugarloaf methods
    let font_size = 12.0; // Default font size, should match config
    let line_height = 1.2; // Default line height, should match config
    
    let cols = (width as f32 / font_size).floor() as usize;
    let lines = (height as f32 / (font_size * line_height)).floor() as usize;
```

**Issues:**
- **CRITICAL:** Uses hardcoded `font_size = 12.0` instead of actual config value
- **CRITICAL:** Uses hardcoded `line_height = 1.2` instead of actual config value  
- Ignores padding completely (no padding_x or padding_y)
- Comments acknowledge values "should match config" but don't
- resize() has no access to config object
- Will desync with new() if config uses different font size or line height

---

## SOLUTION APPROACH

### Step 1: Store Config Values in TerminalPane Struct

**File:** `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs`

**Current struct (lines 38-50):**
```rust
pub struct TerminalPane<U: rio_backend::event::EventListener + Clone + Send + 'static> {
    pub window: Window,
    pub terminal: Terminal<U>,
    pub pty_tx: mpsc::Sender<Vec<u8>>,
    pub event_proxy: U,
    pub clipboard: Rc<RefCell<Clipboard>>,
    pub sugarloaf: Sugarloaf<'static>,
    parser: Processor,
    
    // Thread management resources
    running: Arc<AtomicBool>,
    reader_thread: Option<JoinHandle<()>>,
    writer_thread: Option<JoinHandle<()>>,
}
```

**Add new fields:**
```rust
pub struct TerminalPane<U: rio_backend::event::EventListener + Clone + Send + 'static> {
    pub window: Window,
    pub terminal: Terminal<U>,
    pub pty_tx: mpsc::Sender<Vec<u8>>,
    pub event_proxy: U,
    pub clipboard: Rc<RefCell<Clipboard>>,
    pub sugarloaf: Sugarloaf<'static>,
    parser: Processor,
    
    // Configuration values needed for resize calculations
    font_size: f32,
    line_height: f32,
    padding_x: f32,
    padding_y: [f32; 2],
    
    // Thread management resources
    running: Arc<AtomicBool>,
    reader_thread: Option<JoinHandle<()>>,
    writer_thread: Option<JoinHandle<()>>,
}
```

### Step 2: Fix new() Function - Store Config and Calculate with Padding

**Location:** Lines 188-194

**Replace:**
```rust
// Calculate terminal dimensions manually
// Note: This is a simplified calculation. A real implementation
// might need to account for padding_x/padding_y more precisely
// depending on how Sugarloaf handles it internally.
let cols = (width / config.fonts.size).floor() as usize;
let lines = (height / (config.fonts.size * config.line_height)).floor() as usize;
let terminal_size = (cols, lines); // Store as a tuple for now
```

**With:**
```rust
// Calculate terminal dimensions accounting for padding
// Padding is subtracted from total space before calculating grid dimensions
// padding_x applies to both left and right (total: padding_x * 2)
// padding_y is [top, bottom] array (total: padding_y[0] + padding_y[1])
let available_width = width - (config.padding_x * 2.0);
let available_height = height - (config.padding_y[0] + config.padding_y[1]);

let cols = (available_width / config.fonts.size).floor() as usize;
let lines = (available_height / (config.fonts.size * config.line_height)).floor() as usize;

// Ensure minimum dimensions
let cols = cols.max(1);
let lines = lines.max(1);

let terminal_size = (cols, lines);
```

**Then at end of new() function (around line 470), store config values:**
```rust
Ok(Self {
    window,
    terminal,
    pty_tx,
    event_proxy,
    clipboard: Rc::new(RefCell::new(clipboard)),
    sugarloaf,
    parser: Processor::new(),
    // Store config values for resize calculations
    font_size: config.fonts.size,
    line_height: config.line_height,
    padding_x: config.padding_x,
    padding_y: config.padding_y,
    // Store thread management resources
    running,
    reader_thread: Some(reader_thread),
    writer_thread: Some(writer_thread),
})
```

### Step 3: Fix resize() Function - Use Stored Config and Padding

**Location:** Lines 633-653

**Replace:**
```rust
pub fn resize(&mut self, width: u32, height: u32) {
    let dpr = self.window.scale_factor();
    let _scale = dpr as f32; // Keep but unused
    self.sugarloaf.resize(width, height);
    // Resize the terminal - needs a type implementing Dimensions
    // Calculate terminal dimensions based on window size
    // Use configuration values from Rio for accurate calculations
    // This avoids accessing private sugarloaf methods
    let font_size = 12.0; // Default font size, should match config
    let line_height = 1.2; // Default line height, should match config
    
    let cols = (width as f32 / font_size).floor() as usize;
    let lines = (height as f32 / (font_size * line_height)).floor() as usize;
```

**With:**
```rust
pub fn resize(&mut self, width: u32, height: u32) {
    let dpr = self.window.scale_factor();
    let _scale = dpr as f32; // Keep but unused
    self.sugarloaf.resize(width, height);
    
    // Calculate terminal dimensions accounting for padding
    // Use stored config values to ensure consistency with new()
    let available_width = width as f32 - (self.padding_x * 2.0);
    let available_height = height as f32 - (self.padding_y[0] + self.padding_y[1]);
    
    let cols = (available_width / self.font_size).floor() as usize;
    let lines = (available_height / (self.font_size * self.line_height)).floor() as usize;
    
    // Ensure minimum dimensions
    let cols = cols.max(1);
    let lines = lines.max(1);
```

---

## IMPLEMENTATION CHECKLIST

### Required Changes

1. **Add fields to TerminalPane struct** (around line 38-50):
   - `font_size: f32`
   - `line_height: f32`
   - `padding_x: f32`
   - `padding_y: [f32; 2]`

2. **Fix new() function** (lines 188-194):
   - Calculate `available_width = width - (config.padding_x * 2.0)`
   - Calculate `available_height = height - (config.padding_y[0] + config.padding_y[1])`
   - Use available dimensions for cols/lines calculation
   - Add `.max(1)` guards for minimum dimensions
   - Remove "for now" comment

3. **Store config in new() return** (around line 470):
   - Add `font_size: config.fonts.size`
   - Add `line_height: config.line_height`
   - Add `padding_x: config.padding_x`
   - Add `padding_y: config.padding_y`

4. **Fix resize() function** (lines 633-653):
   - Remove hardcoded `font_size = 12.0` and `line_height = 1.2`
   - Calculate `available_width` using `self.padding_x`
   - Calculate `available_height` using `self.padding_y`
   - Use `self.font_size` and `self.line_height` for calculations
   - Add `.max(1)` guards for minimum dimensions
   - Remove misleading "should match config" comments

---

## PADDING CALCULATION REFERENCE

### Horizontal Padding (padding_x)
```rust
// padding_x is a single value applied to BOTH sides
let total_horizontal_padding = config.padding_x * 2.0;
let available_width = total_width - total_horizontal_padding;
```

### Vertical Padding (padding_y)
```rust
// padding_y is [top, bottom] array
let total_vertical_padding = config.padding_y[0] + config.padding_y[1];
let available_height = total_height - total_vertical_padding;
```

### Complete Example
```rust
// From config:
// padding_x = 10.0 (10px left + 10px right = 20px total)
// padding_y = [5.0, 15.0] (5px top + 15px bottom = 20px total)
// Window: 800x600

let available_width = 800.0 - (10.0 * 2.0);  // = 780.0
let available_height = 600.0 - (5.0 + 15.0);  // = 580.0

// Then calculate grid from available space
let cols = (780.0 / font_size).floor() as usize;
let lines = (580.0 / (font_size * line_height)).floor() as usize;
```

---

## DEFINITION OF DONE

1. ✅ Four new fields added to TerminalPane struct
2. ✅ new() function calculates available_width and available_height before dimension calculation
3. ✅ new() function uses padding-aware calculation for cols and lines
4. ✅ new() function has minimum dimension guards (`.max(1)`)
5. ✅ new() function stores config values in struct
6. ✅ resize() function uses stored config values instead of hardcoded values
7. ✅ resize() function uses padding-aware calculation matching new()
8. ✅ resize() function has minimum dimension guards (`.max(1)`)
9. ✅ All "for now" and "should match config" comments removed or clarified
10. ✅ Code compiles without errors: `cargo check -p sixel6vt`

---

## VERIFICATION STEPS

```bash
# Verify padding is now used in calculations
rg "available_width|available_height" packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs
# Should return matches showing padding calculations

# Verify hardcoded values removed from resize()
rg "font_size = 12.0|line_height = 1.2" packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs
# Should return 0 matches

# Verify struct has new fields
rg "font_size: f32" packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs
# Should show the new struct fields

# Verify code compiles
cd packages/sweetmcp/packages/sixel6vt
cargo check -p sixel6vt
# Should succeed with no errors
```

---

## NOTES

### Why This Matters

Terminal emulators need precise dimension calculations to:
1. Position text correctly within the rendering area
2. Handle cursor positioning accurately
3. Prevent text from rendering in padding areas
4. Maintain consistent sizing between initialization and resize

### Config Source Chain

1. User config: `~/.config/rio/config.toml` (or platform equivalent)
2. Loaded by: `rio-backend/src/config/mod.rs`
3. Used in: `sixel6vt/src/components/terminal/mod.rs`
4. Applied to: Sugarloaf renderer and terminal grid

### Related Files

- Config definition: [`vendor/rio/rio-backend/src/config/mod.rs`](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/mod.rs)
- Config defaults: [`vendor/rio/rio-backend/src/config/defaults.rs`](../packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/config/defaults.rs)
- Terminal implementation: [`packages/sixel6vt/src/components/terminal/mod.rs`](../packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs)
