// target color = [255, 255, 255, 255]

use image::Rgba;
use screenshots::Screen;
use enigo::*;
use std::{thread, time};

use crate::utils::*;

// the color of the target can be slightly offwhite, so we need to use a range
// compare the distance between the target color and the pixel color, if it is less than the margin, it is the target color
// take in RGBA values and return a bool
fn within_margin(target_color: Rgba<u8>, pixel_color: Rgba<u8>, margin: u8) -> bool {
    let mut distance = 0;
    for i in 0..4 {
        distance += (target_color[i] as i32 - pixel_color[i] as i32).abs();
    }
    distance <= margin as i32
}

pub fn aim_trainer() {
    println!("\n----------\n");
    println!("The refresh rate of the screen can be unfortunate, so you might have to run this a few times to get a good run.");

    let mut counter = 0;
    let mut enigo = Enigo::new();
    let click_delay_ms = 3;
    let click_dur = time::Duration::from_millis(click_delay_ms);
    let mut last_xy = (0, 0);

    println!("\n\nPut this window somewhere outside of the play area, then hit enter to start the run.");

    let _ = get_input("Enter to start...");

    // detect the play area
    let play_area = detect_app_region();

    // if width or height is 0, then the app is not found, so restart the program
    if play_area.width == 0 || play_area.height == 0 {
        println!("Playing screen not found, make sure the app is running and in focus, then try again.");

        let _ = get_input("Press enter to restart...");
        aim_trainer();
    }

    println!("Play area: From ({}, {}) to ({}, {})", play_area.x, play_area.y, play_area.x + play_area.width, play_area.y + play_area.height);

    let mut last_mouse_moved = time::Instant::now();
    
    let screen = Screen::from_point(0, 0).unwrap();
    loop {
        let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut _current_line = 0;

        for (i, pixel) in image.pixels().enumerate() {
            if *pixel == Rgba([149, 195, 232, 255]) {
                _current_line += 1;
            } else {
                if _current_line >= 2 {
                    if within_margin(Rgba([255, 255, 255, 255]), *pixel, 20) {
                        x = (i as u32 % play_area.width) as i32;
                        y = (i as u32 / play_area.width) as i32;
                        break;
                    }
                }
                _current_line = 0;
            }
        }

        // if the target color is not found, skip this iteration
        if x == 0 && y == 0 {
            continue;
        }

        // if the same xy, also skip, probably the frame did not refresh
        let error_margin = 5; // after abs
        if (x - last_xy.0).abs() <= error_margin && (y - last_xy.1).abs() <= error_margin {
            // if the mouse has not moved for more than 1.5 second, break, otherwise, skip
            if last_mouse_moved.elapsed().as_millis() >= 1500 {
                println!("Mouse has not moved for more than 1.5 seconds, Emergency break!");
                break;
            } else {
                continue;
            }
        }

        enigo.mouse_move_to(play_area.x as i32 + x, play_area.y as i32 + y);

        // debug sleep here to check mouse pos
        // thread::sleep(time::Duration::from_millis(1000));

        enigo.mouse_down(MouseButton::Left);
        thread::sleep(click_dur);
        enigo.mouse_up(MouseButton::Left);
        counter += 1;
        println!("Clicked {} times! Ctrl + C to exit", counter);

        if counter > 31*50 {
            println!("Clicked a bit too much, finished?");
            break;
        }

        last_xy = (x, y);
        last_mouse_moved = time::Instant::now();
    }

    // Restart the program
    let _ = get_input("Press enter to restart...");

    aim_trainer();
}