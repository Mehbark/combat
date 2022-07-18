use super::{Aspect, Class, Classpect};

pub fn health_from_classpect(Classpect { class, aspect }: Classpect) -> f64 {
    let mut health = 100.0;

    // Yes these are random lol
    health *= match class {
        Class::Thief => 1.0220762312625097,
        Class::Rogue => 0.946123526495138,
        Class::Bard => 1.0338690695971386,
        Class::Prince => 0.8782291938964286,
        Class::Heir => 0.9905788923937958,
        Class::Page => 1.1266210515620192,
        Class::Seer => 0.9701537709953697,
        Class::Maid => 1.1733080213487213,
        Class::Sylph => 1.2299078109041115,
        Class::Knight => 1.4999200873047073,
        Class::Witch => 0.8486991424564972,
        Class::Mage => 0.9986352465715058,
    };

    health *= match aspect {
        Aspect::Space => 0.5,
        Aspect::Time => 0.6,
        Aspect::Mind => 0.9,
        Aspect::Heart => 1.7,
        Aspect::Hope => 1.7,
        Aspect::Rage => 1.3,
        Aspect::Breath => 1.2,
        Aspect::Blood => 1.0,
        Aspect::Life => 2.0,
        Aspect::Doom => 0.7,
        Aspect::Light => 0.9,
        Aspect::Void => 0.5,
    };

    health
}

pub fn defense_from_classpect(Classpect { class, aspect }: Classpect) -> f64 {
    let mut defense = 5.0;

    defense *= match class {
        Class::Thief => 0.6,
        Class::Rogue => 0.6,
        Class::Bard => 0.5,
        Class::Prince => 0.6,
        Class::Heir => 0.5,
        Class::Page => 0.5,
        Class::Seer => 0.6,
        Class::Maid => 0.5,
        Class::Sylph => 0.5,
        Class::Knight => 1.5,
        Class::Witch => 1.3,
        Class::Mage => 1.3,
    };

    defense *= match aspect {
        Aspect::Space => 3.0,
        Aspect::Time => 3.0,
        Aspect::Mind => 2.0,
        Aspect::Heart => 1.0,
        Aspect::Hope => 1.5,
        Aspect::Rage => 0.8,
        Aspect::Breath => 1.3,
        Aspect::Blood => 0.9,
        Aspect::Life => 1.1,
        Aspect::Doom => 0.9,
        Aspect::Light => 0.7,
        Aspect::Void => 3.0,
    };

    defense
}

pub fn speed_from_classpect(Classpect { class, aspect }: Classpect) -> f64 {
    let mut speed = 10.0;

    speed *= match class {
        Class::Thief => 2.0,
        Class::Rogue => 1.8,
        Class::Bard => 1.7,
        Class::Prince => 1.0,
        Class::Heir => 1.2,
        Class::Page => 1.7,
        Class::Seer => 1.0,
        Class::Maid => 1.0,
        Class::Sylph => 1.0,
        Class::Knight => 0.6,
        Class::Witch => 1.1,
        Class::Mage => 1.1,
    };

    speed *= match aspect {
        Aspect::Space => 3.0,
        Aspect::Time => 3.0,
        Aspect::Mind => 1.2,
        Aspect::Heart => 1.0,
        Aspect::Hope => 0.7,
        Aspect::Rage => 1.3,
        Aspect::Breath => 3.0,
        Aspect::Blood => 1.0,
        Aspect::Life => 0.9,
        Aspect::Doom => 0.9,
        Aspect::Light => 0.9,
        Aspect::Void => 0.9,
    };

    speed
}
