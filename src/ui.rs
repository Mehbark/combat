use std::io;

pub trait Choosable {
    type Item: Describable + Clone;
    fn choices() -> Vec<Self::Item> {
        Vec::new()
    }
}

pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Chooser<C: Choosable> {
    choices: Vec<C::Item>,
    stdin: io::Stdin,
}

impl<C: Choosable> Chooser<C> {
    fn new() -> Self {
        Self {
            choices: C::choices(),
            stdin: io::stdin(),
        }
    }

    fn new_raw(choices: &[C::Item]) -> Self {
        Self {
            choices: choices.to_vec(),
            stdin: io::stdin(),
        }
    }

    pub fn choose() -> C::Item {
        Self::new().choose_raw()
    }

    pub fn choose_complex(choices: &[C::Item]) -> C::Item {
        Self::new_raw(choices).choose_raw()
    }

    fn choose_raw(&self) -> C::Item {
        let mut buffer = String::new();

        loop {
            self.print_choices();
            self.stdin
                .read_line(&mut buffer)
                .expect("failed to read line!");

            if let Some(choice) = self.parse_choice(&buffer) {
                println!("You chose: {}", choice.describe());
                break choice;
            }

            println!("Sorry, try again.");
            buffer.clear();
        }
    }

    fn print_choices(&self) {
        for (i, choice) in self.choices.iter().map(Describable::describe).enumerate() {
            println!("{i}: {choice}")
        }
    }

    fn parse_choice(&self, input: &str) -> Option<C::Item> {
        let number: usize = input.trim().parse().ok()?;
        self.choices.get(number).cloned()
    }
}

pub fn choose<C: Choosable>(message: &str) -> C::Item {
    println!("{message}");
    Chooser::<C>::choose()
}

pub fn choose_complex<C: Choosable>(message: &str, choices: &[C::Item]) -> C::Item {
    println!("{message}");
    Chooser::<C>::choose_complex(choices)
}

impl Choosable for bool {
    type Item = Self;

    fn choices() -> Vec<Self::Item> {
        vec![false, true]
    }
}

impl Describable for bool {
    fn describe(&self) -> String {
        if *self { "Yes" } else { "No" }.to_string()
    }
}
