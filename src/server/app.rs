use bevy::log::trace;
use bevy::prelude::{App, IntoSystemDescriptor};

use crate::base::app::get_base_app;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::server::despawn::handle_despawn_event;
use crate::server::join::handle_join_event;
use crate::server::leave::handle_leave_event;
use crate::server::network::{handle_close_event, handle_open_event, handle_websocket_server};
use crate::server::spawn::handle_spawn_event;
use crate::server::websocket::get_websocket_server;

pub fn get_app_for_server() -> App {
    let mut app = get_base_app();

    let web_socket_server = get_websocket_server();

    trace!(
        "client.get_app(); created web_socket_server={:?}",
        web_socket_server
    );

    app.insert_non_send_resource(web_socket_server);

    app.add_system(
        handle_websocket_server
            .before(base_handle_close_event)
            .before(base_handle_incoming_message_event)
            .before(base_handle_open_event),
    )
    .add_system(handle_open_event.after(handle_websocket_server))
    .add_system(handle_join_event.after(handle_open_event))
    .add_system(handle_spawn_event.after(handle_join_event))
    .add_system(handle_close_event.after(handle_spawn_event))
    .add_system(handle_leave_event.after(handle_close_event))
    .add_system(handle_despawn_event.after(handle_leave_event));

    trace!("client.get_app(); returning app={:?}", app);

    return app;
}
