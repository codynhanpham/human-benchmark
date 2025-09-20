use super::colors;
use enigo::{Enigo, Mouse, Button, Keyboard, Direction, Settings, Coordinate};
use ocrs::ImageSource;

use crate::screen::utils::{arena_coords_to_physical_coords, detect_play_arena, find_all_regions};
use crate::ocr::engine;


#[tauri::command]
pub async fn start_typing_test() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (start_mouse_x, start_mouse_y) = enigo.location().unwrap();

    let arena = detect_play_arena().map_err(|e| e.to_string())?;
    let play_arena = arena.ok_or("No play arena detected")?;

    let image = play_arena.screenshot().map_err(|e| e.to_string())?;
    
    // Test with the typing test inactive background color
    let regions = find_all_regions(&image, &colors::TYPING_TEST_INACTIVE_BACKGROUND, 5, 10_000);
    // Find the largest region for the typing test area
    let best_region = regions
        .into_iter()
        .max_by_key(|region| region.area());
        
    if best_region.is_none() {
        return Err("Please start the typing test and ensure the typing test area is visible.".into());
    }
    
    let best_region = best_region.unwrap();
    println!("Detected typing test arena at ({}, {}) with size {}x{}", best_region.min_x, best_region.min_y, best_region.width(), best_region.height());

    // Crop the image to the best arena
    let typing_test_arena_image = image::imageops::crop_imm(
        &image,
        best_region.min_x,
        best_region.min_y,
        best_region.width(),
        best_region.height()
    ).to_image();

    // Calculate the physical coordinates of the center of the typing test arena relative to the monitor
    let typing_test_arena_center_physical = arena_coords_to_physical_coords(
        &play_arena,
        (best_region.min_x + best_region.max_x) as i32 / 2,
        (best_region.min_y + best_region.max_y) as i32 / 2,
    );

    let engine = engine::init_ocr_engine().map_err(|e| e.to_string())?;
    let dimension = typing_test_arena_image.dimensions();
    let text_image = typing_test_arena_image.into_raw();
    let img_source = ImageSource::from_bytes(&text_image, dimension)
        .map_err(|e| e.to_string())?;
    let ocr_input = engine.prepare_input(img_source).map_err(|e| e.to_string())?;

    println!("Performing OCR on detected typing test arena...");
    let line_texts = engine::predict_from_image_buffer(&engine, &ocr_input, 1).map_err(|e| e.to_string())?;
    // Join with spaces
    let line_texts = line_texts.join(" ");
    println!("Detected text: \n\t{}", line_texts);

    // Start the game
    let _ = enigo.move_mouse(typing_test_arena_center_physical.0 as i32, typing_test_arena_center_physical.1 as i32, Coordinate::Abs);
    let _ = enigo.button(Button::Left, Direction::Click);

    // Type as fast as possible

    if cfg!(target_os = "windows") {
        simulate::type_str(&line_texts).unwrap();
    } else {
        // Unix-like
        let _ = enigo.text(&line_texts);
    }

    let _ = enigo.move_mouse(start_mouse_x, start_mouse_y, Coordinate::Abs);
    Ok(())
}