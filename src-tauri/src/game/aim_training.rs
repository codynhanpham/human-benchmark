use super::colors;
use enigo::{Enigo, Mouse, Button, Direction, Settings, Coordinate};
use std::time::{Duration, Instant};

use crate::screen::utils::{arena_coords_to_physical_coords, detect_play_arena, find_target_colored_pixel, find_target_colored_pixels};


const N_CLICKS: usize = 150; // Cap to 150 clicks, should ends at 31
const TIMEOUT_MS: u64 = 10_000; // 10 seconds


#[tauri::command]
pub async fn start_aim_training() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (start_mouse_x, start_mouse_y) = enigo.location().unwrap();

    let arena = detect_play_arena().map_err(|e| e.to_string())?;
    let play_arena = arena.ok_or("No play arena detected")?;

    let ss = play_arena.screenshot().map_err(|e| e.to_string())?;
    let target_pos = find_target_colored_pixel(&ss, &colors::AIM_TRAINER_TARGET, Some(10), Some(10));
    if target_pos.is_none() {
        return Err("Please stay on the Aim Training screen with the start target visible".into());
    }
    let target_pos = target_pos.unwrap();
    let target_physical = arena_coords_to_physical_coords(&play_arena, target_pos.0 as i32, target_pos.1 as i32);
    
    // Start the game
    let _ = enigo.move_mouse(target_physical.0 as i32, target_physical.1 as i32, Coordinate::Abs);
    let _ = enigo.button(Button::Left, Direction::Click);

    let start_time = Instant::now();

    let mut nclicked = 0;
    let mut nscreenshots = 0;

    let colors = vec![
        &colors::AIM_TRAINER_TARGET,
        &colors::PRIMARY_BUTTON,
    ];

    while nclicked < N_CLICKS {
        if start_time.elapsed() > Duration::from_millis(TIMEOUT_MS) {
            return Err("Timeout reached".into());
        }

        let image = play_arena.screenshot().map_err(|e| e.to_string())?;
        nscreenshots += 1;

        let color_checks = find_target_colored_pixels(&image, &colors, Some(5), Some(10));
        let target_pos = color_checks.get(0).unwrap();
        if target_pos.is_none() {
            continue;
        }
        let game_end = color_checks.get(1).unwrap();
        if game_end.is_some() {
            break;
        }

        let target_pos = target_pos.unwrap();
        let target_physical = arena_coords_to_physical_coords(&play_arena, target_pos.0 as i32, target_pos.1 as i32);
        
        let _ = enigo.move_mouse(target_physical.0 as i32, target_physical.1 as i32, Coordinate::Abs);
        let _ = enigo.button(Button::Left, Direction::Click);
        println!("Clicked at ({}, {})", target_physical.0, target_physical.1);
        nclicked += 1;

        // Sleep for a bit to avoid multiple clicks on the same target
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    println!("Finished {} clicks! Average FPS: {}", nclicked, nscreenshots as f64 / start_time.elapsed().as_millis() as f64 * 1000.0);

    let _ = enigo.move_mouse(start_mouse_x, start_mouse_y, Coordinate::Abs);
    Ok(())
}