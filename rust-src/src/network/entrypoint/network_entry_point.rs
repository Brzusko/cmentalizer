use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
use anyhow::{anyhow};
use thiserror::Error;
use godot::prelude::*;
use godot::classes::{INode, Node};

use crate::network::entrypoint::enet_listener::{ENetListener, ENetListenerConstructData};
use crate::network::entrypoint::enet_participant::ENetParticipant;
use crate::network::entrypoint::network_construct_data::NetworkConstructData;

// Creation flow
// Struct A Wants to create NetworkPoint from NetworkEntryPoint
// Gives somehow reference to method via smart pointer
// When NetworkPoint is created, reference

pub(crate) enum WhichTransport
{
    ENetListener,
    ENetParticipant,
}

pub(crate) enum NetworkMode
{
    ENetListener(Gd<ENetListener>),
    ENetParticipant(Gd<ENetParticipant>),
}

#[derive(Error, Debug)]
pub(crate) enum NetworkConstructError
{
    #[error("TODO")]
    Todo,
}

pub(crate) trait NetworkConstructListener
{
    fn construct_result(&mut self, result: anyhow::Result<Weak<RefCell<NetworkMode>>>);
}

pub(crate) trait NetworkListener {}
pub(crate) trait NetworkParticipant {}

pub(crate) trait NetworkConstruct
{
    fn construct(&mut self, network_entry_point: Gd<NetworkEntryPoint>, meta_data: NetworkConstructData);
}

#[derive(GodotClass)]
#[class(base = Object, init)]
struct TestStruct {
    base: Base<Object>,
}

impl TestStruct {
    fn result(&self, result: anyhow::Result<Weak<RefCell<NetworkMode>>>)
    {
        match result {
            Ok(_) => { godot_print!("Created") }
            Err(_) => { godot_print!("Not created") }
        }
    }

    fn to_listener(&self) -> Box<RefCell<TestStructListener>>
    {
        let new_ptr = self.base().clone().cast::<TestStruct>();
        Box::new(RefCell::new(TestStructListener::new(new_ptr)))
    }
}

struct TestStructListener
{
    base: Gd<TestStruct>,
}

impl TestStructListener
{
    fn new(base: Gd<TestStruct>) -> Self {
        Self {
            base
        }
    }
}

impl NetworkConstructListener for TestStructListener {
    fn construct_result(&mut self, result: anyhow::Result<Weak<RefCell<NetworkMode>>>) {

    }
}

// Refactor as Engine Singleton later
#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Node>,
    network_mode_ptr: Option<Rc<RefCell<NetworkMode>>>,
    construct_listener_ptr: Option<Box<RefCell<dyn NetworkConstructListener>>>,
    test_ref: Option<Gd<TestStruct>>,
}

#[godot_api]
impl INode for NetworkEntryPoint {
    fn ready(&mut self) {
        let test_struct = TestStruct::new_alloc();
        let test_struct_listener = test_struct.bind().to_listener();

        self.test_ref = Some(test_struct);

        self.construct(test_struct_listener, WhichTransport::ENetListener, NetworkConstructData::ENetListener(ENetListenerConstructData::default()));
        self.trigger();
    }
}

impl NetworkEntryPoint
{
    pub(crate) fn trigger(&mut self) {
        match self.network_mode_ptr.as_ref().unwrap().borrow_mut().deref_mut() {
            NetworkMode::ENetListener(list) => { list.bind_mut().test() }
            NetworkMode::ENetParticipant(_) => {}
        }
    }
    pub(crate) fn construct(&mut self, listener: Box<RefCell<dyn NetworkConstructListener>>, transport: WhichTransport , options: NetworkConstructData)
    {
        {
            let mut listener_mut = listener.borrow_mut();

            if self.network_mode_ptr.is_some() || self.construct_listener_ptr.is_some()
            {
                listener_mut.construct_result(Err(anyhow!(NetworkConstructError::Todo)));
                return;
            }
        }

        self.construct_listener_ptr = Some(listener);

        {
            let ptr_self;
            let mut socket;
            {
                ptr_self = self.base().clone().cast::<NetworkEntryPoint>();
                socket = ENetListener::new_alloc();
                self.base_mut().add_child(&socket);
            }

            socket.bind_mut().construct(ptr_self, options);

            self.network_mode_ptr = Some(Rc::new(RefCell::new(NetworkMode::ENetListener(socket))));
        }

        // self.network_mode_ptr = match transport {
        //     WhichTransport::ENetListener => {
        //         let ptr_self = self.base_mut().clone().cast::<NetworkEntryPoint>();
        //         let mut socket = ENetListener::new_alloc();
        //         self.base_mut().add_child(&socket);
        //         socket.bind_mut().construct(ptr_self, options);
        //
        //         Some(Rc::new(RefCell::new(NetworkMode::ENetListener(socket))))
        //     }
        //     WhichTransport::ENetParticipant => {
        //         let ptr_self = self.base().clone().cast::<NetworkEntryPoint>();
        //         let mut socket = ENetParticipant::new_alloc();
        //         self.base_mut().add_child(&socket);
        //         socket.bind_mut().construct(ptr_self, options);
        //
        //         Some(Rc::new(RefCell::new(NetworkMode::ENetParticipant(socket))))
        //     }
        // };
    }

    pub(crate) fn construct_result(&mut self, is_success: bool) {
        let listener = self.construct_listener_ptr.take().unwrap();
        let network_mode_ptr = self.network_mode_ptr.take().unwrap();

        if !is_success {
            listener.borrow_mut().construct_result(Err(anyhow!(NetworkConstructError::Todo)));

            match Rc::try_unwrap(network_mode_ptr) {
                Ok(network) => {
                    match network.into_inner() {
                        NetworkMode::ENetListener(mut socket) => { socket.queue_free() }
                        NetworkMode::ENetParticipant(mut socket) => { socket.queue_free() }
                    };
                }
                Err(_) => {}
            }

            return;
        }

        let network_mode_weak_ptr = Rc::downgrade(&network_mode_ptr);
        listener.borrow_mut().construct_result(Ok(network_mode_weak_ptr));
        self.network_mode_ptr = Some(network_mode_ptr);
    }
}