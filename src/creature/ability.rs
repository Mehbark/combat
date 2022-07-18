use std::{fmt::Debug, rc::Rc};

use crate::ui::{choose, Choosable, Describable};

use super::Creature;

pub trait Ability {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn has_target(&self) -> bool;

    fn activate_on_self(&self, you: &mut Creature);
    fn activate_on_enemy(&self, you: &mut Creature, enemy: &mut Creature);

    fn activate(&self, you: &mut Creature, enemy: &mut Creature) {
        loop {
            let target = if self.has_target() {
                choose::<Target>("Do you want to target yourself or the enemy?")
            } else {
                Target::None
            };

            match target {
                Target::You => {
                    if choose::<bool>("Are you sure you want to target yourself?") {
                        break self.activate_on_self(you);
                    }
                }
                Target::Enemy => break self.activate_on_enemy(you, enemy),
                Target::None => break,
            }
        }
    }
}

impl<A: Ability> Describable for A {
    fn describe(&self) -> String {
        format!("{}: {}", self.name(), self.description())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    You,
    Enemy,
    None,
}

impl Choosable for Target {
    type Item = Self;
    fn choices() -> Vec<Self::Item> {
        vec![Self::You, Self::Enemy]
    }
}

impl Describable for Target {
    fn describe(&self) -> String {
        match self {
            Target::You => "Target yourself".to_string(),
            Target::Enemy => "Target enemy".to_string(),
            Target::None => {
                "you shouldn't see this lol! all of this is homestuck be tee dubs".to_string()
            }
        }
    }
}

impl Ability for Rc<dyn Ability> {
    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn description(&self) -> String {
        self.as_ref().description()
    }

    fn has_target(&self) -> bool {
        self.as_ref().has_target()
    }

    fn activate_on_self(&self, you: &mut Creature) {
        self.as_ref().activate_on_self(you)
    }

    fn activate_on_enemy(&self, you: &mut Creature, enemy: &mut Creature) {
        self.as_ref().activate_on_enemy(you, enemy)
    }
}

impl Choosable for Vec<Rc<dyn Ability>> {
    type Item = Rc<dyn Ability>;
}
