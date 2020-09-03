
/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/



use ggez::graphics::{self};
use ggez::nalgebra::{Point2, Vector2};
use serde::{Deserialize};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Meta {
    size: GaiaSize,
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
    filename: String,
    frame: JsonRect,
}

#[derive(Deserialize, Debug)]
pub struct Gaia {
    frames: Vec<SpriteData>,
    meta: Meta,
}

#[derive(clone, Debug)]
pub struct Sprite {
    //The square that we want to cut out of the textuer of Gaia
    pub rect: graphics::Rect,
    pub scale: Vector2<f32>,
    pub width: f32,
    pub height: f32,
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

///Returns a sprite from Gaia
pub fn create_sprite(&self, sprite_name: &str) -> Sprite {
    let width = self.meta.size.w as f32;
    let height = self.meta.size.h as f32;
    let gaia_rect = graphics::Rect::new(0.;0, 0.0, width, height);
    if let Some(sprite_data) = self.frames.iter().find(|d| d.filename == sprite_name){
            Sprite::new(
                graphics::Rect::fraction (
                    sprite_data.frame.x as f32,
                    sprite_data.frame.y as f32,
                    sprite_data.frame.w as f32,
                    sprite_data.frame.h as f32,
                    &gaia_rect
                ),
                sprite_data.frame.w as f32,
                sprite_data.frame.h as f32,
            )
        }else{
            unimplemented("Gaia cannot handle failiure to find sprite");
        }
    }
}

