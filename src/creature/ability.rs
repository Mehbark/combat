use std::{fmt::Debug, rc::Rc};

use crate::ui::{choose, Choosable, Describable};

use super::Creature;

pub trait Ability {
    fn name(&self) -> String;
    fn description(&self) -> String;

    fn activate_on_self(&self, you: &mut Creature);
    fn activate_on_enemy(&self, you: &mut Creature, enemy: &mut Creature);

    fn activate(&self, you: &mut Creature, enemy: &mut Creature) {
        loop {
            let target = choose::<Target>("Do you want to target yourself or the enemy?");

            match target {
                Target::You => {
                    if choose::<bool>("Are you sure you want to target yourself?") {
                        break self.activate_on_self(you);
                    }
                }
                Target::Enemy => break self.activate_on_enemy(you, enemy),
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
