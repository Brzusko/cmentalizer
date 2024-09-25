use std::cell::RefCell;
use std::rc::{Rc, Weak};
use godot::prelude::*;
use godot::classes::{INode, Node};
use crate::network::entrypoint::enet_listener::ENetListener;
use crate::network::entrypoint::enet_participant::ENetParticipant;
// Creation flow
// Struct A Wants to create NetworkPoint from NetworkEntryPoint
// Gives somehow reference to method via smart pointer
// When NetworkPoint is created, reference

pub(crate) enum WhichTransport
{
    // MaxClients
    ENetListener(i32),
    // Server socket address - ipv4/ipv6
    ENetParticipant(GString),
}

pub(crate) enum NetworkMode
{
    ENetListener(Gd<ENetListener>),
    ENetParticipant(Gd<ENetParticipant>),
}

pub(crate) struct CommonTransportOptions
{
    port: i32,
    channels_count: i32,
    transport: WhichTransport,
}

impl CommonTransportOptions
{
    fn new(port: i32, channels_count: i32, transport: WhichTransport) -> Self
    {
        Self
        {
            port,
            channels_count,
            transport
        }
    }
}

pub(crate) trait NetworkConstructListener
{
    fn construct_result(&mut self, result: anyhow::Result<Weak<RefCell<NetworkMode>>>);
}

pub(crate) trait NetworkListener {}
pub(crate) trait NetworkParticipant {}

pub(crate) trait NetworkConstruct
{
    fn construct(&mut self, network_entry_point: Gd<NetworkEntryPoint>);
}

// Refactor as Engine Singleton later
#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Node>,
    network_mode_ptr: Option<Rc<NetworkMode>>,
    construct_listener_ptr: Option<Rc<RefCell<dyn NetworkConstructListener>>>,
}

impl NetworkEntryPoint
{
    pub(crate) fn construct(&mut self, listener: Rc<RefCell<dyn NetworkConstructListener>>)
    {

    }

    pub(crate) fn construct_result(&mut self) {

    }
}