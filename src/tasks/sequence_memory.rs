// [37, 115, 193, 255] board color: 3 x 3 grid
// [43, 135, 209, 255] background color

use image::Rgba;
use screenshots::Screen;
use enigo::*;
use std::{thread, time};

use crate::utils::*;

pub fn sequence_memory() {
    println!("\n----------\n");
    println!("Start the game, MEMORIZE THE FIRST SEQUENCE, then start this script.\nDO NOT CLICK YET...");
    let _ = get_input("Enter to start...");

    // detect the play area
    let play_area = detect_app_region();

    // take a screenshot of the play area
    let screen = Screen::from_point(0, 0).unwrap();
    let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

    println!("Play area: From ({}, {}) to ({}, {})", play_area.x, play_area.y, play_area.x + play_area.width, play_area.y + play_area.height);
    
    println!("Detecting game board coordinates...");
    // detect the 3 x 3 grid
    let board_color = Rgba([37 as u8, 115 as u8, 193 as u8, 255 as u8]);
    let background_color = Rgba([43 as u8, 135 as u8, 209 as u8, 255 as u8]);

    let mut left_edges: Vec<(i32, i32)> = Vec::new();
    let mut last_pixel = background_color;
    
    // check for the transition from the background color to the board color, save the x and y values --> left edges
    for (i, pixel) in image.pixels().enumerate() {
        if *pixel == board_color {
            // check if the previous pixel was the background color
            if i > 0 && last_pixel == background_color {
                // save the x and y values
                let x = (i % play_area.width as usize) as i32;
                let y = (i / play_area.width as usize) as i32;
                
                left_edges.push((x, y));
            }
        }
        last_pixel = *pixel;
    }

    // the left edge contains the left edge of each square, each some pixels long. There are gaps (background color) between each square, though
    let mut nine_points: Vec<(i32, i32)> = Vec::new();

    // convert the left edges to a vector of x values and vector of y values
    let mut x_values: Vec<i32> = Vec::new();
    let mut y_values: Vec<i32> = Vec::new();

    for (x, y) in &left_edges {
        x_values.push(*x);
        y_values.push(*y);
    }

    // there should be a gap, so if x -  or y - 1 is in the vector, then it is not the first point of a square. remove it from the vector
    for (_, (x, y)) in left_edges.iter().enumerate() {
        if x_values.contains(&(x - 1)) || y_values.contains(&(y - 1)) {
            // not the first point of a square
            continue;
        } else {
            // first point of a square
            nine_points.push((x.clone() + 25, y.clone() + 20)); // add some padding so that the mouse is not on the edge of the square
        }
    }
    
    // find the index (position) of these points in the image vector
    let mut nine_points_index: Vec<usize> = Vec::new();
    
    for (x, y) in &nine_points {
        let index = (y * play_area.width as i32 + x) as usize;
        nine_points_index.push(index);
    }
    
    println!("Game board coordinates detected.\n");
    
    // ------- 
    let mut sequence_stream: Vec<usize> = Vec::new(); // sequence of the index of the 9 points, in order changed
    let mut last_sequence = sequence_stream.clone();
    let mut lock_states: Vec<bool> = Vec::new(); // whether the point is locked or not
    // lock state --> 9 bools, default is false for not locked
    lock_states.resize(9, false);
    let mut last_all_blank = time::Instant::now();
    
    let mut enigo = Enigo::new();

    println!("Click on the first sequence to start.");
    
    // game loop: check the color of the 9 points in each iteration screenshot, if the color is not board color and not locked, then add the index to the sequence
    // after changing the color, lock the point, and unlock after that color has changed back to board color
    loop {
        let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

        // filter for the 9 points
        let mut nine_points_color: Vec<Rgba<u8>> = Vec::new();

        for index in &nine_points_index {
            nine_points_color.push(*image.get_pixel(*index as u32 % play_area.width, *index as u32 / play_area.width));
        }

        // println!("Nine points color: {:?}", nine_points_color);

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
            if last_all_blank.elapsed().as_millis() >= 900 {
                last_sequence = sequence_stream.clone();
                sequence_stream.clear();
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

        // if sequence_stream and last_sequence are not empty, print out the values
        if !sequence_stream.is_empty() || !last_sequence.is_empty() {
            println!("Sequence Stream: {:?}", sequence_stream);
            println!("Last Sequence: {:?}", last_sequence);
        }

        // convert the sequence to a vec of the 9 points coordinates
        let mut sequence_points: Vec<(i32, i32)> = Vec::new();

        for index in &last_sequence {
            sequence_points.push(nine_points[*index]);
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

        last_sequence.clear();
        thread::sleep(time::Duration::from_millis(100));
    }
}