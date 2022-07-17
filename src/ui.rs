use std::io;

pub trait Choosable: Copy + Describable {
    fn choices() -> Vec<Self>;
}

pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Chooser<C: Choosable> {
    choices: Vec<C>,
    stdin: io::Stdin,
}

impl<C: Choosable> Chooser<C> {
    fn new() -> Self {
        Self {
            choices: C::choices(),
            stdin: io::stdin(),
        }
    }

    pub fn choose() -> C {
        let chooser = Self::new();

        let mut buffer = String::new();

        loop {
            chooser.print_choices();
            chooser
                .stdin
                .read_line(&mut buffer)
                .expect("failed to read line!");

            if let Some(choice) = chooser.parse_choice(&buffer) {
                println!("You chose: {}", choice.describe());
                break choice;
            }

            println!("Sorry, try again.");
        }
    }

    pub fn print_choices(&self) {
        for (i, choice) in self.choices.iter().map(Describable::describe).enumerate() {
            println!("{i}: {choice}")
        }
    }

    pub fn parse_choice(&self, input: &str) -> Option<C> {
        let number: usize = input.trim().parse().ok()?;
        self.choices.get(number).copied()
    }
}

pub fn choose<C: Choosable>() -> C {
    Chooser::<C>::choose()
}
