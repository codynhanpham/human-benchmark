use super::colors;
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use ocrs::ImageSource;
use std::time::{Duration, Instant};

use crate::screen::utils::{arena_coords_to_physical_coords, calculate_optimal_downscale_factor, detect_play_arena, downscale_image, find_all_regions, find_target_colored_pixel, find_target_colored_pixels};
use crate::ocr::engine;


fn detect_number(image: &image::RgbaImage) -> Result<String, String> {
    // Downscale the image for faster processing (or use original if factor is 1)
    let downscale_factor = calculate_optimal_downscale_factor(image.width(), image.height());
    let processing_image = downscale_image(image, downscale_factor);

    let engine = engine::init_ocr_engine().map_err(|e| e.to_string())?;
    let dimension = processing_image.dimensions();
    let text_image = processing_image.into_raw();
    let img_source = ImageSource::from_bytes(&text_image, dimension)
        .map_err(|e| e.to_string())?;
    let ocr_input = engine.prepare_input(img_source).map_err(|e| e.to_string())?;

    let line_texts = engine::predict_from_image_buffer(&engine, &ocr_input, 0).map_err(|e| e.to_string())?;
    let line_texts = line_texts.join("");
    let line_texts = line_texts.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    let line_texts = line_texts.trim().to_string();
    Ok(line_texts)
}

#[tauri::command]
pub async fn start_number_memory() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (start_mouse_x, start_mouse_y) = enigo.location().unwrap();

    let arena = detect_play_arena().map_err(|e| e.to_string())?;
    let play_arena = arena.ok_or("No play arena detected")?;

    let ss = play_arena.screenshot().map_err(|e| e.to_string())?;
    let target_pos = find_target_colored_pixel(&ss, &colors::PRIMARY_BUTTON, Some(10), Some(10));
    if target_pos.is_none() {
        return Err("Please stay on the Number Memory screen with the start target visible".into());
    }
    let target_pos = target_pos.unwrap();
    let target_physical = arena_coords_to_physical_coords(&play_arena, target_pos.0 as i32, target_pos.1 as i32);
    
    // Start the game
    let _ = enigo.move_mouse(target_physical.0 as i32, target_physical.1 as i32, Coordinate::Abs);
    let _ = enigo.button(Button::Left, Direction::Click);

    let colors = vec![
        &colors::NUMBER_MEMORY_INPUT,
        &colors::PRIMARY_BUTTON,
    ];

    let mut number_detected = false;
    let mut detected_number = String::new();

    let mut start_time = Instant::now();

    let mut current_level = 1;

    let mut timeout = 6000;
    std::thread::sleep(Duration::from_millis(200)); // wait a bit for the start button to disappear

    loop {
        if start_time.elapsed() > Duration::from_millis(timeout) {
            return Err("Timeout reached".into());
        }

        let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = play_arena.screenshot().map_err(|e| e.to_string())?;

        let color_checks = find_target_colored_pixels(&image, &colors, Some(3), Some(10));
        let input_field = color_checks.get(0).unwrap(); // only appears during input
        let primary_button = color_checks.get(1).unwrap(); // can appear either during input or game end
        
        if input_field.is_none() && primary_button.is_none() {
            // On the number display screen
            if number_detected {
                std::thread::sleep(Duration::from_millis(500));
                continue;
            }
            else {
                let result = detect_number(&image).map_err(|e| e.to_string())?;
                // Truncate to the first current_level digits (sometimes OCR detects extra digits)
                let result = result.chars().take(current_level).collect::<String>();
                println!("[Level {}] Detected number: {}", current_level, result);
                number_detected = true;
                detected_number = result;
            }
        } else if input_field.is_some() {
            // On the input screen
            std::thread::sleep(Duration::from_millis(100)); // wait a bit for the input field to appear properly
            if !number_detected || detected_number.is_empty() {
                return Err(format!("[Level {}] No number detected to input", current_level));
            }

            // Find the input field region
            let regions = find_all_regions(&image, &colors::NUMBER_MEMORY_INPUT, 1, 10_000);
            let best_region = regions
                .into_iter()
                .max_by_key(|region| region.area());
                
            if best_region.is_none() {
                println!("[Level {}] Could not find input field region", current_level);
                continue;
            }
            
            let best_region = best_region.unwrap();
            // Calculate the physical coordinates of the center of the input field relative to the monitor
            let input_field = arena_coords_to_physical_coords(
                &play_arena,
                (best_region.min_x + best_region.max_x) as i32 / 2,
                (best_region.min_y + best_region.max_y) as i32 / 2,
            );
            let _ = enigo.move_mouse(input_field.0 as i32, input_field.1 as i32, Coordinate::Abs);
            let _ = enigo.button(Button::Left, Direction::Click);

            // Type the detected number
            let _ = enigo.text(&detected_number);
            let _ = enigo.key(Key::Return, Direction::Click);
            let _ = enigo.key(Key::Return, Direction::Click);

            detected_number.clear();
            number_detected = false;
            start_time = Instant::now(); // reset timeout for next level
            current_level += 1;
            // Add 1000 ms timeout for next level
            timeout += 1000;

            std::thread::sleep(Duration::from_millis(500)); // wait a bit for the next number to appear
        } else if primary_button.is_some() {
            // On the game end screen
            break;
        } else {
            println!("[Level {}] Unexpected state", current_level);
            // Should not reach here, probably
            continue;
        }
    }

    let _ = enigo.move_mouse(start_mouse_x, start_mouse_y, Coordinate::Abs);
    Ok(())
}


