use std::collections::HashMap;

use sprite_file_type::{SpriteSheet, Rect};

fn main() {
    // let mut mapping = HashMap::new();
    // mapping.insert("Hello".into(), Rect {x: 0, y: 0, w: 100, h: 100});
    // mapping.insert("World".into(), Rect {x: 0, y: 0, w: 50, h: 50});
    // mapping.insert("Hi".into(), Rect {x: 0, y: 0, w: 25, h: 25});


    // let img = image::ImageReader::open("cat.jpeg").unwrap().decode().unwrap().into();
    // let sheet = SpriteSheet::new(img, mapping);
    // let sprite = sheet.get_sprite(&String::from("Hello")).unwrap();

    // sprite.image.save("out2.png").unwrap();
    // sheet.save("out.sprite").unwrap();

    let sheet = SpriteSheet::load("out.sprite").unwrap();

    let sprite = sheet.get_sprite(&String::from("World")).unwrap();

    sprite.image.save("out2.png").unwrap();

}
