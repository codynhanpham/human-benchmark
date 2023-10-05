use enigo::*;
use std::fs;
use crate::utils::*;

pub fn typing_test() {
    // ask to hit enter to open a text file
    println!("\n----------\n");
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