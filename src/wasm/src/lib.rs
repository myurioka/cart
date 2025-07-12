#[macro_use]
mod browser;
mod engine;
mod game;
mod sound;

use engine::GameLoop;
use game::GameStage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let game = GameStage::new();

        GameLoop::start(game)
            .await
            .expect("Cloud not start game loop");
    });

    Ok(())
}
