
use sprite_sheet::{SpriteSheet, Image, mapping::{SpriteSheetMetaData, Rect}};
use image::*;
use std::io::{ErrorKind, Write};
use std::io::Cursor;

const BIN_PATH: &str = "tests/bin/";

// Helper function to create a sample image
fn create_sample_image() -> Image {
    let width = 2;
    let height = 2;
    let mut img: RgbaImage = ImageBuffer::new(width, height);

    img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Red
    img.put_pixel(1, 0, Rgba([0, 255, 0, 255])); // Green
    img.put_pixel(0, 1, Rgba([0, 0, 255, 255])); // Blue
    img.put_pixel(1, 1, Rgba([255, 255, 0, 255])); // Yellow

    img
}

// Helper function to create sample metadata
fn create_sample_metadata() -> SpriteSheetMetaData {
    let mut meta = SpriteSheetMetaData::empty();
    meta.mapping.insert(
        "sprite1".to_string(),
        Rect {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        },
    );
    meta.mapping.insert(
        "sprite2".to_string(),
        Rect {
            x: 1,
            y: 0,
            w: 1,
            h: 1,
        },
    );
    meta
}

#[test]
fn test_image_raw_to_writer_and_image_raw_from_reader() {
    let image = create_sample_image();

    // Create a writer using a cursor
    let mut writer = Cursor::new(Vec::new());
    SpriteSheet::image_raw_to_writer(&image, &mut writer).unwrap();

    // Read back from the cursor
    let mut reader = Cursor::new(writer.into_inner());
    let result_image = SpriteSheet::image_raw_from_reader(&mut reader).unwrap();

    // Assert that the written and read images are the same
    assert_eq!(image.dimensions(), result_image.dimensions());
    assert_eq!(image.as_raw(), result_image.as_raw());
}

#[test]
fn test_save_raw_and_load_raw() {
    let image = create_sample_image();
    let meta = create_sample_metadata();
    let sprite_sheet = SpriteSheet::new(image.clone(), meta.clone());

    let path = format!("{}{}", BIN_PATH, "test_sprite_sheet.bin");
    sprite_sheet.save_raw(&path).unwrap();

    let loaded_sprite_sheet = SpriteSheet::load_raw(&path).unwrap();

    // Compare loaded image and metadata with original
    assert_eq!(image.dimensions(), loaded_sprite_sheet.image.dimensions());
    assert_eq!(image.as_raw(), loaded_sprite_sheet.image.as_raw());
    assert_eq!(meta.mapping.len(), loaded_sprite_sheet.meta.mapping.len());

    std::fs::remove_file(&path).unwrap(); // Cleanup test file
}

#[test]
fn test_save_and_load() {
    let image = create_sample_image();
    let meta = create_sample_metadata();
    let sprite_sheet = SpriteSheet::new(image.clone(), meta.clone());

    let image_path = format!("{}{}", BIN_PATH, "test_sprite_sheet_image.png");
    let meta_path = format!("{}{}", BIN_PATH, "test_sprite_sheet_meta.json");

    sprite_sheet.save(&image_path, &meta_path).unwrap();

    let loaded_sprite_sheet = SpriteSheet::load(&image_path, &meta_path).unwrap();

    // Compare loaded image and metadata with original
    assert_eq!(image.dimensions(), loaded_sprite_sheet.image.dimensions());
    assert_eq!(image.as_raw(), loaded_sprite_sheet.image.as_raw());
    assert_eq!(meta.mapping.len(), loaded_sprite_sheet.meta.mapping.len());

    std::fs::remove_file(&image_path).unwrap(); // Cleanup test file
    std::fs::remove_file(&meta_path).unwrap(); // Cleanup test file
}

#[test]
fn test_get_sprite() {
    let image = create_sample_image();
    let meta = create_sample_metadata();
    let sprite_sheet = SpriteSheet::new(image.clone(), meta.clone());

    // Test that sprites are correctly extracted
    let sprite1 = sprite_sheet.get_sprite(&"sprite1".to_string()).unwrap();
    let sprite2 = sprite_sheet.get_sprite(&"sprite2".to_string()).unwrap();

    assert_eq!(sprite1.inner.dimensions(), (1, 1));
    assert_eq!(sprite1.inner.get_pixel(0, 0), &Rgba([255, 0, 0, 255])); // Red sprite

    assert_eq!(sprite2.inner.dimensions(), (1, 1));
    assert_eq!(sprite2.inner.get_pixel(0, 0), &Rgba([0, 255, 0, 255])); // Green sprite
}

#[test]
fn test_version_mismatch() {
    let image = create_sample_image();

    // Write image with wrong version number
    let mut writer = Cursor::new(Vec::new());
    writer.write_all(&[99]).unwrap(); // Invalid version number
    writer.write_all(&image.width().to_le_bytes()).unwrap();
    writer.write_all(&image.height().to_le_bytes()).unwrap();
    writer.write_all(image.as_raw()).unwrap();

    // Attempt to read back
    let mut reader = Cursor::new(writer.into_inner());
    let result = SpriteSheet::image_raw_from_reader(&mut reader);

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "Unsupported file version");
    }
}
