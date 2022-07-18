mod ability;
mod status;

use std::fmt;
use std::rc::Rc;

pub use crate::classpect::{Aspect, Class, Classpect};

pub use self::ability::Ability;
pub use self::status::Status;

pub struct Creature {
    classpect: Classpect,

    maxhealth: f64,
    health: f64,
    defense: f64,
    speed: f64,

    abilities: Vec<Rc<dyn Ability>>,

    // Fungus is to fungi as status is to stati
    stati: Vec<Status>,
}

pub struct CreatureSkeleton {
    pub maxhealth: f64,
    pub health: f64,
    pub defense: f64,
    pub speed: f64,

    pub abilities: Vec<Rc<dyn Ability>>,
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

            stati: Vec::new(),
        }
    }

    pub fn deal_damage(&mut self, damage: f64) {
        let after_block = damage - self.defense;
        self.health -= after_block;
    }

    pub fn add_status(&mut self, status: Status) {
        self.stati.push(status);
    }

    pub fn activate_stati(&mut self) {
        let mut skeleton = self.extract_skeleton();

        for status in self.stati.iter_mut() {
            status.activate(&mut skeleton);
        }

        self.insert_skeleton(skeleton);
    }

    pub fn extract_skeleton(&self) -> CreatureSkeleton {
        CreatureSkeleton {
            maxhealth: self.maxhealth,
            health: self.health,
            defense: self.defense,
            speed: self.speed,
            abilities: self.abilities.clone(),
        }
    }

    fn insert_skeleton(&mut self, skeleton: CreatureSkeleton) {
        let CreatureSkeleton {
            maxhealth,
            health,
            defense,
            speed,
            abilities,
        } = skeleton;

        self.maxhealth = maxhealth;
        self.health = health;
        self.defense = defense;
        self.speed = speed;
        self.abilities = abilities;
    }

    pub fn classpect(&self) -> Classpect {
        self.classpect
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
