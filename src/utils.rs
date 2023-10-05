use std::{thread, time};
use std::io::{stdin,stdout,Write};
use regex::Regex;
use image::Rgba;
use screenshots::Screen;
// use enigo::*;

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    
    input.trim().to_string()
}

pub fn get_input_int(prompt: &str) -> u64 {
    let input = get_input(prompt);
    
    // try to parse the input as a u64, if it fails, ask again
    match input.parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number.");
            get_input_int(prompt)
        }
    }
}

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

// HashMap { x: u32, y: u32, width: u32, height: u32 }
pub fn detect_app_region() -> Region {
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
        let screen_num = get_input_int("Enter the number of the screen: ");
        screen = screens[screen_num as usize];
    }

    println!("\nUsing screen: {:?}", screen.display_info);

    // take a screenshot of the screen and set x, y by the first pixel of value [43, 135, 209, 255]
    let image = screen.capture().unwrap();
    let mut x = 0;
    let mut y = 0;

    for (i, pixel) in image.pixels().enumerate() {
        if *pixel == Rgba([43, 135, 209, 255]) {
            x = (i as u32 % screen.display_info.width) as u32;
            y = (i as u32 / screen.display_info.width) as u32;
            break;
        }
    }

    // for the bottom right corner, find the longest continuous sequence (line) of pixels with value [43, 135, 209, 255]. Set width and height by the last pixel of that line
    let mut width = 0;
    let mut height = 0;

    for (i, pixel) in image.pixels().enumerate() {
        if *pixel == Rgba([43, 135, 209, 255]) {
            let x2 = (i as u32 % screen.display_info.width) as u32;
            let y2 = (i as u32 / screen.display_info.width) as u32;

            if x2 > x && y2 > y {
                if x2 - x > width {
                    width = x2 - x;
                }
                if y2 - y > height {
                    height = y2 - y;
                }
            }
        }
    }

    Region {
        x,
        y,
        width,
        height,
    }

}