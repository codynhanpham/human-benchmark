use screenshots::Screen;
use enigo::*;
use std::{thread, time};

pub fn mouse_pos_tracker() {
    println!("\n----------\n");
    let enigo = Enigo::new();
    loop {
        let (x, y) = enigo.mouse_location();

        let screen = Screen::from_point(0, 0).unwrap();
        let image = screen.capture_area(x, y, 1, 1).unwrap();

        println!("x: {}, y: {}\nPixel Color: {:?}", x, y, *image.get_pixel(0, 0));
        thread::sleep(time::Duration::from_millis(200));
    }
}