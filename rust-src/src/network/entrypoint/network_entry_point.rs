use std::ptr;
use std::rc::{Rc, Weak};
use anyhow::anyhow;
use godot::prelude::*;
use godot::classes::{ENetMultiplayerPeer, INode, Node};
use crate::network::entrypoint::enet_listener::ENetListener;
use crate::network::entrypoint::enet_participant::ENetParticipant;
use crate::network::entrypoint::network_entry_point::Transport::ENetLocal;
// Creation flow
// Struct A Wants to create NetworkPoint from NetworkEntryPoint
// Gives somehow reference to method via smart pointer
// When NetworkPoint is created, reference

pub(crate) trait NetworkConstructListener
{
    fn construct_result(&self, result: anyhow::Result<Weak<NetworkMode>>);
}

pub(crate) trait Listener {}
pub(crate) trait Participant {}
pub(crate) trait Constructor 
{
    fn construct(&mut self, entry_point: Gd<NetworkEntryPoint>);   
}

pub(crate) enum NetworkMode
{
    ENetLocal(Gd<ENetListener>),
    ENetRemote(Gd<ENetParticipant>),
}

pub(crate) enum Transport
{
    // MaxClients -> i32    
    ENetLocal,
    // Address -> GString
    ENetRemote,
}

pub(crate) struct BasicTransportOptions
{
    port: i32,
    channels: i32,
}

pub(crate) enum ConnectionResponse
{
    Success,
    Error,
}

// Refactor as Engine Singleton later
#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Node>,
    pending_construct_listener: Option<Weak<dyn NetworkConstructListener>>,
    network_mode: Option<Rc<NetworkMode>>,
}

#[godot_api]
impl INode for NetworkEntryPoint
{
}

impl NetworkEntryPoint
{
    pub(crate) fn construct_entry_point(&mut self, listener_ptr: Weak<dyn NetworkConstructListener>, transport: Transport, basic_transport_options: BasicTransportOptions)
    {
        let ptr = listener_ptr.upgrade();
        if ptr.is_none() { return; }
        
        let ptr = ptr.unwrap();
        
        if self.pending_construct_listener.is_some() || self.network_mode.is_some()
        {
            ptr.construct_result(Err(anyhow!("Pending net creation or is created")));
            return;
        }
        
        self.pending_construct_listener = Some(listener_ptr);
        
        let network_mode = match transport {
            Transport::ENetRemote => {
                let instance = ENetParticipant::new_alloc();
                let base = self.base_mut().add_child(&instance);
                NetworkMode::ENetRemote(instance)
            }
            Transport::ENetLocal => {
                let instance = ENetListener::new_alloc();
                let base = self.base_mut().add_child(&instance);
                NetworkMode::ENetLocal(instance)
            }
        };
        
        self.network_mode = Some(Rc::new(network_mode));
        let network_mode = Rc::clone(self.network_mode.as_ref().unwrap());
        let ptr = self.base().clone().cast::<NetworkEntryPoint>();
        
        match &*network_mode {
            NetworkMode::ENetLocal(mut mode) => { mode.bind_mut().construct(ptr); }
            NetworkMode::ENetRemote(mut mode) => {}
        }
    }
    
    pub(crate) fn construct_result(&mut self, connection_response: ConnectionResponse)
    {
        match connection_response 
        {
            ConnectionResponse::Success => {
                
            }
            
            ConnectionResponse::Error => {
                
            }
        }
    }
}