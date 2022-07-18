use std::fmt;
use std::rc::Rc;

use crate::ui::{Choosable, Describable};

use crate::creature::Ability;
mod abilities;
mod stats;
use abilities::abilities_from_classpect;

use stats::{defense_from_classpect, health_from_classpect, speed_from_classpect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Classpect {
    class: Class,
    aspect: Aspect,
}

impl Classpect {
    pub fn new(class: Class, aspect: Aspect) -> Self {
        Self { class, aspect }
    }

    pub fn abilities(self) -> Vec<Rc<dyn Ability>> {
        abilities_from_classpect(self)
    }

    pub fn health(self) -> f64 {
        health_from_classpect(self)
    }

    pub fn defense(self) -> f64 {
        defense_from_classpect(self)
    }

    pub fn speed(self) -> f64 {
        speed_from_classpect(self)
    }
}

impl fmt::Display for Classpect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?} of {:#?}", self.class, self.aspect)
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
    type Item = Self;

    fn choices() -> Vec<Self::Item> {
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
    type Item = Self;

    fn choices() -> Vec<Self::Item> {
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
