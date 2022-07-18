mod classpect;
mod creature;
mod ui;

use creature::Creature;
use ui::choose;

fn main() {
    let player = Creature::new_with_classpect(choose(), choose());
    println!("{player:#?}");
}
