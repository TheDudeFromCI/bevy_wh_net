use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NetworkState {
    #[default]
    NotConnected,
    Connecting,
    Connected,
    Joined,
}

pub fn condition_is_connected(state: Res<State<NetworkState>>) -> bool {
    *state.get() == NetworkState::Connected || *state.get() == NetworkState::Joined
}
