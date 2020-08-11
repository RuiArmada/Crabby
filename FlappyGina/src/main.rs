
/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/

use atlas::Sprite;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{
    event::EventHandler,
    graphics::{spritebatch::SpriteBatch, Text},
    Context,
    GameResult,
    graphics,
    event
};

mod entity;
mod atlas;
mod pipe;
mod game_state;
mod crab;
mod audio;
mod window;
mod title;
use entity::PlayState;
use std::time::Duration;
use crate::crab::PlayerEntity;
use crate::game_state::GameState;

pub const NUMBER_OF_TILES: u8 = 14;
pub const RESTART_AFTER: Duration=std::time::Duration::from_secs(1);

/*--------------------------------------------------------- Basic Function Implementation ---------------------------------------------------------*/

impl EventHandler for GameState{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        let state = self.play_state.clone();
        self.handle_after_losing(ctx,state);
        let state = self.player.update(ctx,&self.play_state);
        if !self.play_state.is_playing() && state==PlayState::Play{
            self.play_state=PlayState::Play;
        }
        for i in 0..self.pipes.len(){
            self.pipes[i].update(&mut self.pipe_tracker, &self.play_state);
        }
        update_it(self,ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx,graphics::Color::from_rgb(112,216,255));

        self.player.draw(&mut self.sprite_batch)?;
        if !self.tiles_drawn{
                for i in 0..self.tiles.len(){
                    self.tiles[i].draw(&mut self.sprite_batch);
                }
                self.tiles_drawn = false;
        }
        for i in 0..self.pipes.len(){
            self.pipes[i].draw(ctx,&mut self.sprite_batch)?;
            
        }
        let p = graphics::DrawParam::new().scale(Vector2::new(4.0,4.0));
        {
            graphics::draw(ctx,&mut self.sprite_batch,p)?;
            self.sprite_batch.clear();
        }
        draw_scores(self.score,self.best_score,ctx);
        graphics::present(ctx)?;
        std::thread::yield_now();

        Ok(())
    }
}



fn update_it(game: &mut GameState, ctx: &mut Context){
    let player = &game.player;
    let pipes = &mut game.pipes;
    for i in 0..pipes.len(){
        
    }
}