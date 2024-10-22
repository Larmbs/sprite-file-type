use image::io::Reader;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serialimage::{DynamicSerialImage, SerialImageBuffer};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::{collections::HashMap, path::Path};

const VERSION_NUMBER: u8 = 0;

/// Rectangle representing bounds of image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Sprite image
pub struct Sprite {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

/// SpriteSheet containing sprites and their areas
pub struct SpriteSheet {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    mapping: HashMap<String, Rect>,
}
impl SpriteSheet {
    /// Constructor for a new SpriteSheet
    pub fn new(image: ImageBuffer<Rgba<u8>, Vec<u8>>, mapping: HashMap<String, Rect>) -> Self {
        Self { image, mapping }
    }

    /// Loads a sprite sheet from a file
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // Opening the file
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Reading the header of the file format
        let mut header_buf = [0u8; 13];
        reader.read_exact(&mut header_buf)?;

        // Parse header values
        let version = header_buf[0]; // Version number
        if version != VERSION_NUMBER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported file version",
            ));
        }

        let width = u32::from_le_bytes(header_buf[1..5].try_into().unwrap());
        let height = u32::from_le_bytes(header_buf[5..9].try_into().unwrap());
        let num_mappings = u32::from_le_bytes(header_buf[9..13].try_into().unwrap());

        // Read image data
        let mut image_data = vec![0u8; (width * height * 4) as usize]; // 4 bytes per pixel (RGBA)
        reader.read_exact(&mut image_data)?;

        // Create ImageBuffer from the raw image data
        let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, image_data)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid image data"))?;

        // Read mapping data
        let mut mapping = HashMap::new();
        for _ in 0..num_mappings {
            // Read string length
            let mut length_buf = [0u8; 4];
            reader.read_exact(&mut length_buf)?;
            let string_length = u32::from_le_bytes(length_buf);

            // Read string bytes
            let mut key_buf = vec![0u8; string_length as usize];
            reader.read_exact(&mut key_buf)?;
            let key = String::from_utf8(key_buf)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;

            // Read rectangle values
            let mut rect_buf = [0u8; 16]; // 4 bytes for each of x, y, w, h
            reader.read_exact(&mut rect_buf)?;
            let rect = Rect {
                x: u32::from_le_bytes(rect_buf[0..4].try_into().unwrap()),
                y: u32::from_le_bytes(rect_buf[4..8].try_into().unwrap()),
                w: u32::from_le_bytes(rect_buf[8..12].try_into().unwrap()),
                h: u32::from_le_bytes(rect_buf[12..16].try_into().unwrap()),
            };

            // Insert into mapping
            mapping.insert(key, rect);
        }

        Ok(Self::new(image, mapping))
    }

    /// Saves to a .sprite file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        // Opening the file
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Writing the header of the file format
        let mut header = Vec::with_capacity(13);
        header.push(VERSION_NUMBER); // 1 byte
        header.extend(self.image.width().to_le_bytes()); // 4 bytes
        header.extend(self.image.height().to_le_bytes()); // 4 bytes
        header.extend(self.mapping.len().to_le_bytes()); // 4 bytes

        writer.write(&header)?;

        // Writing image data
        let bytes = self.image.as_raw().to_owned();
        writer.write_all(&bytes)?;

        // Writing mapping data
        for (key, value) in self.mapping.iter() {
            writer.write(&(key.len() as u32).to_le_bytes())?;
            writer.write(key.as_bytes())?;
            let mut rect_values = Vec::with_capacity(16);
            rect_values.extend(value.x.to_le_bytes());
            rect_values.extend(value.y.to_le_bytes());
            rect_values.extend(value.w.to_le_bytes());
            rect_values.extend(value.h.to_le_bytes());
            writer.write(&rect_values)?;
        }
        Ok(())
    }

    /// Returns a sprite image
    pub fn get_sprite(&self, name: &String) -> Option<Sprite> {
        let rect = self.mapping.get(name)?;
        Some(Sprite {
            image: self.image.view(rect.x, rect.y, rect.w, rect.h).to_image(),
        })
    }
}
