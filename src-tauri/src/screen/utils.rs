use std::time::Instant;
use std::collections::{HashSet, VecDeque};
use xcap::Monitor;
use serde::Serialize;

use enigo::{Enigo, Mouse, Settings};


#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub x: i32, // physical top-left corner of this monitor
    pub y: i32, // physical top-left corner of this monitor
    pub width: u32,
    pub height: u32,
    #[serde(skip)]
    pub monitor: Monitor, // the underlying monitor object
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayArena {
    pub x: i32, // top-left corner of the play arena, if (0,0) is top-left of this monitor
    pub y: i32, // top-left corner of the play arena, if (0,0) is top-left of this monitor
    pub width: u32, // width of the play arena
    pub height: u32, // height of the play arena
    pub monitor: MonitorInfo, // the monitor this play arena is on
}

#[derive(Debug, Clone)]
pub struct Region {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub pixel_count: u32,
}

impl Region {
    fn new() -> Self {
        Region {
            min_x: u32::MAX,
            min_y: u32::MAX,
            max_x: 0,
            max_y: 0,
            pixel_count: 0,
        }
    }

    fn add_pixel(&mut self, x: u32, y: u32) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.pixel_count += 1;
    }

    fn width(&self) -> u32 {
        if self.max_x >= self.min_x { self.max_x - self.min_x + 1 } else { 0 }
    }

    fn height(&self) -> u32 {
        if self.max_y >= self.min_y { self.max_y - self.min_y + 1 } else { 0 }
    }

    fn area(&self) -> u32 {
        self.width() * self.height()
    }
}

// Target color: #2b87d1
const TARGET_COLOR: [u8; 3] = [0x2b, 0x87, 0xd1];
const COLOR_TOLERANCE: u8 = 10; // Allow some tolerance for color variations
const MIN_PROCESSING_DIMENSION: u32 = 540; // Minimum dimension for downscaled processing

fn color_matches(pixel: &image::Rgba<u8>, target: &[u8; 3], tolerance: u8) -> bool {
    let [r, g, b, _] = pixel.0;
    let dr = (r as i16 - target[0] as i16).abs() as u8;
    let dg = (g as i16 - target[1] as i16).abs() as u8;
    let db = (b as i16 - target[2] as i16).abs() as u8;
    
    dr <= tolerance && dg <= tolerance && db <= tolerance
}

fn calculate_optimal_downscale_factor(width: u32, height: u32) -> u32 {
    let smaller_dimension = width.min(height);
    
    // If already small enough, don't downscale
    if smaller_dimension <= MIN_PROCESSING_DIMENSION {
        return 1;
    }
    
    // Calculate factor so that smaller dimension becomes approximately MIN_PROCESSING_DIMENSION
    let factor = smaller_dimension / MIN_PROCESSING_DIMENSION;
    
    // Ensure factor is at least 1
    factor.max(1)
}

fn downscale_image(image: &image::RgbaImage, factor: u32) -> image::RgbaImage {
    if factor == 1 {
        return image.clone();
    }
    
    let new_width = (image.width() + factor - 1) / factor; // Ceiling division
    let new_height = (image.height() + factor - 1) / factor;
    
    let mut downscaled = image::RgbaImage::new(new_width, new_height);
    
    for y in 0..new_height {
        for x in 0..new_width {
            let orig_x = x * factor;
            let orig_y = y * factor;
            
            // Sample the original pixel (or use nearest neighbor)
            if orig_x < image.width() && orig_y < image.height() {
                let pixel = image.get_pixel(orig_x, orig_y);
                downscaled.put_pixel(x, y, *pixel);
            }
        }
    }
    
    downscaled
}

fn upscale_region(region: Region, factor: u32) -> Region {
    // If factor is 1, return the region as-is
    if factor == 1 {
        return region;
    }
    
    Region {
        min_x: region.min_x * factor,
        min_y: region.min_y * factor,
        max_x: (region.max_x + 1) * factor - 1, // Adjust for scaling
        max_y: (region.max_y + 1) * factor - 1,
        pixel_count: region.pixel_count * factor * factor, // Approximate scaling
    }
}

fn flood_fill(image: &image::RgbaImage, start_x: u32, start_y: u32, visited: &mut HashSet<(u32, u32)>) -> Option<Region> {
    let width = image.width();
    let height = image.height();
    
    if start_x >= width || start_y >= height || visited.contains(&(start_x, start_y)) {
        return None;
    }
    
    let start_pixel = image.get_pixel(start_x, start_y);
    if !color_matches(start_pixel, &TARGET_COLOR, COLOR_TOLERANCE) {
        return None;
    }
    
    let mut region = Region::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    
    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        
        visited.insert((x, y));
        region.add_pixel(x, y);
        
        // Check 4-connected neighbors
        let neighbors = [
            (x.wrapping_sub(1), y), // left
            (x + 1, y),             // right
            (x, y.wrapping_sub(1)), // up
            (x, y + 1),             // down
        ];
        
        for (nx, ny) in neighbors {
            if nx < width && ny < height && !visited.contains(&(nx, ny)) {
                let pixel = image.get_pixel(nx, ny);
                if color_matches(pixel, &TARGET_COLOR, COLOR_TOLERANCE) {
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    
    Some(region)
}

fn find_all_regions(image: &image::RgbaImage) -> Vec<Region> {
    // Downscale the image for faster processing (or use original if factor is 1)
    let downscale_factor = calculate_optimal_downscale_factor(image.width(), image.height());
    let processing_image = downscale_image(image, downscale_factor);
    let width = processing_image.width();
    let height = processing_image.height();
    
    if downscale_factor == 1 {
        println!("Image size: {}x{} - processing at original resolution", 
            image.width(), image.height());
    } else {
        println!("Original size: {}x{}, Downscaled by {}x to: {}x{}", 
            image.width(), image.height(), downscale_factor, width, height);
    }
    
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    
    // Use smaller step size on downscaled image, or larger step for original resolution
    let step_size = if downscale_factor == 1 { 5 } else { 2 };
    
    for y in (0..height).step_by(step_size) {
        for x in (0..width).step_by(step_size) {
            if !visited.contains(&(x, y)) {
                let pixel = processing_image.get_pixel(x, y);
                if color_matches(pixel, &TARGET_COLOR, COLOR_TOLERANCE) {
                    if let Some(region) = flood_fill(&processing_image, x, y, &mut visited) {
                        // Scale the region back up to original size
                        let final_region = upscale_region(region, downscale_factor);
                        
                        // Filter out small regions - threshold for final size
                        if final_region.pixel_count >= 10000 {
                            regions.push(final_region);
                        }
                    }
                }
            }
        }
    }
    
    regions
}

fn find_largest_region(regions: Vec<Region>) -> Option<Region> {
    regions.into_iter().max_by_key(|r| r.area())
}

fn region_to_play_arena(region: Region, monitor_info: MonitorInfo) -> PlayArena {
    PlayArena {
        x: region.min_x as i32,
        y: region.min_y as i32,
        width: region.width(),
        height: region.height(),
        monitor: monitor_info,
    }
}

pub fn arena_coords_to_physical_coords(arena: &PlayArena, x: i32, y: i32) -> (i32, i32) {
    (arena.monitor.x + arena.x + x, arena.monitor.y + arena.y + y)
}


fn get_monitors() -> Vec<MonitorInfo> {
    let monitors = Monitor::all().unwrap();

    let mut monitor_infos = vec![];

    for monitor in monitors {
        monitor_infos.push(MonitorInfo {
            id: monitor.id().unwrap(),
            name: monitor.name().unwrap(),
            x: monitor.x().unwrap(),
            y: monitor.y().unwrap(),
            width: monitor.width().unwrap(),
            height: monitor.height().unwrap(),
            monitor,
        });
    }

    monitor_infos
}


fn screenshot(monitor_info: &MonitorInfo) -> image::RgbaImage {
    monitor_info.monitor.capture_image().unwrap()
}

#[tauri::command]
pub async fn get_mouse_position() -> (i32, i32) {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let (x, y) = enigo.location().unwrap();
    (x, y)
}




#[tauri::command]
pub async fn detect_play_arena() -> Result<Option<PlayArena>, String> {
    let start = Instant::now();
    let monitors_info = get_monitors();

    let mut all_arenas = Vec::new();
    
    for monitor_info in monitors_info {
        let image = screenshot(&monitor_info);
        let regions = find_all_regions(&image);
        
        if let Some(largest_region) = find_largest_region(regions) {
            // Only consider regions that are reasonably large (at least 100x100 pixels)
            if largest_region.width() >= 100 && largest_region.height() >= 100 {
                all_arenas.push(region_to_play_arena(largest_region, monitor_info));
            }
        }
    }

    // Find the largest play arena across all monitors
    let best_arena = all_arenas
        .into_iter()
        .max_by_key(|arena| arena.width * arena.height);

    if let Some(ref arena) = best_arena {
        println!(
            "Best play arena found: {}x{} at ({}, {}) on monitor {}",
            arena.width, arena.height, arena.x, arena.y, arena.monitor.name
        );
    } else {
        println!("No play arena detected on any monitor");
    }

    println!("Detection completed in: {:?}", start.elapsed());
    Ok(best_arena)
}
