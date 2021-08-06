use chrono::{DateTime, Utc};

pub struct PictureMetadata {
    pub file_path: String,
    pub creation_date: DateTime<Utc>,
    pub height_pixels: u32,
    pub width_pixels: u32,
}

impl PictureMetadata {
    pub(crate) fn new(file_path: String, creation_date: DateTime<Utc>) -> Result<PictureMetadata, &'static str> {
        if file_path.is_empty() {
            return Err("file path is empty");
        }

        Ok(PictureMetadata { file_path, creation_date, height_pixels: 0, width_pixels: 0 })
    }
}

impl From<&PictureMetadata> for String {
    fn from(a_pic_metadata: &PictureMetadata) -> Self {
        let mut result: String = a_pic_metadata.file_path.clone();
        result.push_str(";");
        result.push_str(&*a_pic_metadata.creation_date.to_string());
        result.push_str(";");
        result.push_str(&*a_pic_metadata.height_pixels.to_string());
        result.push_str(";");
        result.push_str(&*a_pic_metadata.width_pixels.to_string());
        result
    }
}
