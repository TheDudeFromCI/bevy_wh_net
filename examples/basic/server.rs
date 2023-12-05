use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_wh_net::server::ServerNetworkingPlugin;

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
        .run()
}
