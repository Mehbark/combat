use std::rc::Rc;

use crate::creature::{Ability, Creature, Status};

use super::Classpect;

// 4 abilities to start
// 1: Basic attack, no downsides, just uses stats
// 2: Class specialty ability  C(target)
// 3: Aspect specialty ability A(target)
// 4: Composition of specials  (C . A)(target)
// + A basic wait
pub fn abilities_from_classpect(classpect: Classpect) -> Vec<Rc<dyn Ability>> {
    let aggrieve = ClasspectAbility {
        name: "Aggrieve",
        description: "Deal damage based on stats (woo)",
        targetable: true,

        activate_on_self: |you| {
            you.deal_damage(you.health() / 100.0 + you.defense() / 2.0 + you.speed() / 2.0)
        },
        activate_on_enemy: |you, target| {
            target.deal_damage(you.health() / 100.0 + you.defense() / 2.0 + you.speed() / 2.0)
        },
    };

    let acromonify = ClasspectAbility {
        name: "Acromonify",
        description:
            "Apply poison for (your speed - target speed) turns, with a minimum of one turn",
        targetable: true,

        activate_on_self: |you| {
            let your_speed = you.speed();

            you.add_status(Status::new_exitless(
                1,
                Rc::new(move |target| target.health -= your_speed / 3.0),
            ))
        },
        activate_on_enemy: |you, target| {
            let your_speed = you.speed();
            let target_speed = target.speed();
            let speed_differential = (your_speed - target_speed).max(2.0) / 2.0;

            target.add_status(Status::new_exitless(
                speed_differential as usize,
                Rc::new(move |target| {
                    target.health -= (speed_differential - target.defense).max(1.0)
                }),
            ))
        },
    };

    let r#await = ClasspectAbility {
        name: "Await",
        description: "Wait out the turn",
        targetable: false,

        activate_on_self: |_| (),
        activate_on_enemy: |_, _| (),
    };

    vec![Rc::new(aggrieve), Rc::new(acromonify), Rc::new(r#await)]
}

// TODO: confirm on activate on self lol

struct ClasspectAbility {
    pub name: &'static str,
    pub description: &'static str,
    pub targetable: bool,

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

    fn has_target(&self) -> bool {
        self.targetable
    }

    fn activate_on_self(&self, you: &mut Creature) {
        (self.activate_on_self)(you)
    }

    fn activate_on_enemy(&self, you: &mut Creature, target: &mut Creature) {
        (self.activate_on_enemy)(you, target)
    }
}
