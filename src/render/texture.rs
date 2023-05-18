use core::fmt;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Deref,
    sync::Arc,
};

use serde::{Serialize, Deserialize, ser::SerializeStruct, de::{Visitor, self, SeqAccess, MapAccess}};

use super::{rgb_to_vec, Colour};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Texture {
    Colour(Colour),
    Image(Image),
}

impl Texture {
    pub fn colour_at(&self, u: f64, v: f64) -> Colour {
        match self {
            Self::Colour(c) => c.clone(),
            Self::Image(img) => {
                let (width, height) = img.dimensions();
                let mut i = (u.clamp(0.0, 1.0) * (width as f64)) as u32;
                let mut j = (v.clamp(0.0, 1.0) * (height as f64)) as u32;

                i = if i >= width { i - 1 } else { i };
                j = if j >= height { j - 1 } else { j };

                rgb_to_vec(img.get_pixel(i, j))
            }
        }
    }
}

impl From<Colour> for Texture {
    fn from(value: Colour) -> Self {
        Texture::Colour(value)
    }
}

impl From<Image> for Texture {
    fn from(value: Image) -> Self {
        Texture::Image(value)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Texture::Colour([0.5, 0.5, 0.5].into())
    }
}

#[derive(Clone, Eq)]
pub struct Image {
    image: Arc<image::RgbImage>,
    hash: u64,
}

impl Image {
    pub fn new(image: image::RgbImage) -> Self {
        let mut hasher = DefaultHasher::new();
        image.hash(&mut hasher);
        Self {
            image: Arc::new(image),
            hash: hasher.finish(),
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
    fn ne(&self, other: &Self) -> bool {
        self.hash != other.hash
    }
}

impl Hash for Image {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Deref for Image {
    type Target = image::RgbImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Image", 4)?;
        state.serialize_field("width", &self.width())?;
        state.serialize_field("height", &self.height())?;
        state.serialize_field("data", self.as_raw())?;
        state.serialize_field("hash", &self.hash)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

        enum Field { Width, Height, Data, Hash }
        const FIELDS: &'static [&'static str] = &["width", "height", "data", "hash"];
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`width` or `height` or `data` or `hash`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "width" => Ok(Field::Width),
                            "height" => Ok(Field::Height),
                            "data" => Ok(Field::Data),
                            "hash" => Ok(Field::Hash),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ImageVisitor;
        impl<'de> Visitor<'de> for ImageVisitor {
            type Value = Image;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Image")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Image, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let width = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let height = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let data = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let hash = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                Ok(Image {
                    image: Arc::new(image::RgbImage::from_raw(width, height, data).expect("Size and data should match")),
                    hash
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Image, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut width = None;
                let mut height = None;
                let mut data = None;
                let mut hash = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Width => {
                            if width.is_some() {
                                return Err(de::Error::duplicate_field("width"));
                            }
                            width = Some(map.next_value()?);
                        }
                        Field::Height => {
                            if height.is_some() {
                                return Err(de::Error::duplicate_field("height"));
                            }
                            height = Some(map.next_value()?);
                        }
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        Field::Hash => {
                            if hash.is_some() {
                                return Err(de::Error::duplicate_field("hash"));
                            }
                            hash = Some(map.next_value()?);
                        }

                    }
                }
                let width = width.ok_or_else(|| de::Error::missing_field("width"))?;
                let height = height.ok_or_else(|| de::Error::missing_field("height"))?;
                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let hash = hash.ok_or_else(|| de::Error::missing_field("hash"))?;
                Ok(Image {
                    image: Arc::new(image::RgbImage::from_raw(width, height, data).expect("Size and data should match")),
                    hash
                })
            }
        }
                
        deserializer.deserialize_struct("Image", FIELDS, ImageVisitor)
    }
}