use ureq::serde_json::{
    self,
    json
};

use std::io::Read;

use pixels_util::prelude::*;

use crate::{
    token,
    url,
};


pub struct Client {
    token: String
}

pub type ClientError = ureq::Error;

impl Client {
    pub fn new() -> Self {
        Client {
            token: String::new()
        }
    }

    pub fn auth(&mut self, refresh: String) -> Result<(), ClientError> {
        let body: serde_json::Value = ureq::post(url!("authenticate"))
            .send_json(json!({
                "refresh_token": refresh,
            }))?.into_json()?;
        self.token = String::from(body["access_token"].as_str().unwrap());
        Ok(())
    }

    pub fn canvas_size(&self) -> Result<(u32, u32), ClientError> {
        let body: serde_json::Value = ureq::get(url!("canvas/size"))
            .call()?.into_json()?;
        Ok((
            body["width"].as_u64().unwrap() as u32,
            body["height"].as_u64().unwrap() as u32,
        ))
    }

    pub fn canvas_pixels(&self) -> Result<Vec<u8>, ClientError> {
        let mut buffer: Vec<u8> = vec![];
        ureq::get(url!("canvas/pixels"))
            .set(
                "Authorization",
                token!(self.token),
            )
            .call()?
            .into_reader()
            .read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn canvas_set_pixel(&self, x: u32, y: u32, color: Color) -> Result<(u32, f32), ClientError> {
        let res = ureq::put(url!("canvas/pixel"))
            .set(
                "Authorization",
                token!(self.token),
            )
            .send_json(json!({
                "x": x,
                "y": y,
                "rgb": color.to_hex(ColorMode::RGB),
            }))?;
        Ok((
            res.header("requests-remaining").unwrap().parse().unwrap(),
            res.header("requests-reset").unwrap().parse().unwrap(),
        ))
    }
}

#[macro_export]
macro_rules! url {
    ($path:expr) => {
        format!("https://pixels.yazilimcilarinmolayeri.com/{}", $path).as_str()
    };
}

#[macro_export]
macro_rules! token {
    ($token:expr) => {
        format!("Bearer {}", $token.clone()).as_str()
    };
}
