use pixels_util::color::Color;
use std::io::Read;
use ureq::serde_json::{self, json};

use crate::prelude::CanvasError;

pub struct Client {
    token: String,
}

impl Client {
    pub fn new() -> Self {
        Client {
            token: String::new(),
        }
    }

    pub fn auth(&mut self, refresh: String) -> Result<(), CanvasError> {
        let body: serde_json::Value =
            ureq::post("https://pixels.yazilimcilarinmolayeri.com/authenticate")
                .send_json(json!({
                    "refresh_token": refresh,
                }))
                .map_err(|_| CanvasError::ClientError)?
                .into_json()
                .map_err(|_| CanvasError::ClientError)?;
        self.token = String::from(body["access_token"].as_str().unwrap());
        Ok(())
    }

    pub fn canvas_size(&self) -> Result<(u64, u64), CanvasError> {
        let body: serde_json::Value =
            ureq::get("https://pixels.yazilimcilarinmolayeri.com/canvas/size")
                .call()
                .map_err(|_| CanvasError::ClientError)?
                .into_json()
                .map_err(|_| CanvasError::ClientError)?;
        Ok((
            body["width"].as_u64().unwrap(),
            body["height"].as_u64().unwrap(),
        ))
    }

    pub fn canvas_pixels(&self) -> Result<Vec<u8>, CanvasError> {
        let mut buffer: Vec<u8> = vec![];
        ureq::get("https://pixels.yazilimcilarinmolayeri.com/canvas/pixels")
            .set(
                "Authorization",
                format!("Bearer {}", self.token.clone()).as_str(),
            )
            .call()
            .map_err(|_| CanvasError::ClientError)?
            .into_reader()
            .read_to_end(&mut buffer)
            .map_err(|_| CanvasError::ClientError)?;
        Ok(buffer)
    }

    pub fn canvas_set_pixel(
        &self,
        x: usize,
        y: usize,
        color: Color,
    ) -> Result<(u32, f32), CanvasError> {
        let res = ureq::put("https://pixels.yazilimcilarinmolayeri.com/canvas/pixel")
            .set(
                "Authorization",
                format!("Bearer {}", self.token.clone()).as_str(),
            )
            .send_json(json!({
                "x": x,
                "y": y,
                "rgb": color.to_string()
            }))
            .map_err(|_| CanvasError::ClientError)?;
        Ok((
            res.header("requests-remaining").unwrap().parse().unwrap(),
            res.header("requests-reset").unwrap().parse().unwrap(),
        ))
    }
}
