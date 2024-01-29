use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_wh_net::server::{OnClientConnected, ServerNetworkingPlugin};

pub fn run() {
    let networking = ServerNetworkingPlugin {
        ip: "127.0.0.1:8123".into(),
        max_clients: 64,
    };

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            ..default()
        })
        .add_plugins(networking)
        .add_systems(Update, on_connected)
        .run()
}

fn on_connected(mut on_client_connected_evs: EventReader<OnClientConnected>) {
    for ev in on_client_connected_evs.read() {
        info!(
            "Client {:?} connected with id {}",
            ev.login_data, ev.client_id
        );
    }
}
