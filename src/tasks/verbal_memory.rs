use image::ImageBuffer;
use image::Rgba;
use screenshots::Screen;
use enigo::*;
use crate::utils::*;
use std::ffi::CStr;
use std::ptr;
use leptonica_sys::{pixCreate, pixSetPixel, pixDestroy};
use tesseract_sys::*;
use std::collections::HashSet;
use std::{thread, time};

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
    let play_area = detect_app_region();
    println!("Play area: From ({}, {}) to ({}, {})", play_area.x, play_area.y, play_area.x + play_area.width, play_area.y + play_area.height);

    let screen = Screen::from_point(0, 0).unwrap();
    let image = screen.capture_area(play_area.x as i32, play_area.y as i32, play_area.width, play_area.height).unwrap();

    println!("Detecting game board coordinates...");
    // detect the 3 x 3 grid
    let board_color = Rgba([255 as u8, 209 as u8, 84 as u8, 255 as u8]);
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
    let mut two_buttons: Vec<(i32, i32)> = Vec::new();

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
            two_buttons.push((x.clone() + 25, y.clone() + 20)); // add some padding so that the mouse is not on the edge of the square
        }
    }
    
    // find the index (position) of these points in the image vector
    let mut nine_points_index: Vec<usize> = Vec::new();
    
    for (x, y) in &two_buttons {
        let index = (y * play_area.width as i32 + x) as usize;
        nine_points_index.push(index);
    }
    
    println!("Game board coordinates detected.\n");
    println!("Button coordinates: {:?}", two_buttons);

    let mut enigo = Enigo::new();

    let mut word_collection: HashSet<String> = HashSet::new();
    let mut last_score = -1;
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
        
        if score != last_score {
            state = true;
            last_score += 1;
        }
        else {
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

        word_collection.insert(word);
        
        // if collection size update, print the new size
        if word_collection.len() != last_collection_size {
            println!("Collection size: {}", word_collection.len());
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

fn ocr(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    unsafe {
        let cube = TessBaseAPICreate();
        TessBaseAPIInit3(cube, ptr::null(), b"eng\0".as_ptr().cast());

        let mut pix = pixCreate(image.width() as i32, image.height() as i32, 32);

        for (x, y, pixel) in image.enumerate_pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = pixel[3] as u32;

            let rgba = (r << 24) | (g << 16) | (b << 8) | a;

            pixSetPixel(pix, x as i32, y as i32, rgba);
        }

        TessBaseAPISetImage2(cube, pix);

        TessBaseAPIRecognize(cube, ptr::null_mut());

        pixDestroy(&mut pix);

        let text = TessBaseAPIGetUTF8Text(cube);

        let c_str = CStr::from_ptr(text);
        let string = c_str.to_str().unwrap();

        // TessDeleteText(text);
        TessBaseAPIEnd(cube);
        TessBaseAPIDelete(cube);

        string.to_string()
    }
}