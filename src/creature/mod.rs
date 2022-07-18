mod ability;

pub use crate::classpect::{Aspect, Class, Classpect};

pub use self::ability::Ability;

#[derive(Debug)]
pub struct Creature {
    classpect: Classpect,

    abilities: Vec<Box<dyn Ability>>,

    health: f64,
    defense: f64,
    speed: f64,
}

impl Creature {
    pub fn new_with_classpect(class: Class, aspect: Aspect) -> Self {
        let classpect = Classpect::new(class, aspect);

        Self {
            classpect,

            abilities: classpect.abilities(),

            health: classpect.health(),
            defense: classpect.defense(),
            speed: classpect.speed(),
        }
    }

    pub fn classpect(&self) -> Classpect {
        self.classpect
    }
}
