use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NetworkState {
    #[default]
    NotConnected,
    Connecting,
    Connected,
}
