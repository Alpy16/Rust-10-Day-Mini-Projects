use std::cell::{Cell, RefCell};

struct Config {
    api_key: String,
    count: Cell<u32>,
    // Renamed to 'access_log' for clarity, but 'address' works too!
    access_log: RefCell<Vec<String>>,
}

impl Config {
    fn new(key: &str) -> Self {
        Self {
            api_key: key.to_string(),
            count: Cell::new(0),
            access_log: RefCell::new(Vec::new()),
        }
    }

    fn get_key(&self, user: &str) -> &str {
        // Increment the Cell (only works on types that have Copy implemented)
        // pretty straightforward, we get the current count and add 1 to it, assign it as the new count
        self.count.set(self.count.get() + 1);

        // Mutate the RefCell (Complex type)
        // We create a message and push it into the borrowed vector
        let entry = format!("User '{}' accessed the key.", user);
        self.access_log.borrow_mut().push(entry);

        &self.api_key
    }
}

fn main() {
    let config = Config::new("MY_API_KEY");
    config.get_key("Alpi");
    config.get_key("Bob");
    config.get_key("Jack");

    println!("Code was accesed '{}' times", config.count.get());
    let names = config.access_log.borrow();
    for name in names.iter() {
        println!("- {}", name);
    }
}
