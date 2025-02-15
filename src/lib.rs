use wasm_bindgen::prelude::*;
use lopdf::Document;
use std::io::Cursor;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct ExtractionResult {
    text: String,
    page_count: usize,
}

#[wasm_bindgen]
pub struct ResumeParser;

#[wasm_bindgen]
impl ResumeParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        log("ResumeParser instance created");
        ResumeParser
    }

    #[wasm_bindgen]
    pub fn parse_pdf(&self, data: &[u8]) -> Result<JsValue, JsError> {
        log(&format!("Starting to parse PDF of size: {} bytes", data.len()));
        
        // Create a cursor from the byte array
        let cursor = Cursor::new(data);
        
        // Load and parse PDF
        log("Attempting to load PDF document");
        let doc = Document::load_from(cursor)
            .map_err(|e| {
                log(&format!("Error loading PDF: {}", e));
                JsError::new(&format!("Failed to parse PDF: {}", e))
            })?;

        let page_count = doc.get_pages().len();
        log(&format!("PDF loaded successfully with {} pages", page_count));

        // Extract text from all pages
        let mut text = String::new();
        for page_num in 1..=page_count {
            log(&format!("Processing page {}", page_num));
            if let Ok(page_text) = doc.extract_text(&[page_num as u32]) {
                text.push_str(&page_text);
                text.push('\n');
            }
        }

        log(&format!("Extracted {} characters of text", text.len()));
        
        let result = ExtractionResult {
            text,
            page_count,
        };

        Ok(serde_wasm_bindgen::to_value(&result)?)
    }
}