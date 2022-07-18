use std::rc::Rc;

use crate::creature::{Ability, Creature};

use super::Classpect;

// 4 abilities to start
// 1: Basic attack, no downsides, just uses stats
// 2: Class specialty ability  C(target)
// 3: Aspect specialty ability A(target)
// 4: Composition of specials  (C . A)(target)
pub fn abilities_from_classpect(classpect: Classpect) -> Vec<Rc<dyn Ability>> {
    let aggrieve = ClasspectAbility {
        name: "Aggrieve",
        description: "Deal damage based on stats (woo)",

        activate_on_self: |you| {
            you.deal_damage(you.health() / 100.0 + you.defense() / 2.0 + you.speed() / 2.0)
        },
        activate_on_enemy: |you, target| {
            target.deal_damage(you.health() / 100.0 + you.defense() / 2.0 + you.speed() / 2.0)
        },
    };

    vec![Rc::new(aggrieve)]
}

// TODO: confirm on activate on self lol

struct ClasspectAbility {
    pub name: &'static str,
    pub description: &'static str,

    pub activate_on_self: fn(&mut Creature),
    pub activate_on_enemy: fn(&mut Creature, &mut Creature),
}

impl Ability for ClasspectAbility {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn description(&self) -> String {
        self.description.to_string()
    }

    fn activate_on_self(&self, you: &mut Creature) {
        (self.activate_on_self)(you)
    }

    fn activate_on_enemy(&self, you: &mut Creature, target: &mut Creature) {
        (self.activate_on_enemy)(you, target)
    }
}
