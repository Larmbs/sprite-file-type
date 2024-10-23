use sprite_sheet_file_type::{SpriteSheet, mapping::{SpriteSheetMetaData, Rect}};

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
