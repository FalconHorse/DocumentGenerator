use crate::zipper::zip_and_return;
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use genpdf::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Cursor, Read, Seek, SeekFrom};
use genpdf::elements::Paragraph;

pub type FileMap = HashMap<String, Vec<u8>>;

pub async fn identify_type_and_transform(mut multipart: Multipart) -> Vec<u8> {
    let mut out: FileMap = HashMap::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = field.file_name();
        let content_type = field.content_type();
        let data = (file_name, content_type);

        if let (Some(file_name), Some(content_type)) = data {
            match content_type {
                "text/plain" => {
                    let file_name = file_name.to_owned();
                    let bytes = field.bytes().await.unwrap();
                    let bytes = bytes.to_vec();
                    let vec = transform_to_pdf_from_txt(bytes, file_name.as_str());
                    out.insert(file_name.to_string(), vec);
                }
                "text/html" => {
                    let vec = transform_to_pdf_from_html(&field, &file_name);
                    out.insert(file_name.to_string(), vec);
                }
                _ => println!("content_type: {}, file_name: {}", content_type, file_name),
            }
        }
    }

    zip_and_return(out).await.unwrap_or_else(|err| Vec::new())
}

fn transform_to_pdf_from_txt(
    // field: &Field,
    bytes: Vec<u8>,
    file_name: &str,
) -> Vec<u8> {
    // Load a font from the file system
    let font_family =
        fonts::from_files("src/fonts", "Arial", None).expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = Document::new(font_family);
    // Change the default settings
    doc.set_title(file_name);

    // Customize the pages
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // let bytes = field.bytes().await; // Your method to get bytes from the field
    let reader = BufReader::new(Cursor::new(bytes));
    for line in reader.lines() {
        match line {
            Ok(line) => {
                doc.push(Paragraph::new(&line));
                //println!("I found the line {}", line)
            },
            _ => {}
        }
    }

    // Create fake "file"
    let mut c = Cursor::new(Vec::new());
    doc.render(&mut c).expect("Failed to render PDF");

    let mut out = Vec::new();

    c.seek(SeekFrom::Start(0)).unwrap();
    c.read_to_end(&mut out).unwrap();

    out
}

fn transform_to_pdf_from_html(field: &Field, file_name: &str) -> Vec<u8> {
    unimplemented!("transform_to_pdf_from_html function is not implemented yet");
    let mut doc = default_pdf_settings();
}

fn default_pdf_settings() -> Document {
    // Load a font from the file system
    let font_family =
        fonts::from_files("src/fonts", "Arial", None).expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = Document::new(font_family);
    // Change the default settings
    doc.set_title("My first PDF document");

    // Customize the pages
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc
}
