use enigo::*;
use std::fs;
use crate::utils::*;
use image::Rgba;
use screenshots::Screen;



// the typing region's color (focused) is rgba(234, 243, 250, 255)
// off focus color is rgba(213, 231, 246, 255) --> use this color to detect the typing region
pub fn typing_test() {
    println!("\n----------\n");
    
    // 2 way to do this, ask for user choice
    // either doing ocr and type automatically,
    // or ask the user to copy the text and paste it into a file

    // ask for user choice
    println!("Do you want to use OCR to detect the text automatically, or do you want to do that manually?");
    println!("1. OCR");
    println!("2. Manual");
    let choice = get_input_int("Enter the number of your choice: ", Some(1));

    if choice == 1 {
        typing_test_ocr();
    } else {
        typing_test_manual();
    }
}

fn typing_test_ocr() {
    println!();
    println!("Start the typing test screen and put this window somewhere outside of the play area.");

    let _ = get_input("Press enter to start doing OCR...");

    // detect the typing region
    let typing_region = detect_app_region(Some(Rgba([213, 231, 246, 255])));
    println!("Typing region: From ({}, {}) to ({}, {})", typing_region.x, typing_region.y, typing_region.x + typing_region.width, typing_region.y + typing_region.height);

    // save a screenshot of the typing region
    let screen = Screen::from_point(0, 0).unwrap();
    let image = screen.capture_area(typing_region.x as i32, typing_region.y as i32, typing_region.width, typing_region.height).unwrap();

    let text = ocr(&image);
    // replace all newlines with spaces
    let text = text.replace("\n", " ");
    println!("OCR detected text:\n\n{}\n\n", text);

    let mut enigo = Enigo::new();

    delay_countdown(3);
    // move the cursor to the typing region, click
    enigo.mouse_move_to((typing_region.x + typing_region.width / 2) as i32, (typing_region.y + typing_region.height / 2) as i32);
    enigo.mouse_click(MouseButton::Left);

    // type the text
    enigo.key_sequence(&text);
    println!("Done!");
}

fn typing_test_manual() {
    println!();
    // ask to hit enter to open a text file
    println!("Press Ctrl + Shift + I to open the developer console. On the typing test page.");
    println!("Click on the typing field, then find the div with class=\"letters notranslate\"");
    println!("Right click on the div and click \"Copy > Copy element\"");
    println!("Paste the copied element into the following file, then save and close the file. It will be deleted after the initialization is finished.\n");
    let _ = get_input("Press enter to open an input text file...");
    // create a ./html.txt file an open that window
    fs::write("html.txt", "").expect("Unable to write file");
    let _ = open::that("html.txt");

    // ask to continue, then check if the file contains anything
    let _ = get_input("Press enter to continue...");
    let html = fs::read_to_string("html.txt").expect("Something went wrong reading the file");
    if html.is_empty() {
        println!("File is empty. Please try again.");
        println!("\n\n\n----------\n\n\n");
        return typing_test();
    }

    // parse the html file and get the text
    let text = html_parser(&html);
    let text_string = text.join("");

    // delete the html.txt file
    fs::remove_file("html.txt").expect("Unable to delete file");
    println!("Temporary file deleted.\n\n----------\n\n");


    let mut enigo = Enigo::new();

    // set a delay countdown of 5 seconds, then start typing the sequence
    println!("Focus the cursor on the typing test window.");
    delay_countdown(8);
    println!("Running typing test...");
    enigo.key_sequence(&text_string);
    println!("Done!");
}