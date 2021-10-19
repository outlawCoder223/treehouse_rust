use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("rance", "Hello supreme leader"),
        Visitor::new("maddy", "Hi sexy woman"),
        Visitor::new("mason", "Howdy Partner"),
        Visitor::new("marcus", "So cute baby"),
    ];

    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    println!("You said your name is, {}?", &your_name);
    let known_visitor = visitor_list.iter().find(|v| v.name == your_name);
    match known_visitor {
        Some(v) => v.greet_visitor(),
        None => println!("Get out of here fart sniffer"),
    }
}
