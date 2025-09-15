use core::time;
use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

use ocrs::{ImageSource, OcrEngine, OcrEngineParams, OcrInput};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;
use tauri::Runtime;

use crate::ocr::engine;


pub fn init_ocr_engine() -> Result<OcrEngine, Box<dyn Error>> {
    let detection_model_slice = include_bytes!("./text-detection.rten");
    let recognition_model_slice = include_bytes!("./text-recognition.rten");

    let detection_model = Model::load_static_slice(detection_model_slice)?;
    let recognition_model = Model::load_static_slice(recognition_model_slice)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;
    
    Ok(engine)
}


pub fn predict_from_image_buffer(engine: &OcrEngine, ocr_input: &OcrInput) -> Result<Vec<String>, Box<dyn Error>> {
    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // Get oriented bounding boxes of text words in input image.
    let word_rects = engine.detect_words(ocr_input)?;

    // Group words into lines. Each line is represented by a list of word
    // bounding boxes.
    let line_rects = engine.find_text_lines(ocr_input, &word_rects);

    // Recognize the characters in each line.
    let line_texts = engine.recognize_text(ocr_input, &line_rects)?;

    let line_texts: Vec<String> = line_texts
        .into_iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
        .map(|l| l.to_string())
        .collect();

    Ok(line_texts)
}



// async fn ocr<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<String>, String> {
//     let engine = init_ocr_engine().map_err(|e| e.to_string())?;

//     let img = include_bytes!("./test-img.png");
//     let img = image::load_from_memory(img).map_err(|e| e.to_string())?;
//     let img = img.into_rgb8();
    
//     let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())
//         .map_err(|e| e.to_string())?;
//     let ocr_input = engine.prepare_input(img_source).map_err(|e| e.to_string())?;
    
//     let line_texts = predict_from_image_buffer(&engine, &ocr_input).map_err(|e| e.to_string())?;

//     Ok(line_texts)
// }