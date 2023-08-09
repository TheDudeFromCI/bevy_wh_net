mod events;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
pub use events::*;
pub use resources::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug)]
pub struct ClientNetworkingPlugin<T>
where T: Serialize + DeserializeOwned + Send + Sync + 'static {
    _phantom: PhantomData<T>,
}

impl<T> Default for ClientNetworkingPlugin<T>
where T: Serialize + DeserializeOwned + Send + Sync + 'static
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> Plugin for ClientNetworkingPlugin<T>
where T: Serialize + DeserializeOwned + Send + Sync + 'static
{
    fn build(&self, app: &mut App) {
        app.add_state::<NetworkState>()
            .add_event::<TryConnectToServerEvent>()
            .add_event::<JoinedServerEvent>()
            .add_event::<DisconnectedFromServerEvent>()
            .add_event::<CouldNotConnectToServerEvent>()
            .add_event::<SendPacket<T>>()
            .add_event::<ReceivePacket<T>>()
            .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
            .add_systems(
                Update,
                (
                    systems::connect_to_server.run_if(in_state(NetworkState::NotConnected)),
                    systems::wait_for_connection.run_if(in_state(NetworkState::Connecting)),
                    systems::close_connection.run_if(not(in_state(NetworkState::NotConnected))),
                    systems::send_packet::<T>.run_if(in_state(NetworkState::Connected)),
                    systems::receive_packets::<T>.run_if(in_state(NetworkState::Connected)),
                    systems::error_handling,
                ),
            );
    }
}
