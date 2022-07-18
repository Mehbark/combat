use crate::ui::{Choosable, Describable};

use crate::creature::Ability;
mod stats;
use stats::abilities_from_classpect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Classpect {
    class: Class,
    aspect: Aspect,
}

impl Classpect {
    pub fn new(class: Class, aspect: Aspect) -> Self {
        Self { class, aspect }
    }

    pub fn abilities(self) -> Vec<Box<dyn Ability>> {
        abilities_from_classpect(self)
    }

    pub fn health(self) -> f64 {
        todo!()
    }

    pub fn defense(self) -> f64 {
        todo!()
    }

    pub fn speed(self) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Thief,
    Rogue,
    Bard,
    Prince,
    Heir,
    Page,
    Seer,
    Maid,
    Sylph,
    Knight,
    Witch,
    Mage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aspect {
    Space,
    Time,
    Mind,
    Heart,
    Hope,
    Rage,
    Breath,
    Blood,
    Life,
    Doom,
    Light,
    Void,
}

impl Choosable for Class {
    fn choices() -> Vec<Self> {
        vec![
            Self::Thief,
            Self::Rogue,
            Self::Bard,
            Self::Prince,
            Self::Heir,
            Self::Page,
            Self::Seer,
            Self::Maid,
            Self::Sylph,
            Self::Knight,
            Self::Witch,
            Self::Mage,
        ]
    }
}

impl Describable for Class {
    fn describe(&self) -> String {
        format!("{self:#?}")
    }
}

impl Choosable for Aspect {
    fn choices() -> Vec<Self> {
        vec![
            Self::Space,
            Self::Time,
            Self::Mind,
            Self::Heart,
            Self::Hope,
            Self::Rage,
            Self::Breath,
            Self::Blood,
            Self::Life,
            Self::Doom,
            Self::Light,
            Self::Void,
        ]
    }
}

impl Describable for Aspect {
    fn describe(&self) -> String {
        format!("{self:#?}")
    }
}
