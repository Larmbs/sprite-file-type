//! SpriteSheet File Format

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Error, ErrorKind, Read, Result, Write};
use std::path::Path;

pub mod mapping;
use mapping::SpriteSheetMetaData;

/// Version number of the format
const FORMAT_VERSION_NUMBER: u8 = 0;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Sprite Image
pub struct Sprite {
    pub inner: Image,
}

/// SpriteSheet File Type
pub struct SpriteSheet {
    pub image: Image,
    pub meta: SpriteSheetMetaData,
}
impl SpriteSheet {
    pub fn new(image: Image, meta: SpriteSheetMetaData) -> Self {
        Self { image, meta }
    }

    /// Image -> Writer
    pub fn image_raw_to_writer<W: Write>(image: &Image, writer: &mut W) -> Result<()> {
        /* Writing Image Header Data */
        writer.write(&[FORMAT_VERSION_NUMBER])?;
        writer.write(&image.width().to_le_bytes())?;
        writer.write(&image.height().to_le_bytes())?;

        /* Writing Raw Image Data */
        let bytes = image.as_raw().to_owned();
        writer.write_all(&bytes)?;

        writer.flush()?;

        Ok(())
    }

    /// Reader -> Image
    pub fn image_raw_from_reader<R: Read>(reader: &mut R) -> Result<Image> {
        /* Reading Image Header */
        // Version Number
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let version_number = u8::from_le_bytes(buf);

        if version_number != FORMAT_VERSION_NUMBER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unsupported file version",
            ));
        }

        // Width
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let width = u32::from_le_bytes(buf);

        // Height
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let height = u32::from_le_bytes(buf);

        /* Reading Raw Image Data */
        let mut buf = vec![0u8; (width * height * 4) as usize];
        reader.read_exact(&mut buf)?;
        println!("{:?}", &buf);
        let image: Image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, buf)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid image data"))?;

        Ok(image)
    }

    /// File Bytes -> SpriteSheet
    pub fn load_raw<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let image = Self::image_raw_from_reader(&mut reader)?;
        let meta = SpriteSheetMetaData::from_reader(&mut reader)?;

        Ok(Self::new(image, meta))
    }

    /// SpriteSheet -> FileBytes
    pub fn save_raw<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // Opening the file
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        Self::image_raw_to_writer(&self.image, &mut writer)?;
        self.meta.to_writer(&mut writer)?;
        writer.flush()?;

        Ok(())
    }

    /// SpriteSheet -> Meta JSON File & Image File
    pub fn save<P1: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        image_path: P1,
        meta_path: P2,
    ) -> io::Result<()> {
        let image: DynamicImage = self.image.clone().into();
        image
            .save(image_path)
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;

        self.meta.save_json(meta_path)?;
        Ok(())
    }

    /// Meta JSON File & Image File -> SpriteSheet
    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>>(
        image_path: P1,
        meta_path: P2,
    ) -> Result<Self> {
        let image = ImageReader::open(image_path)?
            .decode()
            .map_err(|err| Error::new(ErrorKind::Other, Error::new(ErrorKind::Other, err.to_string())))?
            .into();

        let meta = SpriteSheetMetaData::load_json(meta_path)?;

        Ok(Self { image, meta })
    }

    /// Allows you to create a SpriteSheet by providing a image path
    pub fn new_from_path<P: AsRef<Path>>(path: P, meta: SpriteSheetMetaData) -> Result<Self> {
        let image = ImageReader::open(path)?
            .decode()
            .map_err(|err| Error::new(ErrorKind::Other, Error::new(ErrorKind::Other, err.to_string())))?
            .into();
        Ok(Self { image, meta })
    }
    /// Returns a sprite image
    pub fn get_sprite(&self, name: &String) -> Option<Sprite> {
        let rect = self.meta.mapping.get(name)?;
        Some(Sprite {
            inner: self.image.view(rect.x, rect.y, rect.w, rect.h).to_image(),
        })
    }
}
