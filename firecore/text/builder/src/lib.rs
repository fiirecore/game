use std::io::Write;
use std::path::Path;

use firecore_font::{FontSheet, FontSheetFile, SerializedFonts};

pub fn compile<P: AsRef<Path>>(font_folder: P, output_file: P) {
    let font_folder = font_folder.as_ref();
    let output_file = output_file.as_ref();

    let mut fonts = Vec::new();

    println!("Reading fonts...");

    for entry in std::fs::read_dir(font_folder)
        .unwrap_or_else(|err| panic!("Could not read font folder with error {}", err))
    {
        match entry.map(|entry| entry.path()) {
            Ok(file) => {
                if file.is_file() {
                    let content = std::fs::read_to_string(&file)
                        .unwrap_or_else(|err| panic!("Could not read file at {:?} to string with error {}", file, err));
                    let font_sheet_file: FontSheetFile = ron::from_str(&content)
                        .unwrap_or_else(|err| panic!("Could not parse file at {:?} with error {}", file, err));
                    let image = std::fs::read(font_folder.join(&font_sheet_file.file))
                        .unwrap_or_else(|err| panic!("Could not read image file at {} for sheet #{} with error {}", font_sheet_file.file, font_sheet_file.data.id, err));
                    fonts.push(FontSheet {
                        image,
                        data: font_sheet_file.data,
                    });
                }
            }
            Err(err) => eprintln!("Could not read directory entry with error {}", err),
        }        
    }
    
    println!("Serializing fonts...");
    let bytes = firecore_dependencies::ser::serialize(&SerializedFonts { fonts })
        .unwrap_or_else(|err| panic!("Could not serialize fonts with error {}", err));

    println!("Creating and writing to file...");
    let bytes = std::fs::File::create(output_file)
        .unwrap_or_else(|err| panic!("Could not create output file at {:?} with error {}", output_file, err))
            .write(&bytes)
                .unwrap_or_else(|err| panic!("Could not write to output file at {:?} with error {}", output_file, err));
    println!("Wrote {} bytes to font file!", bytes);
}