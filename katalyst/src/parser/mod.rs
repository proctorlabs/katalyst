use crate::prelude::*;

pub enum Format {
    Default,
    Json,
    Yaml,
}

impl Format {
    pub fn ext(ext: Option<&str>) -> Format {
        match ext {
            Some("yml") | Some("yaml") => Format::Yaml,
            Some("json") | Some("js") => Format::Json,
            _ => Format::Default,
        }
    }

    pub fn content_type(content_type: Option<&str>) -> Format {
        if let Some(ct) = content_type {
            match ct {
                "application/json" | "application/javascript" => Format::Json,
                "application/x-yaml" | "text/vnd.yaml" | "text/yaml" | "text/x-yaml" => {
                    Format::Yaml
                }
                _ => Format::Default,
            }
        } else {
            Format::Default
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn from_str<T: serde::de::DeserializeOwned>(
        ser: &str,
        f: Format,
    ) -> Result<T, KatalystError> {
        match f {
            Format::Json | Format::Default => {
                serde_json::from_str(ser).map_err(|_| KatalystError::FeatureUnavailable)
            }
            Format::Yaml => {
                serde_yaml::from_str(ser).map_err(|_| KatalystError::FeatureUnavailable)
            }
        }
    }

    pub fn from_slice<T: serde::de::DeserializeOwned>(
        ser: &[u8],
        f: Format,
    ) -> Result<T, KatalystError> {
        match f {
            Format::Json | Format::Default => {
                serde_json::from_slice(ser).map_err(|_| KatalystError::FeatureUnavailable)
            }
            Format::Yaml => {
                serde_yaml::from_slice(ser).map_err(|_| KatalystError::FeatureUnavailable)
            }
        }
    }
}
