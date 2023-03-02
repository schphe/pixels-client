use macroquad::color::Color;
use bevy_ecs::prelude::*;
use serde_json::json;
use std::io::Read;
use crate::util;

#[derive(Resource)]
pub struct Client {
    token: String
}

impl Client {
    pub fn new() -> Self {
        return Client{
            token: String::new(),
        }
    }

    pub fn auth(&mut self, refresh: String) -> Result<(), ureq::Error> {
        let body: serde_json::Value = ureq::post("https://pixels.yazilimcilarinmolayeri.com/authenticate")
            .send_json(json!({
                "refresh_token": refresh,
            }))?.into_json()?;
        self.token = String::from(body["access_token"].as_str().unwrap());
        Ok(())
    }

    pub fn canvas_size(&self) -> Result<(u64, u64), ureq::Error> {
        let body: serde_json::Value = ureq::get("https://pixels.yazilimcilarinmolayeri.com/canvas/size")
            .call()?.into_json()?;
        Ok((body["width"].as_u64().unwrap(), body["height"].as_u64().unwrap()))
    }

    pub fn canvas_pixels(&self) -> Result<Vec<u8>, ureq::Error> {
        let mut buffer: Vec<u8> = vec![];
        ureq::get("https://pixels.yazilimcilarinmolayeri.com/canvas/pixels")
            .set("Authorization", format!("Bearer {}", self.token.clone()).as_str())
            .call()?.into_reader().read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn canvas_set_pixel(&self, x: u64, y: u64, color: Color) -> Result<(), ureq::Error> {
        let body: serde_json::Value = ureq::put("https://pixels.yazilimcilarinmolayeri.com/canvas/pixel")
            .send_json(json!({
                "x": x,
                "y": y,
                "rgb": util::color_to_hex(color)
            }))?.into_json()?;
        Ok(())
    }
}
