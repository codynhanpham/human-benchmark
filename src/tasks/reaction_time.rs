use image::Rgba;
use screenshots::Screen;
use enigo::*;
use std::{thread, time};

pub fn reaction_time() {
    println!("\n----------\n");
    let mut enigo = Enigo::new();
    let click_dur = time::Duration::from_millis(20);
    let screen = Screen::from_point(0, 0).unwrap();
    
    println!("Focus the cursor on the reaction time window, then click to start the test.");
    println!("Keep the cursor on the reaction time window, and the program will auto-click when it turns green.\n");
    println!("Screen info: {:?}\n", screen);

    println!("\n");
    println!("Waiting for Green... (Ctrl + C to exit)");

    loop {
        let (x, y) = enigo.mouse_location();
        let image = screen.capture_area(x, y, 1, 1).unwrap();

        // Click on Green
        // let pixel = Rgba([75, 219, 106, 255]);
        if *image.get_pixel(0, 0) == Rgba([75, 219, 106, 255]) {
            enigo.mouse_down(MouseButton::Left);
            thread::sleep(click_dur);
            enigo.mouse_up(MouseButton::Left);
            println!("Clicked!");
            println!("Waiting for Green... (Ctrl + C to exit)");
        }
    }
}