use crate::network::entrypoint::enet_listener::ENetListenerConstructData;
use crate::network::entrypoint::enet_participant::ENetParticipantConstructData;

pub(crate) enum NetworkConstructData
{
    ENetListener(ENetListenerConstructData),
    ENetParticipant(ENetParticipantConstructData)
}

struct A {

}

impl A
{
    pub fn test(&mut self)
    {
        // kod
    }
}

struct B
{
    
}