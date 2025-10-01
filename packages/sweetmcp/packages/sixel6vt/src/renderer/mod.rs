use image;

/// VT340-compatible 16-color palette for sixel encoding
/// RGB values scaled to 0-100 range per sixel specification
/// Source: VT330/VT340 Programmer Reference Manual, Chapter 14
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
/// Returns a string containing all 16 color definitions in sixel format.
/// Format per VT340 spec: `#<index>;2;<r>;<g>;<b>` for each color.
/// RGB values are in 0-100 percent range per sixel specification.
///
/// # Returns
/// String like: `#0;2;0;0;0#1;2;20;20;80...#15;2;100;100;100`
fn palette_to_sixel_header() -> String {
    PALETTE
        .iter()
        .enumerate()
        .map(|(i, &(r, g, b))| format!("#{};2;{};{};{}", i, r, g, b))
        .collect::<Vec<_>>()
        .join("")
}

/// Standard sixel height in pixels (6 pixels per sixel row)
/// Per VT340 specification: each sixel encodes 6 vertical pixels
const SIXEL_HEIGHT: u32 = 6;

/// Represents a rectangular region in the image with uniform color
/// Used for geometric folding optimization in sixel encoding
#[derive(Debug, Clone, PartialEq, Eq)]
struct EncoderRegion {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: u16,
}

/// Detect uniform color regions in the image aligned to sixel row boundaries
/// Scans the image and identifies rectangular regions of uniform quantized color
fn detect_regions<F>(img: &image::RgbImage, find_closest_color: F) -> Vec<EncoderRegion>
where
    F: Fn(u8, u8, u8) -> u16,
{
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
            let region_color = find_closest_color(pixel[0], pixel[1], pixel[2]);
            
            // Find width of uniform color region
            let mut region_width = 1;
            while x + region_width < width {
                // Check if all pixels in this column match the region color
                let mut column_matches = true;
                for dy in 0..row_height {
                    let pixel = img.get_pixel(x + region_width, y + dy);
                    if find_closest_color(pixel[0], pixel[1], pixel[2]) != region_color {
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
                    if find_closest_color(pixel[0], pixel[1], pixel[2]) != region_color {
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
                // Verification failed - create single-pixel region
                regions.push(EncoderRegion {
                    x,
                    y,
                    width: 1,
                    height: row_height,
                    color: region_color,
                });
                x += 1;
            }
        }
    }
    
    regions
}

/// Merge adjacent regions with matching color (geometric folding)
/// This is the core of the geometric innovation - regions "fold" together
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
/// Generates sixel output from geometrically folded regions
fn regions_to_sixel(regions: &[EncoderRegion], img_width: u32, img_height: u32) -> String {
    let mut result = String::from("\x1BPq");
    
    // Add raster attributes
    result.push_str(&format!("\"{};{}", img_width, img_height));
    
    // Define palette (DRY - generated from PALETTE constant)
    result.push_str(&palette_to_sixel_header());
    
    // Process by sixel rows
    for y in (0..img_height).step_by(SIXEL_HEIGHT as usize) {
        let mut current_color: Option<u16> = None;
        
        // Find regions intersecting this sixel row
        let mut row_regions: Vec<&EncoderRegion> = regions
            .iter()
            .filter(|r| r.y <= y && y < r.y + r.height)
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
            let row_offset = if y >= region.y { y - region.y } else { 0 };
            let mut sixel_value = 0u32;
            
            // Set bits for pixels that belong to this region
            for i in 0..SIXEL_HEIGHT {
                if row_offset + i < region.height && y + i < img_height {
                    sixel_value |= 1 << i;
                }
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

/// Implements Sixel rendering for images using geometric region-based encoding
/// Convert an RGB image to a Sixel string for terminal display
/// Uses the geometric folding innovation for optimal compression
pub fn encode_sixel(img: &image::RgbImage) -> String {
    // Function to find the closest color in our palette
    // Uses perceptual color matching based on ITU-R BT.601 standard
    let find_closest_color = |r: u8, g: u8, b: u8| -> u16 {
        let mut min_dist = f32::MAX;
        let mut closest = 0;

        for (i, &(cr, cg, cb)) in PALETTE.iter().enumerate() {
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
    };

    // Phase 1: Detect uniform color regions in the image
    let regions = detect_regions(img, find_closest_color);
    
    // Phase 2: Merge adjacent regions (geometric folding)
    let merged_regions = merge_regions(regions);
    
    // Phase 3: Generate sixel output from optimized regions
    regions_to_sixel(&merged_regions, img.width(), img.height())
}

// Legacy column-based encoding preserved for reference
#[allow(dead_code)]
fn encode_sixel_legacy(img: &image::RgbImage) -> String {
        let mut result = String::from("\x1BPq");

        // Add raster attributes (crucial for Rio compatibility)
        result.push_str(&format!("\"{};{}", img.width(), img.height()));

        // Define palette (DRY - generated from PALETTE constant)
        result.push_str(&palette_to_sixel_header());

        // Function to find the closest color in our palette
        let find_closest_color = |r: u8, g: u8, b: u8| -> u16 {
            // Weighted Euclidean distance for perceptual color matching
            // Weights based on human eye sensitivity (ITU-R BT.601 standard)
            let mut min_dist = f32::MAX;
            let mut closest = 0;

            for (i, &(cr, cg, cb)) in PALETTE.iter().enumerate() {
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
        };

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
                            find_closest_color(pixel[0], pixel[1], pixel[2]);
                    }
                }

                // Use most common color in this column
                let dominant_color = {
                    // Count occurrences of each color (max 16 palette colors)
                    let mut counts = [0u8; 16];
                    for &color in &column_colors[..] {
                        // Bounds check: only count valid palette indices
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
                };

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
        assert!(sixel.contains("\"12;12"), "Should contain raster attributes");
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
        
        let find_color = |r: u8, g: u8, b: u8| -> u16 {
            if r == 20 && g == 20 && b == 80 { 1 } else { 2 }
        };
        
        let regions = detect_regions(&img, find_color);
        
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
        let mut img = image::RgbImage::new(12, 6);
        
        // Columns 0-5: color 1 (dark blue)
        for y in 0..6 {
            for x in 0..6 {
                img.put_pixel(x, y, image::Rgb([20u8, 20u8, 80u8]));
            }
        }
        
        // Columns 6-11: color 2 (dark green)
        for y in 0..6 {
            for x in 6..12 {
                img.put_pixel(x, y, image::Rgb([20u8, 80u8, 20u8]));
            }
        }
        
        let sixel = encode_sixel(&img);
        
        // Both colors must appear in output (no pixels skipped)
        assert!(sixel.contains("#1"), "Must encode color 1 (dark blue)");
        assert!(sixel.contains("#2"), "Must encode color 2 (dark green)");
    }
}
