
use sprite_file_type::{SpriteSheet, mapping::{SpriteSheetMetaData, Rect}};

fn main() {
    let mut meta = SpriteSheetMetaData::new();
    meta.mapping.insert(String::from("Hello"), Rect{x: 20, y: 20, w: 40, h: 60});

    let sheet = SpriteSheet::new_from_path("cat.jpeg", meta).unwrap();
    sheet.save("sheet.png", "sheet.json").unwrap();
}

