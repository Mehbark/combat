use std::fmt::Debug;

use super::Creature;

pub trait Ability: Debug {
    fn name(&self) -> String;
    fn description(&self) -> String;

    fn activate_on_self(&self, target: &mut Creature);
    fn activate_on_enemy(&self, target: &mut Creature);
}
