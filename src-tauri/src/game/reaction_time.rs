use super::colors;
use enigo::{Enigo, Mouse, Button, Direction, Settings, Coordinate};
use std::time::{Duration, Instant};

use crate::screen::utils::{detect_play_arena, screenshot_region, color_matches, arena_coords_to_monitor_relative_coords, arena_coords_to_physical_coords};


const N_CLICKS: usize = 5;
const TIMEOUT_MS: u64 = 60_000; // 60 seconds


#[tauri::command]
pub async fn start_reaction_time() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (start_mouse_x, start_mouse_y) = enigo.location().unwrap();

    let arena = detect_play_arena().map_err(|e| e.to_string())?;
    let play_arena = arena.ok_or("No play arena detected")?;

    let target = arena_coords_to_monitor_relative_coords(&play_arena, 6, 9);
    let target_physical = arena_coords_to_physical_coords(&play_arena, 50, 50);
    
    // Start the game
    let _ = enigo.move_mouse(target_physical.0 as i32, target_physical.1 as i32, Coordinate::Abs);
    let _ = enigo.button(Button::Left, Direction::Click);

    let start_time = Instant::now();

    let mut nclicked = 0;
    let mut nscreenshots = 0;

    while nclicked < N_CLICKS {
        if start_time.elapsed() > Duration::from_millis(TIMEOUT_MS) {
            return Err("Timeout reached".into());
        }

        let image = screenshot_region(&play_arena.monitor, target.0 as u32, target.1 as u32, 1, 1).map_err(|e| e.to_string())?;
        nscreenshots += 1;
        let _pixel = image.get_pixel(0, 0);
        if !color_matches(&_pixel, &colors::REACTION_TIME_CLICK, 10) {
            continue;
        }

        let _ = enigo.move_mouse(target_physical.0 as i32, target_physical.1 as i32, Coordinate::Abs);
        let _ = enigo.button(Button::Left, Direction::Click);
        // Click again to move to the next rep
        let _ = enigo.button(Button::Left, Direction::Click);
        println!("Clicked at ({}, {})", target_physical.0, target_physical.1);
        nclicked += 1;
        
        // Sleep for a bit to avoid multiple clicks on the same target
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("Finished {} clicks! Average FPS: {}", nclicked, nscreenshots as f64 / start_time.elapsed().as_millis() as f64 * 1000.0);

    let _ = enigo.move_mouse(start_mouse_x, start_mouse_y, Coordinate::Abs);
    Ok(())
}