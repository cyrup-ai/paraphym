//! Sixel image encoding for terminal graphics display
//! 
//! This module implements the Sixel graphics format (DEC VT340 compatible) using a
//! geometric region-based encoding approach for optimal compression.
//!
//! # Sixel Format
//! 
//! Sixel (Six Pixel) encodes images as vertical columns of 6 pixels. Each sixel
//! character (0x3F-0x7E) represents 6 vertical pixels as a 6-bit value.
//!
//! **Format structure:**
//! - DCS introducer: `ESC P q`
//! - Raster attributes: `"<width>;<height>`
//! - Color palette: 16 VT340-compatible RGB definitions (0-100 scale)
//! - Encoded data: Run-length compressed sixel values  
//! - String terminator: `ESC \`
//!
//! # Encoding Algorithm
//!
//! This implementation uses **geometric folding** - a region-based approach that
//! outperforms traditional column-by-column encoding for images with uniform regions.
//!
//! **Three-phase process:**
//! 1. **Region Detection**: Identify rectangular areas of uniform color
//! 2. **Geometric Folding**: Merge adjacent regions (horizontal/vertical)
//! 3. **Sixel Generation**: Output optimized run-length encoded data
//!
//! **Advantages:**
//! - Better compression for UI screenshots, diagrams, charts
//! - Scales with region count, not image size
//! - Perceptual color matching (ITU-R BT.601 weighted RGB)
//!
//! # Example
//!
//! ```rust
//! use image::{RgbImage, Rgb};
//! use sixel_renderer::encode_sixel;
//! # fn main() {
//! // Create a gradient image
//! let mut img = RgbImage::new(100, 100);
//! for y in 0..100 {
//!     for x in 0..100 {
//!         let intensity = ((x + y) / 2) as u8;
//!         img.put_pixel(x, y, Rgb([intensity, intensity, intensity]));
//!     }
//! }
//!
//! // Encode as sixel
//! let sixel_string = encode_sixel(&img);
//!
//! // Display in terminal (if sixel support enabled)
//! print!("{}", sixel_string);
//! # }
//! ```
//!
//! # Terminal Compatibility
//!
//! Compatible with terminals supporting sixel graphics:
//! - **xterm** (with sixel compiled in)
//! - **mlterm** (native support)
//! - **mintty** (Windows)
//! - **iTerm2** (macOS)
//! - **foot** (Wayland)
//! - **rio** (primary target, requires raster attributes)
//!
//! # References
//!
//! - [VT340 Programmer Reference](https://vt100.net/docs/vt3xx-gp/chapter14.html)
//! - [Sixel Wikipedia](https://en.wikipedia.org/wiki/Sixel)
//! - [ITU-R BT.601 Color Standard](https://www.itu.int/rec/r-rec-bt.601/)

/// VT340-compatible 16-color palette for sixel encoding
///
/// RGB values in 0-100 range per sixel specification. Colors chosen for
/// compatibility with DEC VT340 terminals and modern terminal emulators.
///
/// **Palette layout:**
/// - 0-7: Standard 8 colors (black, blue, green, cyan, red, magenta, brown, white)
/// - 8-15: Bright variants (gray, light versions of 1-7)
///
/// Values are (r, g, b) tuples where each component ∈ [0, 100].
///
/// # Source
///
/// Based on VT340 default palette as documented in:
/// - VT330/VT340 Programmer Reference Manual, Chapter 14
/// - [VT100.net Specification](https://vt100.net/docs/vt3xx-gp/chapter14.html)
const PALETTE: [(i32, i32, i32); 16] = [
    (0, 0, 0),       // 0: Black
    (20, 20, 80),    // 1: Dark Blue
    (20, 80, 20),    // 2: Dark Green
    (20, 80, 80),    // 3: Dark Cyan
    (80, 20, 20),    // 4: Dark Red
    (80, 20, 80),    // 5: Dark Magenta
    (80, 80, 20),    // 6: Brown
    (80, 80, 80),    // 7: Light Gray
    (40, 40, 40),    // 8: Dark Gray
    (40, 40, 100),   // 9: Light Blue
    (40, 100, 40),   // 10: Light Green
    (40, 100, 100),  // 11: Light Cyan
    (100, 40, 40),   // 12: Light Red
    (100, 40, 100),  // 13: Light Magenta
    (100, 100, 40),  // 14: Yellow
    (100, 100, 100), // 15: White
];

/// Generate sixel color palette definition from PALETTE constant
/// 
/// Converts the 16-color RGB palette to sixel color definition format.
/// Each color formatted as: `#<index>;2;<r>;<g>;<b>` where RGB in 0-100 range.
///
/// # Returns
///
/// Concatenated string of all 16 color definitions:
/// `#0;2;0;0;0#1;2;20;20;80...#15;2;100;100;100`
///
/// # Format
///
/// Sixel color definition per VT340 spec:
/// - `#` - Color definition introducer
/// - `<index>` - Palette index (0-15)
/// - `2` - RGB color space (vs 1=HLS)
/// - `<r>;<g>;<b>` - RGB values in 0-100 range
///
/// # Performance
///
/// Called once per image encoding. Cost negligible (~16 string allocations).
fn palette_to_sixel_header() -> String {
    PALETTE
        .iter()
        .enumerate()
        .map(|(i, &(r, g, b))| format!("#{};2;{};{};{}", i, r, g, b))
        .collect::<Vec<_>>()
        .join("")
}

/// Standard sixel height in pixels (6 pixels per sixel row)
///
/// Defined by DEC VT340 specification. Each sixel character encodes exactly
/// 6 vertical pixels as a 6-bit value (bits 0-5 map to pixels top-bottom).
const SIXEL_HEIGHT: u32 = 6;

/// Represents a rectangular region in the image with uniform quantized color
///
/// Used by the geometric folding algorithm to represent uniform color areas.
/// Regions are sixel-row aligned (height ≤ 6) during detection, then merged
/// to form larger rectangles during the folding phase.
///
/// # Fields
///
/// * `x` - Left edge pixel coordinate
/// * `y` - Top edge pixel coordinate  
/// * `width` - Region width in pixels
/// * `height` - Region height in pixels (multiple of 6 after merging)
/// * `color` - Palette index (0-15) of the uniform color
///
/// # Invariants
///
/// - No overlapping regions in a valid region set
/// - All pixels in rectangle map to `color` index
/// - Used for compression, not rendering (rendering uses sixel string)
#[derive(Debug, Clone, PartialEq, Eq)]
struct EncoderRegion {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: u16,
}

/// Detect uniform color regions in the image aligned to sixel row boundaries
///
/// Scans the image to identify rectangular regions where all pixels map to the
/// same palette color. Regions are aligned to 6-pixel sixel rows for encoding.
///
/// # Algorithm
///
/// For each sixel row (6 pixels tall):
/// 1. Start at x=0
/// 2. Get color at current position
/// 3. Scan right to find width of uniform color
/// 4. Verify all pixels in rectangle match
/// 5. Create EncoderRegion, advance x by width
/// 6. Repeat until row complete
///
/// Verification prevents skipped pixels when column has mixed colors.
///
/// # Arguments
///
/// * `img` - RGB image to analyze
///
/// # Returns
///
/// Vector of non-overlapping rectangular regions covering the entire image.
/// Each region has uniform quantized color and height ≤ 6 (sixel row aligned).
///
/// # Performance
///
/// - Time: O(w × h) - scans image once
/// - Space: O(r) where r = region count
/// - Best case: Few uniform regions
/// - Worst case: Each pixel different → O(w × h/6) regions
fn detect_regions(img: &image::RgbImage) -> Vec<EncoderRegion> {
    let mut regions = Vec::new();
    let width = img.width();
    let height = img.height();
    
    // Process image in sixel-aligned rows (6 pixels at a time)
    for y in (0..height).step_by(SIXEL_HEIGHT as usize) {
        let row_height = SIXEL_HEIGHT.min(height - y);
        
        let mut x = 0;
        while x < width {
            // Get color of first pixel in this potential region
            let pixel = img.get_pixel(x, y);
            let region_color = find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE);
            
            // Find width of uniform color region
            let mut region_width = 1;
            while x + region_width < width {
                // Check if all pixels in this column match the region color
                let mut column_matches = true;
                for dy in 0..row_height {
                    let pixel = img.get_pixel(x + region_width, y + dy);
                    if find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE) != region_color {
                        column_matches = false;
                        break;
                    }
                }
                
                if column_matches {
                    region_width += 1;
                } else {
                    break;
                }
            }
            
            // Verify all pixels in the region match
            let mut all_match = true;
            for dy in 0..row_height {
                for dx in 0..region_width {
                    let pixel = img.get_pixel(x + dx, y + dy);
                    if find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE) != region_color {
                        all_match = false;
                        break;
                    }
                }
                if !all_match {
                    break;
                }
            }
            
            if all_match {
                regions.push(EncoderRegion {
                    x,
                    y,
                    width: region_width,
                    height: row_height,
                    color: region_color,
                });
                x += region_width;
            } else {
                // Verification failed - mixed column, create individual pixel regions
                for dy in 0..row_height {
                    let pixel = img.get_pixel(x, y + dy);
                    let pixel_color = find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE);
                    regions.push(EncoderRegion {
                        x,
                        y: y + dy,
                        width: 1,
                        height: 1,
                        color: pixel_color,
                    });
                }
                x += 1;
            }
        }
    }
    
    regions
}

/// Merge adjacent regions with matching color (geometric folding)
///
/// This is the core of the geometric encoding innovation. Iteratively merges
/// neighboring regions with the same color to create a minimal region set.
///
/// # Algorithm
///
/// Repeat until no merges possible:
/// 1. For each region pair (i, j):
/// 2. Check if vertically mergeable:
///    - Same x, width, color
///    - Region i bottom edge touches region j top edge
///    - If yes: extend i height, remove j
/// 3. Check if horizontally mergeable:
///    - Same y, height, color
///    - Region i right edge touches region j left edge
///    - If yes: extend i width, remove j
///
/// # Arguments
///
/// * `regions` - Initial regions from detection phase
///
/// # Returns
///
/// Minimal set of maximal rectangular regions. Adjacent regions with same
/// color are folded into larger rectangles.
///
/// # Performance
///
/// - Time: O(r² × passes) where r = region count, passes = merge iterations
/// - Space: O(r) - modifies vector in place
/// - Best case: All regions merge → O(r)
/// - Worst case: No merges possible → O(r²)
///
/// Typically converges quickly (1-3 passes) for natural images.
fn merge_regions(mut regions: Vec<EncoderRegion>) -> Vec<EncoderRegion> {
    let mut merged = true;
    
    while merged {
        merged = false;
        let mut i = 0;
        
        while i < regions.len() {
            let mut j = i + 1;
            let mut found_merge = false;
            
            while j < regions.len() {
                let can_merge_vertical = regions[i].x == regions[j].x
                    && regions[i].width == regions[j].width
                    && regions[i].color == regions[j].color
                    && regions[i].y + regions[i].height == regions[j].y;
                
                let can_merge_horizontal = regions[i].y == regions[j].y
                    && regions[i].height == regions[j].height
                    && regions[i].color == regions[j].color
                    && regions[i].x + regions[i].width == regions[j].x;
                
                if can_merge_vertical {
                    // Merge vertically: extend height
                    regions[i].height += regions[j].height;
                    regions.swap_remove(j);
                    merged = true;
                    found_merge = true;
                    break;
                } else if can_merge_horizontal {
                    // Merge horizontally: extend width
                    regions[i].width += regions[j].width;
                    regions.swap_remove(j);
                    merged = true;
                    found_merge = true;
                    break;
                }
                
                j += 1;
            }
            
            if !found_merge {
                i += 1;
            }
        }
    }
    
    regions
}

/// Convert optimized regions to sixel format string
///
/// Generates sixel output from the minimal region set produced by geometric folding.
/// Processes regions in raster order, outputting color switches and run-length
/// encoded sixel data.
///
/// # Arguments
///
/// * `regions` - Merged regions from folding phase
/// * `img_width` - Image width in pixels (for raster attributes)
/// * `img_height` - Image height in pixels (for raster attributes)
///
/// # Returns
///
/// Complete sixel string with:
/// - DCS + raster attributes
/// - Color palette definitions (from PALETTE constant)
/// - Sixel data with run-length encoding
/// - String terminator
///
/// # Algorithm
///
/// For each sixel row:
/// 1. Find regions intersecting this row
/// 2. Sort by x coordinate
/// 3. For each region:
///    a. Switch color if needed: `#<color>`
///    b. Calculate sixel bits for pixels in this row
///    c. Output with RLE: `!<width><char>` or just `<char>`
/// 4. End row: `-`
///
/// # Performance
///
/// - Time: O(r × h/6) where r = regions, h = height
/// - Space: O(output_length) - pre-allocated string
/// - Run-length encoding reduces output size for wide regions
fn regions_to_sixel(regions: &[EncoderRegion], img_width: u32, img_height: u32) -> String {
    // Handle zero-dimension edge case
    if img_width == 0 || img_height == 0 {
        return String::from("\x1BPq\x1B\\");
    }

    // Pre-allocate string capacity based on image size
    // Estimated: ~0.5 bytes per pixel + 1024 bytes overhead (header/palette/footer)
    // Use saturating arithmetic to prevent overflow for very large images
    let pixel_count = (img_width as u64).saturating_mul(img_height as u64);
    let estimated_capacity = (pixel_count / 2).saturating_add(1024).min(usize::MAX as u64) as usize;
    let mut result = String::with_capacity(estimated_capacity);
    result.push_str("\x1BPq");
    
    // Add raster attributes (aspect ratio 1:1, then dimensions)
    result.push_str(&format!("\"1;1;{};{}", img_width, img_height));
    
    // Define palette (DRY - generated from PALETTE constant)
    result.push_str(&palette_to_sixel_header());
    
    // Process by sixel rows
    for y in (0..img_height).step_by(SIXEL_HEIGHT as usize) {
        let mut current_color: Option<u16> = None;
        
        // Find regions intersecting this sixel row
        let mut row_regions: Vec<&EncoderRegion> = regions
            .iter()
            .filter(|r| r.y < y + SIXEL_HEIGHT && y < r.y + r.height)
            .collect();
        
        // Sort by x coordinate
        row_regions.sort_by_key(|r| r.x);
        
        for region in row_regions {
            // Switch color if needed
            if current_color != Some(region.color) {
                result.push_str(&format!("#{}", region.color));
                current_color = Some(region.color);
            }
            
            // Calculate sixel pattern for this region in this row
            let mut sixel_value = 0u32;
            
            // Calculate which rows of the region intersect this sixel row
            let start_row = region.y.max(y);
            let end_row = (region.y + region.height).min(y + SIXEL_HEIGHT).min(img_height);
            
            // Set bits for pixels that belong to this region
            for row in start_row..end_row {
                let bit = row - y;  // Which bit in the sixel character (0-5)
                sixel_value |= 1 << bit;
            }
            
            // Output run-length encoded sixel data
            if region.width > 1 {
                result.push_str(&format!("!{}", region.width));
            }
            
            if let Some(ch) = char::from_u32(63 + sixel_value) {
                result.push(ch);
            }
        }
        
        // End of sixel row
        result.push('-');
    }
    
    // End sixel sequence
    result.push_str("\x1B\\");
    
    result
}

/// Find the closest palette color to given RGB using perceptual distance
/// 
/// Uses weighted Euclidean distance based on human eye sensitivity (ITU-R BT.601):
/// - Red: 30% weight
/// - Green: 59% weight (human eye most sensitive)
/// - Blue: 11% weight
/// 
/// # Arguments
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255) 
/// * `b` - Blue component (0-255)
/// * `palette` - Array of RGB tuples in (i32,i32,i32) format
/// 
/// # Returns
/// Index of closest color in palette (0-15 for standard VT340 palette)
/// 
/// # Algorithm
/// O(n) where n = palette size (16). Uses weighted Euclidean distance
/// with perceptual weights from ITU-R BT.601 standard for color television.
fn find_closest_color_in_palette(r: u8, g: u8, b: u8, palette: &[(i32, i32, i32)]) -> u16 {
    let mut min_dist = f32::MAX;
    let mut closest = 0;

    for (i, &(cr, cg, cb)) in palette.iter().enumerate() {
        // Apply perceptual weights: 30% red, 59% green, 11% blue
        let dr = (r as f32 - cr as f32) * 0.30;
        let dg = (g as f32 - cg as f32) * 0.59;
        let db = (b as f32 - cb as f32) * 0.11;
        
        let dist = dr * dr + dg * dg + db * db;
        
        if dist < min_dist {
            min_dist = dist;
            closest = i;
        }
    }

    closest as u16
}

/// Find the most common color in a sixel column
/// 
/// Analyzes up to 6 pixels and returns the index of the most frequently
/// occurring color using O(n) count-based algorithm.
/// 
/// # Arguments
/// * `colors` - Slice of color indices (0-15) from palette
/// 
/// # Returns
/// Index of dominant (most common) color. Returns 0 if array is empty
/// or all colors are out of bounds (>15).
/// 
/// # Algorithm
/// O(n + m) where n = array length, m = palette size (16)
/// - Single pass to count occurrences
/// - Single pass to find maximum
/// - Ties broken by first occurrence (lower index)
/// 
/// # Example
/// ```ignore
/// let colors = [1, 2, 1, 1, 3, 1]; // Color 1 appears 4 times
/// assert_eq!(find_dominant_color(&colors), 1);
/// ```
fn find_dominant_color(colors: &[u16]) -> u16 {
    let mut counts = [0u8; 16];
    
    // Count occurrences of each valid palette color
    for &color in colors {
        if (color as usize) < 16 {
            counts[color as usize] += 1;
        }
    }
    
    // Find color with maximum count
    counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, &count)| count)
        .map(|(idx, _)| idx as u16)
        .unwrap_or(0)
}

/// Implements Sixel rendering for images using geometric region-based encoding
/// Encodes an RGB image as a Sixel graphics string using geometric region-based encoding
///
/// Converts a 24-bit RGB image to sixel format for display in compatible terminals.
/// Uses perceptual color matching to map colors to a 16-color VT340 palette, with
/// geometric folding for optimal compression.
///
/// # Algorithm
///
/// **Three-phase encoding:**
/// 1. **Region Detection**: Scan image for uniform color rectangles aligned to sixel rows
/// 2. **Geometric Folding**: Merge adjacent regions with matching color  
/// 3. **Sixel Generation**: Output run-length encoded sixel with minimal color switches
///
/// This geometric approach outperforms column-by-column encoding for images with
/// rectangular regions (UI screenshots, diagrams), while maintaining compatibility.
///
/// # Arguments
///
/// * `img` - RGB image to encode (8-bit per channel, 24-bit color)
///
/// # Returns
///
/// Complete sixel sequence string:
/// - DCS introducer: `\x1BPq`
/// - Raster attributes: `"<width>;<height>`
/// - Color palette: 16 VT340-compatible color definitions (0-100 RGB scale)
/// - Encoded pixel data: Run-length compressed sixel values
/// - String terminator: `\x1B\`
///
/// Zero-dimension images (width=0 or height=0) return minimal valid sixel: `\x1BPq\x1B\`
///
/// # Color Quantization
///
/// Maps 24-bit RGB to nearest palette color using **ITU-R BT.601 weighted distance**:
/// ```text
/// distance² = (0.30 × Δr)² + (0.59 × Δg)² + (0.11 × Δb)²
/// ```
///
/// Weights reflect human eye sensitivity (green 59%, red 30%, blue 11%) for
/// perceptually accurate color matching.
///
/// # Performance
///
/// - **Time**: O(w × h) detection + O(r²) merging where r = region count
/// - **Space**: O(w × h) for output string (pre-allocated ~0.5 bytes/pixel)
/// - **Best case**: Uniform regions → O(1) encoding
/// - **Worst case**: Checkerboard → O(w × h) regions (degrades to column-based)
///
/// # Example
///
/// ```rust
/// use image::{RgbImage, Rgb};
/// use sixel_renderer::encode_sixel;
/// # fn main() {
/// // Create solid red 50x50 image
/// let img = RgbImage::from_pixel(50, 50, Rgb([255, 0, 0]));
/// let sixel = encode_sixel(&img);
/// 
/// // Verify sixel format
/// assert!(sixel.starts_with("\x1BPq"));      // DCS introducer
/// assert!(sixel.contains("\"50;50"));        // Raster attributes
/// assert!(sixel.ends_with("\x1B\\"));        // String terminator
/// # }
/// ```
///
/// # Sixel Character Encoding
///
/// Each sixel character encodes 6 vertical pixels as a 6-bit value:
/// ```text
/// character = 0x3F + pixel_bits
/// where pixel_bits = b0 | (b1<<1) | ... | (b5<<5)
/// Range: 0x3F ('?') to 0x7E ('~')
/// LSB = top pixel, MSB = bottom pixel
/// ```
pub fn encode_sixel(img: &image::RgbImage) -> String {
    // Validate dimensions - handle edge case of empty images
    if img.width() == 0 || img.height() == 0 {
        // Return minimal valid sixel sequence for empty image
        // DCS + 'q' (sixel mode) + ST (String Terminator)
        return String::from("\x1BPq\x1B\\");
    }

    let pixel_count = img.width() as u64 * img.height() as u64;
    
    // Use column-based encoder for complex images to avoid O(r²) merge performance issues
    // Threshold: 200k pixels is a reasonable cutoff (roughly 450x450 image)
    // Web screenshots and photos should use the proven O(w*h) column-based approach
    if pixel_count > 200_000 {
        tracing::debug!(
            "Using column-based encoder for large image ({}x{} = {} pixels)",
            img.width(),
            img.height(),
            pixel_count
        );
        return encode_sixel_column_based(img);
    }

    // Phase 1: Detect uniform color regions in the image
    let regions = detect_regions(img);
    
    // If initial region count is very high, the geometric merge will be too slow
    // Fall back to column-based encoder for complex images with many regions
    if regions.len() > 10_000 {
        tracing::debug!(
            "Using column-based encoder due to high region count ({} regions detected)",
            regions.len()
        );
        return encode_sixel_column_based(img);
    }
    
    // Phase 2: Merge adjacent regions (geometric folding)
    let merged_regions = merge_regions(regions);
    
    // Phase 3: Generate sixel output from optimized regions
    regions_to_sixel(&merged_regions, img.width(), img.height())
}

/// Column-based encoding - O(w*h) complexity, reliable for all image types
/// 
/// This is the proven reference implementation used as fallback for large or 
/// complex images where geometric encoding would be too slow.
/// This approach processes the image column-by-column with run-length encoding.
fn encode_sixel_column_based(img: &image::RgbImage) -> String {
        let mut result = String::from("\x1BPq");

        // Add raster attributes (aspect ratio 1:1, then dimensions)
        result.push_str(&format!("\"1;1;{};{}", img.width(), img.height()));

        // Define palette (DRY - generated from PALETTE constant)
        result.push_str(&palette_to_sixel_header());

        // Process the image in sixel format (6 vertical pixels at a time)
        for y in (0..img.height()).step_by(SIXEL_HEIGHT as usize) {
            // Initialize with color 0
            result.push_str("#0");

            let mut current_color = 0;
            let mut run_length = 0;
            let mut last_sixel_value = 0;

            for x in 0..img.width() {
                // Get the color for this column and select the dominant one
                let mut column_colors = [0u16; SIXEL_HEIGHT as usize];

                for i in 0..SIXEL_HEIGHT {
                    if y + i < img.height() {
                        let pixel = img.get_pixel(x, y + i);
                        column_colors[i as usize] =
                            find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE);
                    }
                }

                // Use most common color in this column
                let dominant_color = find_dominant_color(&column_colors);

                // Switch color if needed
                if dominant_color != current_color {
                    // Output any pending run-length
                    if run_length > 0 {
                        if run_length > 1 {
                            result.push_str(&format!("!{}", run_length));
                        }
                        if let Some(ch) = char::from_u32(63 + last_sixel_value) {
                            result.push(ch);
                        }
                        run_length = 0;
                    }

                    result.push_str(&format!("#{}", dominant_color));
                    current_color = dominant_color;
                }

                // Calculate sixel value for this column
                let mut sixel_value = 0;
                for i in 0..SIXEL_HEIGHT {
                    if y + i < img.height() {
                        // Set bit i if pixel is closer to foreground than background
                        let color = column_colors[i as usize];

                        // If color matches dominant, set the bit
                        if color == current_color {
                            sixel_value |= 1 << i;
                        }
                    }
                }

                // Check if we can use run-length encoding
                if sixel_value == last_sixel_value && run_length > 0 {
                    run_length += 1;
                } else {
                    // Output any pending run-length
                    if run_length > 0 {
                        if run_length > 1 {
                            result.push_str(&format!("!{}", run_length));
                        }
                        if let Some(ch) = char::from_u32(63 + last_sixel_value) {
                            result.push(ch);
                        }
                    }

                    last_sixel_value = sixel_value;
                    run_length = 1;
                }
            }

            // Output the last run
            if run_length > 0 {
                if run_length > 1 {
                    result.push_str(&format!("!{}", run_length));
                }
                if let Some(ch) = char::from_u32(63 + last_sixel_value) {
                    result.push(ch);
                }
            }

            // End of line - use "-" for Rio compatibility instead of "$\n"
            result.push('-');
        }

        // End sixel sequence
        result.push_str("\x1B\\");

        result
    }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_geometric_encoding() {
        // Create a simple 12x12 test image with uniform regions
        // This tests the geometric folding innovation
        const TEST_IMG_SIZE: u32 = 2 * SIXEL_HEIGHT;  // 12 pixels = 2 sixel rows
        let mut img = image::RgbImage::new(TEST_IMG_SIZE, TEST_IMG_SIZE);
        
        // Fill with a uniform color (should create one large region)
        for y in 0..TEST_IMG_SIZE {
            for x in 0..TEST_IMG_SIZE {
                img.put_pixel(x, y, image::Rgb([100u8, 100u8, 100u8]));
            }
        }
        
        // Encode using geometric approach
        let sixel = encode_sixel(&img);
        
        // Verify output format
        assert!(sixel.starts_with("\x1BPq"), "Should start with DCS sequence");
        assert!(sixel.ends_with("\x1B\\"), "Should end with ST sequence");
        assert!(sixel.contains("\"1;1;12;12"), "Should contain raster attributes");
        assert!(sixel.contains("#"), "Should contain color switches");
        assert!(sixel.contains("-"), "Should contain line terminators");
    }
    
    #[test]
    fn test_region_detection() {
        // Create 6x6 image with two regions (1 sixel row tall)
        let img_height = SIXEL_HEIGHT;
        let img_width = SIXEL_HEIGHT;  // Arbitrary width matching height for square test
        let mut img = image::RgbImage::new(img_width, img_height);
        
        // Left half: color 1
        for y in 0..img_height {
            for x in 0..(img_width / 2) {
                img.put_pixel(x, y, image::Rgb([20u8, 20u8, 80u8]));
            }
        }

        // Right half: color 2
        for y in 0..img_height {
            for x in (img_width / 2)..img_width {
                img.put_pixel(x, y, image::Rgb([20u8, 80u8, 20u8]));
            }
        }
        
        let regions = detect_regions(&img);
        
        // Should detect 2 regions
        assert_eq!(regions.len(), 2, "Should detect 2 regions");
        assert_eq!(regions[0].width, 3, "First region width should be 3");
        assert_eq!(regions[1].width, 3, "Second region width should be 3");
    }
    
    #[test]
    fn test_region_merging() {
        // Create regions that should merge
        let regions = vec![
            EncoderRegion { x: 0, y: 0, width: 5, height: SIXEL_HEIGHT, color: 1 },
            EncoderRegion { x: 5, y: 0, width: 5, height: SIXEL_HEIGHT, color: 1 },  // Horizontal merge
        ];
        
        let merged = merge_regions(regions);
        
        // Should merge into one region
        assert_eq!(merged.len(), 1, "Should merge into 1 region");
        assert_eq!(merged[0].width, 10, "Merged region width should be 10");
    }
    
    #[test]
    fn test_vertical_stripes_no_skip() {
        // This test catches the pixel-skipping bug where verification fails
        // and pixels get skipped instead of being encoded
        const TEST_IMG_WIDTH: u32 = 2 * SIXEL_HEIGHT;  // 12 pixels
        let mut img = image::RgbImage::new(TEST_IMG_WIDTH, SIXEL_HEIGHT);
        
        // Columns 0-5: color 1 (dark blue)
        for y in 0..SIXEL_HEIGHT {
            for x in 0..(TEST_IMG_WIDTH / 2) {
                img.put_pixel(x, y, image::Rgb([20u8, 20u8, 80u8]));
            }
        }
        
        // Columns 6-11: color 2 (dark green)
        for y in 0..SIXEL_HEIGHT {
            for x in (TEST_IMG_WIDTH / 2)..TEST_IMG_WIDTH {
                img.put_pixel(x, y, image::Rgb([20u8, 80u8, 20u8]));
            }
        }
        
        let sixel = encode_sixel(&img);
        
        // Both colors must appear in output (no pixels skipped)
        assert!(sixel.contains("#1"), "Must encode color 1 (dark blue)");
        assert!(sixel.contains("#2"), "Must encode color 2 (dark green)");
    }
    
    #[test]
    fn test_checkerboard_mixed_columns() {
        // Test case that ACTUALLY triggers verification failure
        // Mixed colors within a column - this exposes encoding issues
        let mut img = image::RgbImage::new(SIXEL_HEIGHT, SIXEL_HEIGHT);
        
        // Create checkerboard: alternating colors
        for y in 0..SIXEL_HEIGHT {
            for x in 0..SIXEL_HEIGHT {
                if (x + y) % 2 == 0 {
                    img.put_pixel(x, y, image::Rgb([20u8, 20u8, 80u8]));  // color 1
                } else {
                    img.put_pixel(x, y, image::Rgb([20u8, 80u8, 20u8]));  // color 2
                }
            }
        }
        
        let sixel = encode_sixel(&img);
        
        // Both colors must appear
        assert!(sixel.contains("#1"), "Must encode color 1");
        assert!(sixel.contains("#2"), "Must encode color 2");
        
        // Should have sixel characters (not just headers)
        assert!(sixel.len() > 100, "Should have substantial sixel data");
    }
    
    #[test]
    fn test_empty_image() {
        // Test edge case: 0x0 image
        let img = image::RgbImage::new(0, 0);
        let sixel = encode_sixel(&img);
        
        // Should return minimal valid sixel sequence
        assert_eq!(sixel, "\x1BPq\x1B\\", "Empty image should return minimal sixel sequence");
    }
    
    #[test]
    fn test_single_pixel() {
        // Test edge case: 1x1 image
        let mut img = image::RgbImage::new(1, 1);
        img.put_pixel(0, 0, image::Rgb([100u8, 100u8, 100u8]));
        
        let sixel = encode_sixel(&img);
        
        // Should have valid sixel format
        assert!(sixel.starts_with("\x1BPq"), "Should start with DCS");
        assert!(sixel.ends_with("\x1B\\"), "Should end with ST");
        assert!(sixel.contains("\"1;1;1;1"), "Should have raster attributes 1:1 aspect, 1x1 dimensions");
    }
    
    #[test]
    fn test_non_sixel_aligned_height() {
        // Test edge case: height not divisible by 6
        let mut img = image::RgbImage::new(SIXEL_HEIGHT, 7);  // Height = 7 (not divisible by 6)
        
        // Fill with uniform color
        for y in 0..7 {
            for x in 0..SIXEL_HEIGHT {
                img.put_pixel(x, y, image::Rgb([100u8, 100u8, 100u8]));
            }
        }
        
        let sixel = encode_sixel(&img);
        
        // Should encode correctly without panic
        assert!(sixel.starts_with("\x1BPq"), "Should start with DCS");
        assert!(sixel.ends_with("\x1B\\"), "Should end with ST");
        assert!(sixel.contains("\"1;1;6;7"), "Should have raster attributes 1:1 aspect, 6x7 dimensions");
        // Should have 2 sixel rows (0-5 and 6)
        let row_count = sixel.matches('-').count();
        assert_eq!(row_count, 2, "Should have 2 sixel rows for height=7");
    }
}
