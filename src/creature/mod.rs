mod ability;

use std::fmt;
use std::rc::Rc;

pub use crate::classpect::{Aspect, Class, Classpect};

pub use self::ability::Ability;

pub struct Creature {
    classpect: Classpect,

    maxhealth: f64,
    health: f64,
    defense: f64,
    speed: f64,

    abilities: Vec<Rc<dyn Ability>>,
}

impl Creature {
    pub fn new_with_classpect(class: Class, aspect: Aspect) -> Self {
        let classpect = Classpect::new(class, aspect);

        Self {
            classpect,

            maxhealth: classpect.health(),
            health: classpect.health(),
            defense: classpect.defense(),
            speed: classpect.speed(),

            abilities: classpect.abilities(),
        }
    }

    pub fn classpect(&self) -> Classpect {
        self.classpect
    }

    pub fn deal_damage(&mut self, damage: f64) {
        let after_block = damage - self.defense;
        self.health -= after_block;
    }

    pub fn maxhealth(&self) -> f64 {
        self.maxhealth
    }

    pub fn health(&self) -> f64 {
        self.health
    }

    pub fn defense(&self) -> f64 {
        self.defense
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn abilities(&self) -> &[Rc<dyn Ability>] {
        self.abilities.as_ref()
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.classpect)?;

        writeln!(f, "Health:  {:.1}/{:.1}", self.health, self.maxhealth)?;
        writeln!(f, "Defense: {:.1}", self.defense)?;
        writeln!(f, "Speed:   {:.1}", self.speed)?;

        writeln!(f, "Abilities:")?;
        for ability in self.abilities.iter() {
            writeln!(f, "{}: {}", ability.name(), ability.description())?;
        }

        Ok(())
    }
}
