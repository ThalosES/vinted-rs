use std::fmt;

use crate::model::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: i64,
    pub url: String,
    pub dominant_color: String,
    pub dominant_color_opaque: String,
}

impl Photo {
    pub async fn visualize_picture(&self) -> Result<String, reqwest::Error> {
        if self.url.is_empty() {
            return Ok("No image URL provided".to_string());
        }

        let response = reqwest::get(&self.url).await?;
        if !response.status().is_success() {
            return Ok("Failed to retrieve image".to_string());
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");

        if content_type.starts_with("image/") {
            Ok(format!("Image: [{}]", self.url))
        } else {
            Ok("Invalid image URL".to_string())
        }
    }
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Dominant Color: {}", self.dominant_color)?;
        writeln!(f, "Dominant Color Opaque: {}", self.dominant_color_opaque)?;

        Ok(())
    }
}
