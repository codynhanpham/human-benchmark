// [37, 115, 193, 255] board color: 3 x 3 grid
// [43, 135, 209, 255] background color

use image::Rgba;
use screenshots::Screen;
use enigo::*;
use std::{thread, time};
use std::io::{stdout,Write};

use crate::utils::*;

pub fn sequence_memory() {
    println!("\n----------\n");
    println!("Start the game, MEMORIZE THE FIRST SEQUENCE, then start this script.\nDO NOT CLICK YET...");
    let _ = get_input("Enter to start...");

    delay_countdown(2);

    // detect the play area
    let play_area = detect_app_region(None);

    // take a screenshot of the play area
    let screen = Screen::from_point(0, 0).unwrap();
    let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

    println!("Play area: From ({}, {}) to ({}, {})", play_area.x, play_area.y, play_area.x + play_area.width, play_area.y + play_area.height);
    
    println!("Detecting game board coordinates...");
    // detect the 3 x 3 grid
    let board_color = Rgba([37 as u8, 115 as u8, 193 as u8, 255 as u8]);
    let background_color = Rgba([43 as u8, 135 as u8, 209 as u8, 255 as u8]);

    let board = detect_track_points(board_color, background_color, &image, &play_area);

    let nine_points_index = board.track_indexes;
    let nine_points = board.track_points;
    
    println!("Game board coordinates detected.\n");
    
    // ------- 
    let mut sequence_stream: Vec<usize> = Vec::new(); // sequence of the index of the 9 points, in order changed
    let mut sequence_stream_last_len: usize = 0;
    let mut last_sequence = sequence_stream.clone();
    let mut lock_states: Vec<bool> = Vec::new(); // whether the point is locked or not
    // lock state --> 9 bools, default is false for not locked
    lock_states.resize(9, false);
    let mut last_all_blank = time::Instant::now();
    let mut reset_sequence = true;
    
    let mut enigo = Enigo::new();

    println!("\x1b[33mNote: If this script fail during the first sequence, restart the script and try again.\x1b[0m");

    println!("\nClick on the first sequence to start.");
    
    // game loop: check the color of the 9 points in each iteration screenshot, if the color is not board color and not locked, then add the index to the sequence
    // after changing the color, lock the point, and unlock after that color has changed back to board color
    loop {
        let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

        // filter for the 9 points
        let mut nine_points_color: Vec<Rgba<u8>> = Vec::new();

        for index in &nine_points_index {
            nine_points_color.push(*image.get_pixel(*index as u32 % play_area.width, *index as u32 / play_area.width));
        }
        
        // if all 9 points are board color, then reset the sequence
        let mut all_blank = true;
        for point in &nine_points_color {
            if *point != board_color {
                all_blank = false;
                break;
            }
        }

        if all_blank {
            // if the last all blank was more than 1 seconds ago, reset the sequence
            if last_all_blank.elapsed().as_millis() >= 800 {
                last_sequence = sequence_stream.clone();
                sequence_stream.clear();
                reset_sequence = true;
                lock_states = vec![false; 9];
            }
        } else {
            last_all_blank = time::Instant::now();
        }

        // check if the color is not board color and not locked
        for (index, point) in nine_points_color.iter().enumerate() {
            if *point != board_color && !lock_states[index] {
                // add the index to the sequence
                sequence_stream.push(index);

                // lock the point
                lock_states[index] = true;
            }

            // if the color is board color and locked, unlock the point
            if *point == board_color && lock_states[index] {
                lock_states[index] = false;
            }
        }
        
        if sequence_stream == last_sequence || sequence_stream == [0, 1, 2, 3, 4, 5, 6, 7, 8] {
            sequence_stream.clear();
        }
        
        // remove the [0, 1, 2, 3, 4, 5, 6, 7, 8] sequence anywhere in the sequence
        let mut index = 0;
        while index < sequence_stream.len() {
            if sequence_stream[index..].starts_with(&[0, 1, 2, 3, 4, 5, 6, 7, 8]) {
                sequence_stream.drain(index..index + 9);
            } else {
                index += 1;
            }
        }

        // convert the sequence to a vec of the 9 points coordinates
        let mut sequence_points: Vec<(i32, i32)> = Vec::new();
        
        for index in &last_sequence {
            sequence_points.push(nine_points[*index]);
        }
        
        // log:
        if sequence_stream.len() == 1 && reset_sequence {
            reset_sequence = false;
            print!("Sequence: {:?}", sequence_stream[0]);
            stdout().flush().expect("Failed to flush stdout");
        }
        else if sequence_stream.len() == sequence_stream_last_len + 1 {
            print!(" {:?}", sequence_stream[sequence_stream.len() - 1]);
            stdout().flush().expect("Failed to flush stdout");
        }

        if !last_sequence.is_empty() && last_sequence != sequence_stream {
            println!("\nLast sequence: {:?}", last_sequence);
        }
        
        // in order, move the mouse to the points in the sequence and click
        for (x, y) in &sequence_points {
            enigo.mouse_move_to(play_area.x as i32 + x, play_area.y as i32 + y);
            
            // debug sleep here to check mouse pos
            // thread::sleep(time::Duration::from_millis(1000));
            
            enigo.mouse_down(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(20));
            enigo.mouse_up(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(50));
        }
        
        sequence_stream_last_len = sequence_stream.len();
        last_sequence.clear();
        thread::sleep(time::Duration::from_millis(100));
    }
}