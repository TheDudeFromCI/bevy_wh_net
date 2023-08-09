use bevy::prelude::*;
use bevy_wh_net::client::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum PingPongPacket {
    Ping,
    Pong,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ClientNetworkingPlugin::<PingPongPacket>::default())
        .add_systems(
            Update,
            (
                connect,
                try_join_server,
                joined_server,
                disconnected_from_server,
                failed_to_connect,
                ping,
                pong,
            ),
        )
        .run();
}

fn connect(
    time: Res<Time>,
    mut has_run: Local<bool>,
    mut conn_event: EventWriter<TryConnectToServerEvent>,
) {
    if *has_run {
        return;
    }

    if time.elapsed_seconds() < 2.0 {
        return;
    }

    *has_run = true;
    conn_event.send(TryConnectToServerEvent {
        ip: String::from("127.0.0.1:8123"),
    });
}

fn try_join_server(mut events: EventReader<TryConnectToServerEvent>) {
    for ev in events.iter() {
        println!("Trying to connect to server at: {}", ev.ip);
    }
}

fn joined_server(mut events: EventReader<JoinedServerEvent>) {
    for _ in events.iter() {
        println!("Joined server.");
    }
}

fn disconnected_from_server(mut events: EventReader<DisconnectedFromServerEvent>) {
    for _ in events.iter() {
        println!("Disconnected from server.");
    }
}

fn failed_to_connect(mut events: EventReader<CouldNotConnectToServerEvent>) {
    for _ in events.iter() {
        println!("Failed to connect to server.");
    }
}

fn ping(
    time: Res<Time>,
    mut last_ping: Local<f32>,
    mut packet_event: EventWriter<SendPacket<PingPongPacket>>,
) {
    let seconds = time.elapsed_seconds();
    if seconds - *last_ping < 1.0 {
        return;
    }

    *last_ping = seconds;
    packet_event.send(SendPacket {
        packet: PingPongPacket::Ping,
    });

    println!("Ping!");
}

fn pong(mut events: EventReader<ReceivePacket<PingPongPacket>>) {
    for _ in events.iter() {
        println!("Pong!");
    }
}
