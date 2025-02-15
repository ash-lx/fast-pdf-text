use std::path::Path;
use std::fs;
use thiserror::Error;
use lopdf::Document;
use serde_json::json;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unsupported file format")]
    UnsupportedFormat,
    #[error("Failed to read file: {0}")]
    FileReadError(String),
    #[error("Failed to parse document: {0}")]
    ParseError(String),
    #[error("Failed to write JSON: {0}")]
    JsonWriteError(String),
}

pub trait DocumentParser {
    fn parse(&self, path: &Path) -> Result<String, ParserError>;
}

pub struct PDFParser;

impl DocumentParser for PDFParser {
    fn parse(&self, path: &Path) -> Result<String, ParserError> {
        // Load document
        let doc = Document::load(path)
            .map_err(|e| ParserError::ParseError(e.to_string()))?;

        // Extract text from all pages
        let mut text = String::new();
        for page_num in 1..=doc.get_pages().len() {
            if let Ok(page_text) = doc.extract_text(&[page_num as u32]) {
                text.push_str(&page_text);
                text.push('\n');
            }
        }

        Ok(text)
    }
}

pub struct ResumeParser {
    pdf_parser: PDFParser,
}

impl ResumeParser {
    pub fn new() -> Self {
        Self {
            pdf_parser: PDFParser,
        }
    }

    pub fn parse_and_save(&self, input_path: &Path) -> Result<(), ParserError> {
        // Verify the file is a PDF
        if input_path.extension().and_then(|ext| ext.to_str()) != Some("pdf") {
            return Err(ParserError::UnsupportedFormat);
        }

        // Extract raw text
        let raw_text = self.pdf_parser.parse(input_path)?;

        // Create JSON structure
        let json_content = json!({
            "raw_resume": raw_text
        });

        // Save to resume.json
        fs::write(
            "resume.json", 
            serde_json::to_string_pretty(&json_content).map_err(|e| ParserError::JsonWriteError(e.to_string()))?
        ).map_err(|e| ParserError::FileReadError(e.to_string()))?;

        Ok(())
    }
}

fn main() {
    let parser = ResumeParser::new();
    let path = Path::new("resume.pdf");
    
    match parser.parse_and_save(path) {
        Ok(()) => println!("Successfully extracted raw text and saved to resume.json"),
        Err(e) => eprintln!("Error: {}", e),
    }
}