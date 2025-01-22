use std::rc::{Rc, Weak};

use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, Copy, Eq, PartialEq, Debug)]
#[godot(via = i64)]
pub(crate) enum GameStatIDs {
    None,
    MaxSpeed,
    MinSpeed,
    Acceleration,
    Deceleration,
}

impl Hash for GameStatIDs {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i64(self.to_godot());
    }
}

impl Default for GameStatIDs {
    fn default() -> Self {
        GameStatIDs::None
    }
}

#[derive(GodotClass)]
#[class(base = Resource, init)]
pub(crate) struct GameStatInitData {
    #[export]
    stat_id: GameStatIDs,
    #[export]
    initial_value: f32,
    #[export]
    max_value: f32,
}

impl GameStatInitData {
    pub fn construct_game_stat(&self) -> (GameStatIDs, GameStat) {
        let game_stat = GameStat {
            init_value: self.initial_value,
            current_value: self.initial_value,
            max_value: self.max_value,
        };

        return (self.stat_id.clone(), game_stat);
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
struct GameStatSet {
    base: Base<Node>,
    stats_init_data: Array<Gd<GameStatInitData>>,
    stats_map: HashMap<GameStatIDs, Rc<RefCell<GameStat>>>,
}

#[godot_api]
impl INode for GameStatSet {
    fn ready(&mut self) {
        for init_data in self.stats_init_data.iter_shared() {
            let game_stat_pair = init_data.bind().construct_game_stat();

            let entry = self.stats_map.entry(game_stat_pair.0);

            if let Entry::Vacant(vacant) = entry {
                vacant.insert(Rc::new(RefCell::new(game_stat_pair.1)));
            }
        }
    }
}

#[godot_api]
impl GameStatSet {
    #[signal]
    fn stat_changed(stat_id: GameStatIDs) {}

    pub(crate) fn get_stat_weak_ptr(
        &mut self,
        stat_id: GameStatIDs,
    ) -> Option<Weak<RefCell<GameStat>>> {
        match self.stats_map.entry(stat_id) {
            Entry::Occupied(occupied_entry) => Some(Rc::downgrade(occupied_entry.get())),
            Entry::Vacant(_) => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct GameStat {
    init_value: f32,
    current_value: f32,
    max_value: f32,
}

pub(crate) struct ConstModifiersHolder {}
impl Default for ConstModifiersHolder {
    fn default() -> Self {
        todo!()
    }
}

pub(crate) struct TimedModifiersHolder {}
impl Default for TimedModifiersHolder {
    fn default() -> Self {
        todo!()
    }
}

pub(crate) struct PerTickModifiersHolder {}
impl Default for PerTickModifiersHolder {
    fn default() -> Self {
        todo!()
    }
}

#[derive(GodotClass)]
#[class(base = Resource, init)]
pub(crate) struct SimpleModifierInitData {
    #[export]
    modifier_value: f32,
    #[export]
    modifier_operation: ModifierOperation,
}

impl SimpleModifierInitData {
    pub(crate) fn construct_modifier(&self) -> SimpleModifier {
        SimpleModifier {
            modifier_value: self.modifier_value,
            modifier_operation: self.modifier_operation,
        }
    }
}

#[derive(GodotClass)]
#[class(base = Resource, init)]
pub(crate) struct SimpleTimedModifierInitData {
    #[export]
    modifier_value: f32,
    #[export]
    modifier_operation: ModifierOperation,
    #[export]
    time_left: f32,
}

impl SimpleTimedModifierInitData {
    pub(crate) fn construct_modifier(&self) -> SimpleTimedModifier {
        SimpleTimedModifier {
            modifier_value: self.modifier_value,
            modifier_operation: self.modifier_operation,
            time_left: self.time_left,
        }
    }
}

#[derive(GodotClass)]
#[class(base = Resource, init)]
pub(crate) struct PerTickModifierInitData {
    #[export]
    modifier_value: f32,
    #[export]
    modifier_operation: ModifierOperation,
    #[export]
    time_left: f32,
    #[export]
    time_to_trigger: f32,
}

impl PerTickModifierInitData {
    pub(crate) fn construct_modifier(&self) -> PerTickModifier {
        PerTickModifier {
            modifier_value: self.modifier_value,
            modifier_operation: self.modifier_operation,
            time_left: self.time_left,
            time_to_trigger: self.time_to_trigger,
            time_accumulator: 0.0,
        }
    }
}

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy)]
#[godot(via = i64)]
pub(crate) enum ModifierOperation {
    Add,
    Substract,
    Multiply,
}

impl Default for ModifierOperation {
    fn default() -> Self {
        Self::Add
    }
}

#[derive(Debug, Default)]
pub(crate) struct SimpleModifier {
    pub modifier_value: f32,
    pub modifier_operation: ModifierOperation,
}

#[derive(Debug, Default)]
pub(crate) struct SimpleTimedModifier {
    pub modifier_value: f32,
    pub modifier_operation: ModifierOperation,
    pub time_left: f32,
}

#[derive(Debug, Default)]
pub(crate) struct PerTickModifier {
    pub modifier_value: f32,
    pub modifier_operation: ModifierOperation,
    pub time_left: f32,
    pub time_to_trigger: f32,
    pub time_accumulator: f32,
}
