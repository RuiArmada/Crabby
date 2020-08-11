/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/


use crate::{atlas};
use crate::entity::{PlayState,PipeEntity};
use crate::atlas::Sprite;
use ggez::nalgebra::{Point2,Vector2};
use ggez::{Context,graphics,GameResult};
use ggez::graphics::spritebatch::SpriteBatch;

const GRAVITY: f32 = 0.25;
const JUMP_IMPULSE: f32 = 2.75;
pub const SCREEN_TOP: f32 = -16.0;

//Physics on the player
pub struct Physics{
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub gravity: bool,
}

pub struct PlayerEntity{
    pub sprite: Sprite,
    pub position: Point2<f32>,
    pub player_sprites: Vec<Sprite>,
    can_jump: bool,
    pub physics: Physics,
}



/*------------------------------------------------------- Auxiliary Function Implementation -------------------------------------------------------*/



impl Physics{
    pub fn new(with_gravity:bool) -> Self{
        Self{
            velocity: Vector2::new(0.0,0.0),
            acceleration: Vector2::new(0.0,0.0),
            gravity: with_gravity,
        }
    }
}



pub fn create_player(sprites: &atlas::Atlas) -> Box<PlayerEntity> {
    let crab0 = sprites.create_sprite("crab0.png");
    let sprite = crab0.clone;
    let crab1 = sprites.create_sprite("crab1.png");
    let player_sprites = vec![crab0,crab1];
    let player = PlayerEntity::new(sprite,(40.0,SCREEN_TOP), player_sprites);

    Box::new(player)
}