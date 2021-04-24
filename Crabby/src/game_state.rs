/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/


use ggez::{graphics::spritebatch::SpriteBatch, Context};

use crate::entity::{PipeEntity, PlayState};
use crate::gina::{create_player, PlayerEntity};
use crate::pipes::{create_pipes, PipeTracker};
use crate::tile::{create_tiles, TileEntity};
use crate::{audio, gaia, pipes, RESTART_AFTER};
use audio::Player;

pub struct GameState {
    pub tiles_drawn: bool,
    pub pipes: Vec<Box<PipeEntity>>,
    pub tiles: Vec<Box<TileEntity>>,
    pub player: Box<PlayerEntity>,
    ///The sprite batch of all the images
    pub sprite_batch: SpriteBatch,
    ///The struct that moves the pipes around
    ///Can use any function over time between 0 and 600/16
    pub pipe_tracker: PipeTracker,
    pub play_state: PlayState,
    gaia: gaia::Gaia,
    pub score: i128,
    pub best_score: i128,
    pub sound_player: audio::Player,
}


/*------------------------------------------------------- Auxiliary Function Implementation -------------------------------------------------------*/


impl GameState {
    pub fn handle_after_losing(&mut self, ctx: &mut Context, state: PlayState) {
        match state {
            PlayState::Dead { time } => {
                if (ggez::timer::time_since_start(ctx) - time) > RESTART_AFTER {
                    self.restart()
                }
            }
            _ => {}
        }
    }

    /// Creates a new GameState
    ///Burns if can't access the sprite image resource
    pub fn new(ctx: &mut Context, sprite_batch: SpriteBatch) -> Self {
        let mut pipe_tracker = pipes::PipeTracker::new();
        let gaia =
            gaia::Gaia::parse_atlas_json(std::path::Path::new("resources/texture_atlas.json"));
        let sound_player = Player::new(ctx);

        Self {
            tiles_drawn: false,
            pipes: GameState::crate_start_entities(&gaia, &mut pipe_tracker),
            player: create_player(&gaia),
            tiles: create_tiles(&gaia),
            sprite_batch,
            pipe_tracker,
            play_state: PlayState::StartScreen,
            gaia,
            score: 0,
            best_score: 0,
            sound_player,
        }
    }

    pub fn crate_start_entities(
        sprites: &gaia::Gaia,
        pipe_tracker: &mut PipeTracker,
    ) -> Vec<Box<PipeEntity>> {
        let pipes = create_pipes(
            sprites.create_sprite("pipe_bottom.png"),
            sprites.create_sprite("pipe_top.png"),
            pipe_tracker,
            200.0,
        );
        pipes
    }

    pub fn restart(&mut self) {
        self.sound_player.begin();
        let mut pt = PipeTracker::new();
        self.pipes = GameState::crate_start_entities(&self.gaia, &mut pt);
        self.player = create_player(&self.gaia);
        self.pipe_tracker = pt;
        self.play_state = PlayState::StartScreen;
        self.swap_score();
        self.score = 0;
    }

    pub fn swap_score(&mut self) {
        if self.score > self.best_score {
            self.best_score = self.score;
        }
    }
}


/*------------------------------------------------------------------------------------------------------------------------------------------------*/
