use godot::prelude::*;
use godot::classes::{INode, Node, ENetMultiplayerPeer, RefCounted};
use godot::global::Error;
use crate::network::entrypoint::network_entry_point::{CommonNetworkInterface, NetworkConstructConsumer};
use crate::network::entrypoint::utils::{get_multiplayer_api, is_any_network_initialized};

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub(crate) struct ENetClientArgs
{
    base: Base<RefCounted>,
    #[var]
    pub address: GString,
    #[var]
    pub port: i32,
    #[var]
    pub max_channels: i32,
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct ENetClient
{
    base: Base<Node>,
    args: Option<Gd<ENetClientArgs>>,
    client_peer: Option<Gd<ENetMultiplayerPeer>>,
    is_connected: bool,
}

#[godot_api]
impl ENetClient
{
    #[func]
    pub fn is_connected(&self) -> bool
    {
        return self.is_initialized();
    }

    #[func]
    pub fn connected(&mut self)
    {
        self.is_connected = true;
    }

    #[func]
    pub fn disconnected(&mut self)
    {
        self.is_connected = false;
    }
}

impl NetworkConstructConsumer<ENetClientArgs> for ENetClient
{
    fn consume_args(&mut self, args: Gd<ENetClientArgs>) -> bool
    {
        let base_ptr = self.base().clone();

        if is_any_network_initialized(&base_ptr)
        {
            return false;
        }

        let mut multiplayer_api = get_multiplayer_api(&base_ptr);
        let connected_callable = self.base().callable("connected");
        let disconnected_callable = self.base().callable("disconnected");
        let connection_failed_callable = self.base().callable("disconnected");

        multiplayer_api.connect(StringName::from("connected_to_server"), connected_callable);
        multiplayer_api.connect(StringName::from("server_disconnected"), disconnected_callable);
        multiplayer_api.connect(StringName::from("connection_failed"), connection_failed_callable);

        let mut enet_peer = ENetMultiplayerPeer::new_gd();
        let connect_result;
        {
            let bound_args = args.bind();
            connect_result = enet_peer
                .create_client_ex(bound_args.address.clone(), bound_args.port)
                .channel_count(bound_args.max_channels)
                .done();
        }

        if connect_result != Error::OK
        {
            return false;
        }

        self.args = Some(args);
        self.client_peer = Some(enet_peer);
        return true;
    }
}

impl CommonNetworkInterface for ENetClient
{
    fn is_initialized(&self) -> bool
    {
        return self.is_connected;
    }
}