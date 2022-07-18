mod classpect;
mod creature;
mod ui;

use std::rc::Rc;

use creature::Ability;
use creature::{Aspect, Class, Creature};
use ui::{choose, choose_complex};

fn main() {
    let mut player = Creature::new_with_classpect(
        choose::<Class>("What class do you want?"),
        choose::<Aspect>("What aspect do you want?"),
    );
    let mut test_creature = Creature::new_with_classpect(Class::Knight, Aspect::Blood);

    while player.health() > 0.0 && test_creature.health() > 0.0 {
        println!(">>———+ 🏠 +———<<");
        println!("You:");
        println!("{player}");
        println!("Enemy:");
        println!("{test_creature}");

        choose_complex::<Vec<Rc<dyn Ability>>>(
            "What ability do you want to use?",
            player.abilities(),
        )
        .activate(&mut player, &mut test_creature);

        player.activate_stati();
        test_creature.activate_stati();
    }
}
