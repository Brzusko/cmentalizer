use godot::prelude::Gd;
use godot::classes::{MultiplayerApi, Node, OfflineMultiplayerPeer};
use godot::global::godot_print;

pub fn is_any_network_initialized(caller: &Gd<Node>) -> bool
{
    assert_eq!(caller.is_instance_valid(), true);

    let main_tree = caller.get_tree();
    assert_eq!(main_tree.is_none(), false);

    let multiplayer_api = main_tree.unwrap().get_multiplayer();
    assert_eq!(multiplayer_api.is_none(), false);

    let multiplayer_api_peer = multiplayer_api.unwrap().get_multiplayer_peer();

    if multiplayer_api_peer.is_none()
    {
        return false;
    }

    multiplayer_api_peer.unwrap().try_cast::<OfflineMultiplayerPeer>().is_err()
}

pub fn get_multiplayer_api(caller: &Gd<Node>) -> Gd<MultiplayerApi>
{
    assert_eq!(caller.is_instance_valid(), true);

    let main_tree = caller.get_tree();
    assert_eq!(main_tree.is_none(), false);

    let multiplayer_api = main_tree.unwrap().get_multiplayer();
    assert_eq!(multiplayer_api.is_none(), false);

    multiplayer_api.unwrap()
}