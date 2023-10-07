use std::{thread, time};
use std::io::{stdin,stdout,Write};
use regex::Regex;
use image::{Rgba,ImageBuffer};
use screenshots::Screen;
// use enigo::*;
use std::ffi::CStr;
use std::ptr;
use leptonica_sys::{pixCreate, pixSetPixel, pixDestroy};
use tesseract_sys::*;

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    
    input.trim().to_string()
}

pub fn get_input_int(prompt: &str, default: Option<u64>) -> u64 {
    let input = get_input(prompt);

    // if blank, return default if some, otherwise reprompt
    if input.is_empty() {
        if default == None {
            println!("Please enter a valid input.");
            return get_input_int(prompt, default);
        } else {
            return default.unwrap();
        }
    }
    
    // try to parse the input as a u64, if it fails, ask again
    match input.parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid input.");
            return get_input_int(prompt, default);
        }
    }
}

// pub fn get_input_bool(prompt: &str, default: Option<bool>) -> bool {
//     let input = get_input(prompt);
//     let input = input.trim();

//     // if blank, return default if some, otherwise reprompt
//     if input.is_empty() {
//         if default == None {
//             println!("Please enter a valid input.");
//             return get_input_bool(prompt, default);
//         } else {
//             return default.unwrap();
//         }
//     }

//     // accept "y" "yes" "true" "t" "1" as true
//     // other inputs are false
//     let input = input.to_lowercase();
//     if input == "y" || input == "yes" || input == "true" || input == "t" || input == "1" {
//         return true;
//     }
//     return false;
// }

pub fn delay_countdown(seconds: u64) {
    // print "Starting in _ seconds..." every second
    for i in (1..seconds+1).rev() {
        println!("Starting in {} seconds...", i);
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn html_parser(html_elem: &str) -> Vec<&str> {
    let re = Regex::new(r#">(\s*.*?)\s*?<"#).unwrap();
    let result: Vec<&str> = re.captures_iter(html_elem)
    .map(|m| m.get(1).map_or("", |m| m.as_str()))
    .filter(|s| !s.is_empty())
    .collect();
result
}

#[derive(Debug)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn detect_app_region(mut color: Option<Rgba<u8>>) -> Region {
    // if arg color is None, use Rgba([43, 135, 209, 255]) as default
    // if arg color is Some(color), use that color
    if color == None {
        color = Some(Rgba([43, 135, 209, 255]));
    }

    let background_color = Rgba([43, 135, 209, 255]);

    let screens = Screen::all().unwrap();

    // if there is only one screen, use that, otherwise prompt the user to select one
    let screen: Screen;
    if screens.len() == 1 {
        screen = screens[0];
    } else {
        println!("Which screen would you like to use?");
        for (i, screen) in screens.iter().enumerate() {
            println!("{}. {:?}", i, screen.display_info);
        }
        let screen_num = get_input_int("Enter the number of the screen: ", None);
        screen = screens[screen_num as usize];
    }

    println!("\nUsing screen: {:?}", screen.display_info);

    // for the top left corner, find the first continuous sequence of 10 pixels with value color. Set x and y by the last pixel of that sequence, offset x by 10
    let image = screen.capture().unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut current_line = 0;

    for (i, pixel) in image.pixels().enumerate() {
        // if the pixel is the target color, increment current_line
        // at any point, if the pixel is not the target color, reset current_line to 0

        if *pixel == color.unwrap() {
            current_line += 1;
        } else {
            current_line = 0;
        }

        // if current_line is 10, set x and y by the last pixel of that sequence, offset x by -10
        if current_line == 10 {
            x = (i as u32 % screen.display_info.width) as u32;
            y = (i as u32 / screen.display_info.width) as u32;
            x -= 10;
            break;
        }
    }

    let mut play_blue_area_width = 0; // longest sequence of background_color
    let mut current_line = 0;
    for (_, pixel) in image.pixels().enumerate() {
        if *pixel == background_color {
            current_line += 1;
        } else {
            current_line = 0;
        }

        if current_line > play_blue_area_width {
            play_blue_area_width = current_line;
        }
    }

    // for the bottom right corner, find the last color pixel that is before a sequence of play_blue_area_width pixels. Set width and height by that last color pixel.
    let mut last_color_pixel = (0, 0);
    let mut current_line = 0; // for background_color
    let mut width = 0;
    let mut height = 0;

    for (i, pixel) in image.pixels().enumerate() {
        if *pixel == color.unwrap() {
            last_color_pixel = (i as u32 % screen.display_info.width, i as u32 / screen.display_info.width);
        }
        
        if *pixel == background_color {
            current_line += 1;
        } else {
            current_line = 0;
        }

        if current_line >= play_blue_area_width {
            width = last_color_pixel.0 - x;
            height = last_color_pixel.1 - y;
        }
    }

    Region {
        x,
        y,
        width,
        height,
    }

}

pub fn ocr(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
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

#[derive(Debug)]
pub struct TrackPoints {
    pub track_points: Vec<(i32, i32)>,
    pub track_indexes: Vec<usize>,
}

// track points / game boards
pub fn detect_track_points(track_color: Rgba<u8>, background_color: Rgba<u8>, play_area_image: &ImageBuffer<Rgba<u8>, Vec<u8>>, play_area: &Region) -> TrackPoints {
    let mut track_points: Vec<(i32, i32)> = Vec::new();
    let mut track_indexes: Vec<usize> = Vec::new();

    let mut left_edges: Vec<(i32, i32)> = Vec::new();
    let mut last_pixel = background_color;
    
    // check for the transition from the background color to the track color, save the x and y values --> left edges
    for (i, pixel) in play_area_image.pixels().enumerate() {
        if *pixel == track_color {
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

    // the left edge contains the left edge of each box, each some pixels long. There are gaps (background color) between each box, though

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
            track_points.push((x.clone() + 25, y.clone() + 20)); // add some padding so that the mouse is not on the edge of the square
        }
    }

    // find the index (position) of these points in the image vector
    for (x, y) in &track_points {
        let index = (y * play_area.width as i32 + x) as usize;
        track_indexes.push(index);
    }

    TrackPoints {
        track_points,
        track_indexes,
    }
}