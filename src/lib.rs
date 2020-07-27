use derive_builder::Builder;
use serde::Serialize;
#[allow(unused_imports)]
use std::io::Write;
pub mod error;
pub mod pango_markup;
//Yeah I got no clue why builder dosen't work.
#[derive(Builder, Default, Debug, Serialize)]
#[allow(non_camel_case_types)]
///This struct represents a Json Object that will
///Be sent to Waybar and be read.
pub struct WayBarJsonObj {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u8>,
}

impl WayBarJsonObj {
    pub fn new(text: String) -> WayBarJsonObj {
        WayBarJsonObj {
            text,
            tooltip: None,
            class: None,
            percentage: None,
        }
    }

    pub fn send(&self) -> Result<(), error::Error> {
        serde_json::to_writer(std::io::stdout(), &self)?;
        println!();
        Ok(())
    }
}

