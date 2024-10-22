use bincode;
use image::Pixel;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::ops::Deref;
use std::{collections::HashMap, path::Path};

/// Custom serialization for ImageBuffer
fn serialize_image<S, P, C>(image: &ImageBuffer<P, C>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    P: Pixel,
    C: Deref<Target = [<PI as Pixel>::Subpixel]>,
{
    todo!()
}

/// Custom deserialization for ImageBuffer
fn deserialize_image<'de, D, P, C>(deserializer: D) -> Result<ImageBuffer<P, C>, D::Error>
where
    D: Deserializer<'de>,
    P: Pixel + 'static,
    C: Deref<Target = [<PI as Pixel>::Subpixel]>,
{
    todo!()
}

/// Rectangle representing bounds of image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Sprite image
pub struct Sprite<PI, C>
where
    PI: Pixel + 'static,
    C: Deref<Target = [<PI as Pixel>::Subpixel]>,
{
    pub image: ImageBuffer<PI, C>,
}

/// SpriteSheet containing sprites and their areas
#[derive(Serialize, Deserialize)]
pub struct SpriteSheet<PI, C>
where
    PI: Pixel + 'static,
    C: Deref<Target = [<PI as Pixel>::Subpixel]>,
{
    #[serde(
        serialize_with = "serialize_image",
        deserialize_with = "deserialize_image"
    )]
    image: ImageBuffer<PI, C>,
    mapping: HashMap<String, Rect>,
}

impl<PI, C> SpriteSheet<PI, C>
where
    PI: Pixel + 'static,
    C: Deref<Target = [<PI as Pixel>::Subpixel]>,
{
    /// Constructor for a new SpriteSheet
    fn new(image: ImageBuffer<PI, C>, mapping: HashMap<String, Rect>) -> Self {
        Self { image, mapping }
    }

    /// Loads from a .sprite file (assumed to be a binary file for simplicity)
    fn load<P: AsRef<Path>>(path: P) -> io::Result<SpriteSheet<Rgba<u8>, Vec<u8>>> {
        todo!()
    }

    /// Saves to a .sprite file
    fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        todo!()
    }

    /// Returns a sprite image
    fn get_sprite(&self, name: &String) -> Option<Sprite<PI, C>> {
        let rect = self.mapping.get(name)?;
        Some(Sprite {
            image: self.image.view(rect.x, rect.y, rect.w, rect.h).into(),
        })
    }
}
