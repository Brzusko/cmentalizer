use godot::prelude::*;
use godot::classes::{ENetMultiplayerPeer, INode, Node};
pub(crate) static NETWORK_ENTRY_POINT_PATH: &str = "/root/GlobalNetworkEntryPoint";
pub(crate) enum TransportConnectionDetails
{
    ENetHost(i32),
    EnetClient(GString, i32),
    SteamP2PHost(i32),
    SteamP2PClient(i32),
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Node>,
}

#[godot_api]
impl INode for NetworkEntryPoint
{
    fn ready(&mut self) {
    }
}

#[godot_api]
impl NetworkEntryPoint
{
}