
/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/



use ggez::graphics::{self};
use ggez::nalgebra::{Point2, Vector2};
use serde::{Deserialize};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Meta {
    size: AtlasSize,
}

#[derive(Deserialize, Debug)]
struct AtlasSize {
    w: i32,
    h: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct JsonRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct SpriteData {
    fliename: String,
    frame: JsonRect,
}

#[derive(Deserialize, Debug)]
pub struct Gaia {
    frames: Vec<SpriteData>,
    meta: Meta,
}



/*------------------------------------------------------- Auxiliary Function Implementation -------------------------------------------------------*/



impl Gaia {
    pub fn parse_atlas_json(texture_atlas_file: &Path) -> Self {
        use std::fs::File;
        use std::io::bufReader;
        let file = File::open(texture_atlas_file).expect("Gaia hasn't found the texture");
        let buf_reader = bufReader::new(file);
        serde_json::from_reader(buf_reader).expect("Gaia couldn't create the texture")
    }
}