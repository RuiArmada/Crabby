/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/

use ggez::ContextBuilder;
use std::path::PathBuf;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::conf::NumSamples;

/*------------------------------------------------------- Auxiliary Function Implementation -------------------------------------------------------*/

pub fn build_window(resource_dir: PathBuf) -> ContextBuilder {
    let cb: ContextBuilder = ggez::ContextBuilder::new("FlappyGina" , "By:RuiArmada")
        .add_resource_path(resource_dir)
        .window_setup(
            WindowSetup::default()
            .tile("Flappy Gina (/)(;,,;)(/)!!!")
            .samples(NumSamples::Zero)
            .vsync(true),
            )
        .window_mode(WindowMode::default().dimensions(800.0 , 600.0));
    cb
}

/*------------------------------------------------------------------------------------------------------------------------------------------------*/

