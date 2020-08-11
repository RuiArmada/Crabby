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



impl PlayerEntity {
   pub fn update(
       &mut self,
       ctx: &mut Context,
       state: &PlayState,
   ) -> PlayState {
       let physics = &mut self.physics;
       physics.acceleration = if physics.gravity {
           Vector2::new(0.0,GRAVITY)
       }else{
           Vector2::new(0.0,0.0)
       };
       let mut state = state.clone();
       if state.is_not_dead(){
           use ggez::event::KeyCode;
           use ggez::input::keyboard;
           if !keyboard::pressed_keys(ctx).contains(&KeyCode::Space) && !self.can_jump{
               self.can_jump = true;
           }
           if keyboard::is_key_pressed(ctx, KeyCode::Space) && self.can_jump{
               let physics = &mut self.physics;
               PlayerEntity::jump(physics);
               //exit start screen state
               if state == PlayState::StartScreen{
                   state = PlayState::Play;
               }
           }
       }
       //Self jumping script on Start Screen
       if state == PlayState::StartScreen{
           self.auto_jump()
       }
       self.change_player_position(ctx);

       //Gina should not fly above the top of screen easily. She might get burned by the Sun :(
       self.prevent_going_off();
       state
   }

   fn change_player_position(&mut self, ctx:&mut Context){
       let delta = ggez::timer::delta(ctx).as_nanos() as f32;
       let physics = &mut self.physics;
       physics.acceleration.scale(1.0/delta);
       physics.velocity += physics.acceleration;
       physics.velocity.scale(1.0/delta);
       //moves all thing on the board
       self.position += physics.velocity;
   }

   pub fn new(sprite: Sprite, position: (f32,f32), player_sprites: Vec<Sprite>) -> Self {
       Self{
           sprite,
           position: Point2::new(position.0,position.1),
           physics: Physics::new(true),
           can_jump: true,
           player_sprites,
       } 
   }
}