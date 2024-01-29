use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_wh_net::server::{
    DoKickPlayer,
    DoValidateClient,
    OnClientConnected,
    ServerNetworkingPlugin,
};

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
        .add_systems(Update, validate_user)
        .run()
}

fn validate_user(
    mut on_client_connected_evs: EventReader<OnClientConnected>,
    mut do_validate_client_evs: EventWriter<DoValidateClient>,
    mut do_kick_player_evs: EventWriter<DoKickPlayer>,
) {
    for ev in on_client_connected_evs.read() {
        let Some(login_data) = &ev.login_data else {
            do_kick_player_evs.send(DoKickPlayer {
                client_id: ev.client_id,
                reason: "You must login to play".into(),
            });
            continue;
        };

        let username = login_data.get_username();
        let salt = format!("random salt {}", username);
        let password = login_data.get_salted_password(&salt);

        if authenticate(username, &password) {
            do_validate_client_evs.send(DoValidateClient {
                client_id: ev.client_id,
            });
        } else {
            do_kick_player_evs.send(DoKickPlayer {
                client_id: ev.client_id,
                reason: "Invalid username or password".into(),
            });
        }
    }
}

fn authenticate(_username: &str, _password: &str) -> bool {
    // Check your local database or something

    true
}
