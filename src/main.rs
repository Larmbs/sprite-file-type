
use sprite_sheet::{SpriteSheet, mapping::{SpriteSheetMetaData, Rect}};

fn main() {
    let mut meta = SpriteSheetMetaData::empty();
    meta.mapping.insert(String::from("Hello"), Rect{x: 20, y: 20, w: 40, h: 60});

    let sheet = SpriteSheet::new_from_path("cat.jpeg", meta).unwrap();
    sheet.save("sheet.png", "sheet.json").unwrap();

    sheet.save_raw("raw_image.ssprite").unwrap();
}

