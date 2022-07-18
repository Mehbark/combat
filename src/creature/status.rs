use std::rc::Rc;

use super::CreatureSkeleton;

pub struct Status {
    timer: usize,
    activate: Rc<dyn Fn(&mut CreatureSkeleton)>,
    exit: Rc<dyn Fn(&mut CreatureSkeleton)>,
}

impl Status {
    pub fn new(
        timer: usize,
        activate: Rc<dyn Fn(&mut CreatureSkeleton)>,
        exit: Rc<dyn Fn(&mut CreatureSkeleton)>,
    ) -> Self {
        Self {
            timer,
            activate,
            exit,
        }
    }

    pub fn new_exitless(timer: usize, activate: Rc<dyn Fn(&mut CreatureSkeleton)>) -> Self {
        Self {
            timer,
            activate,
            exit: Rc::new(|_| ()),
        }
    }

    pub fn activate(&mut self, creature: &mut CreatureSkeleton) {
        (self.activate)(creature);

        self.timer = self.timer.saturating_sub(1);
    }

    pub fn is_alive(&self) -> bool {
        self.timer > 0
    }
}
