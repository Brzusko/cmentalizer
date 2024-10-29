use godot::prelude::*;
use godot::classes::{INode, Node, ENetMultiplayerPeer};
use godot::global::Error;
use crate::network::entrypoint::network_entry_point::{NetworkConstructConsumer, CommonNetworkInterface};
use crate::network::entrypoint::utils::{is_any_network_initialized, get_multiplayer_api};

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub(crate) struct ENetHostArgs
{
    base: Base<RefCounted>,
    #[var]
    pub port: i32,
    #[var]
    pub max_clients: i32,
    #[var]
    pub channels: i32,
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct ENetHost
{
    base: Base<Node>,
    args: Option<Gd<ENetHostArgs>>,
    server_peer: Option<Gd<ENetMultiplayerPeer>>,
}

impl NetworkConstructConsumer<ENetHostArgs> for ENetHost
{
    fn consume_args(&mut self, _args: Gd<ENetHostArgs>) -> bool
    {
        let base_ptr = self.base().clone();
        let is_any_network_initialized = is_any_network_initialized(&base_ptr);

        if is_any_network_initialized
        {
            return false;
        }

        let mut multiplayer_api = get_multiplayer_api(&base_ptr);
        let mut host_peer = ENetMultiplayerPeer::new_gd();
        let server_create_result;
        {
            let bound_args = _args.bind();
            server_create_result = host_peer.create_server_ex(bound_args.port)
            .max_clients(bound_args.max_clients)
            .max_channels(bound_args.max_clients)
            .done();
        }

        if server_create_result != Error::OK
        {
            return false;
        }

        multiplayer_api.set_multiplayer_peer(&host_peer);
        self.server_peer = Some(host_peer);
        self.args = Some(_args);
        true
    }
}

impl CommonNetworkInterface for ENetHost
{
    fn is_initialized(&self) -> bool
    {
        true
    }
}
