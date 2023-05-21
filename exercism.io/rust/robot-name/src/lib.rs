use std::{cell::RefCell, collections::HashSet};

use rand::Rng;

thread_local!(
    static NAMES: std::cell::RefCell<HashSet<String>> = RefCell::new(HashSet::new())
);

pub struct Robot {
    name: String,
}

impl Robot {
    fn generate() -> String {
        fn get() -> String {
            let mut rng = rand::thread_rng();
            let name: String = (0..2)
                .map(|_| rng.gen_range(b'A'..b'Z'))
                .map(|i| i as char)
                .collect();

            format!("{}{:03}", name, rng.gen_range(0..1000))
        }

        let name: String = loop {
            let name = get();

            if NAMES.with(|c| !c.borrow_mut().contains(&name)) {
                break name;
            }
        };

        NAMES.with(|c| c.borrow_mut().insert(name.clone()));
        name
    }

    pub fn new() -> Self {
        Robot {
            name: Robot::generate(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        NAMES.with(|c| c.borrow_mut().remove(&self.name));
        self.name = Robot::generate();
    }
}
