mod classpect;
pub use classpect::{Aspect, Class};

#[derive(Debug)]
pub struct Creature {
    pub class: Class,
    pub aspect: Aspect,
}

impl Creature {
    pub fn new_with_classpect(class: Class, aspect: Aspect) -> Self {
        Self { class, aspect }
    }
}
