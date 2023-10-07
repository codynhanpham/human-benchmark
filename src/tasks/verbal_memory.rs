use image::Rgba;
use screenshots::Screen;
use enigo::*;
use std::collections::HashSet;
use std::{thread, time};

use crate::utils::*;

// click able button for this is [255, 209, 84, 255]
// Left is Seen
// Right is New
pub fn verbal_memory() {
    // set env of TESSDATA_PREFIX to "src"
    std::env::set_var("TESSDATA_PREFIX", ".");

    println!("\n----------\n");
    println!("\n\nPut this window somewhere outside of the play area.");
    println!("Start the game, stay on the first word, then start this script.\n");
    let _ = get_input("Enter to start...");

    // detect the play area
    let play_area = detect_app_region(None);
    println!("Play area: From ({}, {}) to ({}, {})", play_area.x, play_area.y, play_area.x + play_area.width, play_area.y + play_area.height);

    let screen = Screen::from_point(0, 0).unwrap();
    let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

    println!("Detecting game board coordinates...");
    // detect the 3 x 3 grid
    let board_color = Rgba([255 as u8, 209 as u8, 84 as u8, 255 as u8]);
    let background_color = Rgba([43 as u8, 135 as u8, 209 as u8, 255 as u8]);

    let buttons = detect_track_points(board_color, background_color, &image, &play_area);

    let two_buttons = buttons.track_points;
    
    println!("Game board coordinates detected.\n");
    println!("Button coordinates: {:?}\n\n", two_buttons);

    let mut enigo = Enigo::new();

    let mut word_collection: HashSet<String> = HashSet::new();
    let mut last_score = -1;
    let mut last_lives = -1;
    let mut state:bool; // true is on, false is off
    let mut last_collection_size = 0;

    loop {
        let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();
        let text = ocr(&image);
        /* text is in the format:
        Lives|3 Score |0
        decompositions
         */
        // the actual word is on the 2nd line
        let word = match text.lines().nth(1) {
            Some(n) => n.trim().to_string(),
            None => break,
        };
        let score = match text.lines().nth(0).unwrap_or("error").split("|").nth(2).unwrap_or("error").trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => break,
        };
        let lives = match text.lines().nth(0).unwrap_or("error").split("|").nth(1).unwrap_or("error").split("Score").nth(0).unwrap_or("error").trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => break,
        };

        // println!("Word: {}", word);
        // println!("Score: {}", score);
        // println!("Lives: {}", lives);
        
        // frame is updated either when score is updated or lives is updated
        if score != last_score || lives != last_lives {
            // update last_score and last_lives
            last_score = score;
            last_lives = lives;
            // update state
            state = true;
        } else {
            state = false;
        }

        if !state {
            continue;
        }

        if word_collection.contains(&word) {
            // seen
            // click the left button
            enigo.mouse_move_to(play_area.x as i32 + two_buttons[0].0, play_area.y as i32 + two_buttons[0].1);
            enigo.mouse_click(MouseButton::Left);
        } else {
            // new
            // click the right button
            enigo.mouse_move_to(play_area.x as i32 + two_buttons[1].0, play_area.y as i32 + two_buttons[1].1);
            enigo.mouse_click(MouseButton::Left);
        }

        word_collection.insert(word.clone());
        
        // if collection size update, print the new size
        if word_collection.len() != last_collection_size {
            println!("Collection size:\t{}\t║\tNew word:\t{}", word_collection.len(), word);
            last_collection_size = word_collection.len();
        }

        thread::sleep(time::Duration::from_millis(40));
    }

    println!("Done.");
    println!("Score: {}", last_score);

    // wait for enter before restart
    println!("\n");
    let _ = get_input("Press enter to restart...");
    verbal_memory();
}