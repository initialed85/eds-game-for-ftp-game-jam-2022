use wasm_bindgen::prelude::wasm_bindgen;

use eds_game_for_ftp_game_jam_2022::client::app::get_app_for_client;

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = get_app_for_client();

    app.run();
}

pub fn main() {
    // wasm_bindgen invokes run() for us
    // run();
}
