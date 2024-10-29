use godot::prelude::*;
use godot::classes::{INode, Node};
use crate::network::entrypoint::enet_host::{ENetHostArgs, ENetHost};

pub(crate) trait NetworkConstructor<T>
where
    T: Inherits<RefCounted>
{
    fn construct(&mut self, args: Gd<T>);
}

pub(crate) trait NetworkConstructConsumer<T>
where
    T: Inherits<RefCounted>
{
    fn consume_args(&mut self, args: Gd<T>) -> bool;
}

pub(crate) trait CommonNetworkInterface
{
    fn is_initialized(&self) -> bool;
}

#[derive(GodotConvert, Var, Export)]
#[godot(via = i64)]
pub(crate) enum NetworkTransport
{
    None,
    ENetHost,
    ENetClient,
    //SteamHost
    //SteamClient
}

impl Default for NetworkTransport
{
    fn default() -> Self
    {
        NetworkTransport::None
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Node>,
    network_node: Option<Gd<Node>>,
    network_transport_type: NetworkTransport,
}

#[godot_api]
impl INode for NetworkEntryPoint
{
}

#[godot_api]
impl NetworkEntryPoint
{
    #[func]
    pub fn is_network_created(&self) -> bool
    {
        if self.network_node.is_none()
        {
            return false;
        }

        let network_node_ptr = self.network_node.as_ref().unwrap().clone();

        match self.network_transport_type
        {
            NetworkTransport::None => false,
            NetworkTransport::ENetHost => {
                network_node_ptr
                    .try_cast::<ENetHost>()
                    .is_ok_and(|node| node.bind().is_initialized())
            },
            NetworkTransport::ENetClient => {
                todo!()
            },
        }
    }

    #[func]
    pub fn create_enet_host(&mut self, args: Gd<ENetHostArgs>)
    {
        self.construct(args);
    }
}

impl NetworkConstructor<ENetHostArgs> for NetworkEntryPoint
{
    fn construct(&mut self, args: Gd<ENetHostArgs>)
    {
        let mut enet_host_node = ENetHost::new_alloc();
        self.base_mut().add_child(&enet_host_node);
        enet_host_node.set_name(GString::from("Network"));
        let mut construct_result= false;

        {
            construct_result = enet_host_node.bind_mut().consume_args(args);
        }

        if !construct_result
        {
            self.base_mut().remove_child(&enet_host_node);
            enet_host_node.queue_free();
            return;
        }

        self.network_node = Some(enet_host_node.upcast());
        self.network_transport_type = NetworkTransport::ENetHost;
    }
}