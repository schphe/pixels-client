use pixels_util::color::Color;
use ureq::serde_json::{
    self, json
};
use std::io::Read;


pub struct Client {
    token: String
}

impl Client {
    pub fn new() -> Self {
        Client{
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

    pub fn canvas_set_pixel(&self, x: usize, y: usize, color: Color) -> Result<(u32, f32), ureq::Error> {
        let res = ureq::put("https://pixels.yazilimcilarinmolayeri.com/canvas/pixel")
            .set("Authorization", format!("Bearer {}", self.token.clone()).as_str())
            .send_json(json!({
                "x": x,
                "y": y,
                "rgb": color.to_hex()
            }))?;
        Ok((
            res.header("requests-remaining").unwrap().parse().unwrap(),
            res.header("requests-reset").unwrap().parse().unwrap()
        ))
    }
}
