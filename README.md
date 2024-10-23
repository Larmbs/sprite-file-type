# SpriteSheet File Format

## Overview
This project aims to create a more convenient sprite sheet file format for use in game development and other graphics applications. Typically, sprite sheets consist of a single image alongside defined rectangles and sprite names to sample from that image. This project seeks to streamline that process.

## Features
- **Convenient Format**: A custom file format that allows for easier management and usage of sprite sheets.
- **Structured Metadata**: Clearly defined metadata for sprites, including names, dimensions, and positions.
- **Flexibility**: Supports various image formats and allows for easy integration into different projects.

## Example Usage

Here’s a sample code snippet demonstrating how to use the `spritesheet` library to create and manipulate sprite sheets.
```Rust
use sprite_sheet::{SpriteSheet, mapping::{SpriteSheetMetaData, Rect}};

fn main() {
    // Create metadata for the sprite
    let mut meta = SpriteSheetMetaData::empty();
    meta.mapping.insert(String::from("Hello"), Rect { x: 20, y: 20, w: 40, h: 60 });

    // Create a new SpriteSheet from an image file
    let sheet = SpriteSheet::new_from_path("cat.jpeg", meta).unwrap();

    // Save the sprite sheet in PNG format and its metadata in JSON
    sheet.save("sheet.png", "sheet.json").unwrap();

    // Save the sprite sheet in a raw binary format
    sheet.save_raw("raw_image.ssprite").unwrap();

    // Retrieve the sprite using its name
    let sprite = sheet.get_sprite(&String::from("Hello")).unwrap();
    println!("Retrieved sprite: {:?}", sprite.inner.dimensions());
}

```

### Dependencies
Ensure you have the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
image = "0.23"
```