/*------------------------------------------------------ Constant Definition/Library Calling ------------------------------------------------------*/


use ggez::conf::NumSamples;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::ContextBuilder;
use std::path::PathBuf;


/*------------------------------------------------------- Auxiliary Function Implementation -------------------------------------------------------*/


pub fn build_window(resource_dir: PathBuf) -> ContextBuilder {
    let cb: ContextBuilder = ggez::ContextBuilder::new("FlappyGina", "By:RuiArmada")
        .add_resource_path(resource_dir)
        .window_setup(
            WindowSetup::default()
                .title("Flappy Gina (/)(;,,;)(/)!!!")
                .samples(NumSamples::Zero)
                .vsync(true),
        )
        .window_mode(WindowMode::default().dimensions(800.0, 600.0));
    cb
}


/*------------------------------------------------------------------------------------------------------------------------------------------------*/
