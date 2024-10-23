//! SpriteSheet Meta Data
//! A structure representing the mappings between Sprite names and their Rect areas within an image.

use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Result, Write};
use std::path::Path;

/// MetaData Format Version
const MAPPING_VERSION_NUMBER: u8 = 0;

/// Rectangle Area of Sprite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// MetaData of SpriteSheet File Type
#[derive(Serialize, Deserialize, Clone)]
pub struct SpriteSheetMetaData {
    pub mapping: HashMap<String, Rect>,
}
impl SpriteSheetMetaData {
    /// SpriteSheet Meta -> JSON File
    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;

        Ok(())
    }

    /// JSON File -> SpriteSheet Meta
    pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    /// SpriteSheet Meta -> File
    pub fn save_raw<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_writer(&mut writer)?;
        writer.flush()?;
        Ok(())
    }

    /// File -> SpriteSheet Meta
    pub fn load_raw<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    /// SpriteSheet Meta -> Bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.to_writer(&mut bytes).unwrap();
        bytes
    }

    /// Bytes -> SpriteSheet Meta
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut reader = BufReader::new(Cursor::new(bytes));
        Self::from_reader(&mut reader)
    }

    /// SpriteSheet Meta -> Writer
    pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        /* Writing Header Data */
        writer.write(&[MAPPING_VERSION_NUMBER])?;
        writer.write(&(self.mapping.len() as u32).to_le_bytes())?;

        /* Writing Mapping Data */
        for (key, value) in self.mapping.iter() {
            // Writing String Data
            writer.write(&(key.bytes().len() as u32).to_le_bytes())?;
            writer.write(key.as_bytes())?;

            // Writing Rect Data
            let mut rect_values = Vec::with_capacity(16);
            rect_values.extend(value.x.to_le_bytes());
            rect_values.extend(value.y.to_le_bytes());
            rect_values.extend(value.w.to_le_bytes());
            rect_values.extend(value.h.to_le_bytes());
            writer.write(&rect_values)?;
        }
        writer.flush()?;

        Ok(())
    }

    /// Reader -> SpriteSheet Meta
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self> {
        /* Reading Header Data */
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let version_number = u8::from_le_bytes(buf);

        if version_number != MAPPING_VERSION_NUMBER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unsupported file version",
            ));
        }

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let entry_count = u32::from_le_bytes(buf);

        /* Reading Mapping Data */
        let mut mapping = HashMap::new();
        for _ in 0..entry_count {
            // Reading String Length
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            let string_length = u32::from_le_bytes(buf);

            // Reading The String
            let mut key_buf = vec![0u8; string_length as usize];
            reader.read_exact(&mut key_buf)?;
            let key = String::from_utf8(key_buf)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8 string"))?;

            // Reading Rect Data
            let mut rect_buf = [0u8; 16]; // 4 bytes for each of x, y, w, h
            reader.read_exact(&mut rect_buf)?;
            let rect = Rect {
                x: u32::from_le_bytes(rect_buf[0..4].try_into().unwrap()),
                y: u32::from_le_bytes(rect_buf[4..8].try_into().unwrap()),
                w: u32::from_le_bytes(rect_buf[8..12].try_into().unwrap()),
                h: u32::from_le_bytes(rect_buf[12..16].try_into().unwrap()),
            };

            // Inserting
            mapping.insert(key, rect);
        }

        Ok(Self { mapping })
    }

    pub fn empty() -> Self {
        Self { mapping: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        self.mapping.len()
    }
}
