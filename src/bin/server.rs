use eds_game_for_ftp_game_jam_2022::server::app::get_app_for_server;

fn run() {
    let mut app = get_app_for_server();

    app.run();
}

pub fn main() {
    run();
}
